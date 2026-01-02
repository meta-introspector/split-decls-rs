mkuse!{use core :: ptr ;}
mkuse!{use core :: sync :: atomic :: { Atomic , AtomicU32 , Ordering } ;}
mkuse!{use super :: { AsRawHandle , DirBuff , File , FromRawHandle } ;}
mkuse!{use crate :: sys :: c ;}
mkuse!{use crate :: sys :: pal :: api :: { UnicodeStrRef , WinError , unicode_str } ;}
mkuse!{use crate :: thread ;}
mkitem!{const MAX_RETRIES : usize = 50 ;}

macro_rules! nt_open_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nt_open_file in module {}", module_path!());
    };
}

mkfn!{
    nt_open_file_introspect!();
    # [doc = " A wrapper around a raw NtOpenFile call."] # [doc = ""] # [doc = " This isn't completely safe because `OBJECT_ATTRIBUTES` contains raw pointers."] unsafe fn nt_open_file (access : u32 , object_attribute : & c :: OBJECT_ATTRIBUTES , share : u32 , options : u32 ,) -> Result < File , WinError > { unsafe { let mut handle = ptr :: null_mut () ; let mut io_status = c :: IO_STATUS_BLOCK :: PENDING ; let status = c :: NtOpenFile (& mut handle , access , object_attribute , & mut io_status , share , options) ; if c :: nt_success (status) { Ok (File :: from_raw_handle (handle)) } else { let win_error = if status == c :: STATUS_DELETE_PENDING { WinError :: DELETE_PENDING } else { WinError :: new (c :: RtlNtStatusToDosError (status)) } ; Err (win_error) } } }
}

macro_rules! open_link_no_reparse_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function open_link_no_reparse in module {}", module_path!());
    };
}

mkfn!{
    open_link_no_reparse_introspect!();
    # [doc = " Open the file `path` in the directory `parent`, requesting the given `access` rights."] # [doc = " `options` will be OR'd with `FILE_OPEN_REPARSE_POINT`."] fn open_link_no_reparse (parent : & File , path : UnicodeStrRef < '_ > , access : u32 , options : u32 ,) -> Result < Option < File > , WinError > { static ATTRIBUTES : Atomic < u32 > = AtomicU32 :: new (c :: OBJ_DONT_REPARSE) ; let result = unsafe { let mut object = c :: OBJECT_ATTRIBUTES { ObjectName : path . as_ptr () , RootDirectory : parent . as_raw_handle () , Attributes : ATTRIBUTES . load (Ordering :: Relaxed) , .. c :: OBJECT_ATTRIBUTES :: with_length () } ; let share = c :: FILE_SHARE_DELETE | c :: FILE_SHARE_READ | c :: FILE_SHARE_WRITE ; let options = c :: FILE_OPEN_REPARSE_POINT | options ; let result = nt_open_file (access , & object , share , options) ; if matches ! (result , Err (WinError :: INVALID_PARAMETER)) && ATTRIBUTES . load (Ordering :: Relaxed) == c :: OBJ_DONT_REPARSE { ATTRIBUTES . store (0 , Ordering :: Relaxed) ; object . Attributes = 0 ; nt_open_file (access , & object , share , options) } else { result } } ; match result { Ok (f) => Ok (Some (f)) , Err (WinError :: FILE_NOT_FOUND | WinError :: PATH_NOT_FOUND | WinError :: BAD_NETPATH | WinError :: BAD_NET_NAME | WinError :: DELETE_PENDING ,) => Ok (None) , Err (e) => Err (e) , } }
}

macro_rules! open_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function open_dir in module {}", module_path!());
    };
}

mkfn!{
    open_dir_introspect!();
    fn open_dir (parent : & File , name : UnicodeStrRef < '_ >) -> Result < Option < File > , WinError > { open_link_no_reparse (parent , name , c :: SYNCHRONIZE | c :: FILE_LIST_DIRECTORY , c :: FILE_SYNCHRONOUS_IO_NONALERT ,) }
}

macro_rules! delete_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function delete in module {}", module_path!());
    };
}

mkfn!{
    delete_introspect!();
    fn delete (parent : & File , name : UnicodeStrRef < '_ >) -> Result < () , WinError > { match open_link_no_reparse (parent , name , c :: DELETE , 0) { Ok (Some (f)) => f . delete () , Ok (None) => Ok (()) , Err (e) => Err (e) , } }
}

macro_rules! retry_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function retry in module {}", module_path!());
    };
}

mkfn!{
    retry_introspect!();
    # [doc = " A simple retry loop that keeps running `f` while it fails with the given"] # [doc = " error code or until `MAX_RETRIES` is reached."] fn retry < T : PartialEq > (mut f : impl FnMut () -> Result < T , WinError > , ignore : WinError ,) -> Result < T , WinError > { let mut i = MAX_RETRIES ; loop { i -= 1 ; if i == 0 { return f () ; } else { let result = f () ; if result != Err (ignore) { return result ; } } thread :: yield_now () ; } }
}

macro_rules! remove_dir_all_iterative_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all_iterative in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_iterative_introspect!();
    pub fn remove_dir_all_iterative (dir : File) -> Result < () , WinError > { let mut buffer = DirBuff :: new () ; let mut dirlist = vec ! [dir] ; let mut restart = true ; 'outer : while let Some (dir) = dirlist . pop () { let more_data = dir . fill_dir_buff (& mut buffer , restart) ? ; for (name , is_directory) in buffer . iter () { let name = unicode_str ! (& name) ; if is_directory { let Some (subdir) = open_dir (& dir , name) ? else { continue } ; dirlist . push (dir) ; dirlist . push (subdir) ; continue 'outer ; } else { retry (| | delete (& dir , name) , WinError :: SHARING_VIOLATION) ? ; } } if more_data { dirlist . push (dir) ; restart = false ; } else { let name = unicode_str ! ("") ; retry (| | delete (& dir , name) , WinError :: DIR_NOT_EMPTY) ? ; restart = true ; } } Ok (()) }
}