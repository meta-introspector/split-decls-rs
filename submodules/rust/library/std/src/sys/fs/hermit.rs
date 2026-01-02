mkuse!{use crate :: ffi :: { CStr , OsStr , OsString , c_char } ;}
mkuse!{use crate :: fs :: TryLockError ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , Error , ErrorKind , IoSlice , IoSliceMut , SeekFrom } ;}
mkuse!{use crate :: os :: hermit :: ffi :: OsStringExt ;}
mkuse!{use crate :: os :: hermit :: hermit_abi :: { self , DT_DIR , DT_LNK , DT_REG , DT_UNKNOWN , O_APPEND , O_CREAT , O_DIRECTORY , O_EXCL , O_RDONLY , O_RDWR , O_TRUNC , O_WRONLY , S_IFDIR , S_IFLNK , S_IFMT , S_IFREG , dirent64 , stat as stat_struct , } ;}
mkuse!{use crate :: os :: hermit :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , RawFd } ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sync :: Arc ;}
mkuse!{use crate :: sys :: common :: small_c_string :: run_path_with_cstr ;}
mkuse!{use crate :: sys :: fd :: FileDesc ;}
mkuse!{pub use crate :: sys :: fs :: common :: { copy , exists } ;}
mkuse!{use crate :: sys :: time :: SystemTime ;}
mkuse!{use crate :: sys :: { cvt , unsupported , unsupported_err } ;}
mkuse!{use crate :: sys_common :: { AsInner , AsInnerMut , FromInner , IntoInner } ;}
mkuse!{use crate :: { fmt , mem } ;}
mkitem!{mkstruct!{# [derive (Debug)] pub struct File (FileDesc) ;}}
mkitem!{mkstruct!{# [derive (Clone)] pub struct FileAttr { stat_val : stat_struct , }}}
mkitem!{mkimpl!{impl FileAttr { fn from_stat (stat_val : stat_struct) -> Self { Self { stat_val } } }}}
mkitem!{mkstruct!{struct InnerReadDir { root : PathBuf , dir : Vec < u8 > , }}}
mkitem!{mkimpl!{impl InnerReadDir { pub fn new (root : PathBuf , dir : Vec < u8 >) -> Self { Self { root , dir } } }}}
mkitem!{mkstruct!{pub struct ReadDir { inner : Arc < InnerReadDir > , pos : usize , }}}
mkitem!{mkimpl!{impl ReadDir { fn new (inner : InnerReadDir) -> Self { Self { inner : Arc :: new (inner) , pos : 0 } } }}}
mkitem!{mkstruct!{pub struct DirEntry { # [doc = " path to the entry"] root : PathBuf , # [doc = " 64-bit inode number"] ino : u64 , # [doc = " File type"] type_ : u8 , # [doc = " name of the entry"] name : OsString , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug)] pub struct OpenOptions { read : bool , write : bool , append : bool , truncate : bool , create : bool , create_new : bool , mode : i32 , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , Default)] pub struct FileTimes { }}}
mkitem!{mkstruct!{# [derive (Clone , PartialEq , Eq , Debug)] pub struct FilePermissions { mode : u32 , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Eq , Debug)] pub struct FileType { mode : u8 , }}}
mkitem!{mkimpl!{impl PartialEq for FileType { fn eq (& self , other : & Self) -> bool { self . mode == other . mode } }}}
mkitem!{mkimpl!{impl core :: hash :: Hash for FileType { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . mode . hash (state) ; } }}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct DirBuilder { mode : u32 , }}}
mkitem!{mkimpl!{impl FileAttr { pub fn modified (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: new (self . stat_val . st_mtim . tv_sec , self . stat_val . st_mtim . tv_nsec)) } pub fn accessed (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: new (self . stat_val . st_atim . tv_sec , self . stat_val . st_atim . tv_nsec)) } pub fn created (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: new (self . stat_val . st_ctim . tv_sec , self . stat_val . st_ctim . tv_nsec)) } pub fn size (& self) -> u64 { self . stat_val . st_size as u64 } pub fn perm (& self) -> FilePermissions { FilePermissions { mode : self . stat_val . st_mode } } pub fn file_type (& self) -> FileType { let masked_mode = self . stat_val . st_mode & S_IFMT ; let mode = match masked_mode { S_IFDIR => DT_DIR , S_IFLNK => DT_LNK , S_IFREG => DT_REG , _ => DT_UNKNOWN , } ; FileType { mode } } }}}
mkitem!{mkimpl!{impl FilePermissions { pub fn readonly (& self) -> bool { self . mode & 0o222 == 0 } pub fn set_readonly (& mut self , _readonly : bool) { unimplemented ! () } # [allow (dead_code)] pub fn mode (& self) -> u32 { self . mode as u32 } }}}
mkitem!{mkimpl!{impl FileTimes { pub fn set_accessed (& mut self , _t : SystemTime) { } pub fn set_modified (& mut self , _t : SystemTime) { } }}}
mkitem!{mkimpl!{impl FileType { pub fn is_dir (& self) -> bool { self . mode == DT_DIR } pub fn is_file (& self) -> bool { self . mode == DT_REG } pub fn is_symlink (& self) -> bool { self . mode == DT_LNK } }}}
mkitem!{mkimpl!{impl fmt :: Debug for ReadDir { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * self . inner . root , f) } }}}
mkitem!{mkimpl!{impl Iterator for ReadDir { type Item = io :: Result < DirEntry > ; fn next (& mut self) -> Option < io :: Result < DirEntry > > { let mut counter : usize = 0 ; let mut offset : usize = 0 ; loop { if offset >= self . inner . dir . len () { return None ; } let dir = unsafe { & * (self . inner . dir . as_ptr () . add (offset) as * const dirent64) } ; if counter == self . pos { self . pos += 1 ; let name_bytes = unsafe { CStr :: from_ptr (& dir . d_name as * const _ as * const c_char) . to_bytes () } ; let entry = DirEntry { root : self . inner . root . clone () , ino : dir . d_ino , type_ : dir . d_type , name : OsString :: from_vec (name_bytes . to_vec ()) , } ; return Some (Ok (entry)) ; } counter += 1 ; offset = offset + usize :: from (dir . d_reclen) ; } } }}}
mkitem!{mkimpl!{impl DirEntry { pub fn path (& self) -> PathBuf { self . root . join (self . file_name_os_str ()) } pub fn file_name (& self) -> OsString { self . file_name_os_str () . to_os_string () } pub fn metadata (& self) -> io :: Result < FileAttr > { let mut path = self . path () ; path . set_file_name (self . file_name_os_str ()) ; lstat (& path) } pub fn file_type (& self) -> io :: Result < FileType > { Ok (FileType { mode : self . type_ }) } # [allow (dead_code)] pub fn ino (& self) -> u64 { self . ino } pub fn file_name_os_str (& self) -> & OsStr { self . name . as_os_str () } }}}
mkitem!{mkimpl!{impl OpenOptions { pub fn new () -> OpenOptions { OpenOptions { read : false , write : false , append : false , truncate : false , create : false , create_new : false , mode : 0o777 , } } pub fn read (& mut self , read : bool) { self . read = read ; } pub fn write (& mut self , write : bool) { self . write = write ; } pub fn append (& mut self , append : bool) { self . append = append ; } pub fn truncate (& mut self , truncate : bool) { self . truncate = truncate ; } pub fn create (& mut self , create : bool) { self . create = create ; } pub fn create_new (& mut self , create_new : bool) { self . create_new = create_new ; } fn get_access_mode (& self) -> io :: Result < i32 > { match (self . read , self . write , self . append) { (true , false , false) => Ok (O_RDONLY) , (false , true , false) => Ok (O_WRONLY) , (true , true , false) => Ok (O_RDWR) , (false , _ , true) => Ok (O_WRONLY | O_APPEND) , (true , _ , true) => Ok (O_RDWR | O_APPEND) , (false , false , false) => { Err (io :: const_error ! (ErrorKind :: InvalidInput , "invalid access mode")) } } } fn get_creation_mode (& self) -> io :: Result < i32 > { match (self . write , self . append) { (true , false) => { } (false , false) => { if self . truncate || self . create || self . create_new { return Err (io :: const_error ! (ErrorKind :: InvalidInput , "invalid creation mode")) ; } } (_ , true) => { if self . truncate && ! self . create_new { return Err (io :: const_error ! (ErrorKind :: InvalidInput , "invalid creation mode")) ; } } } Ok (match (self . create , self . truncate , self . create_new) { (false , false , false) => 0 , (true , false , false) => O_CREAT , (false , true , false) => O_TRUNC , (true , true , false) => O_CREAT | O_TRUNC , (_ , _ , true) => O_CREAT | O_EXCL , }) } }}}
mkitem!{mkimpl!{impl File { pub fn open (path : & Path , opts : & OpenOptions) -> io :: Result < File > { run_path_with_cstr (path , & | path | File :: open_c (& path , opts)) } pub fn open_c (path : & CStr , opts : & OpenOptions) -> io :: Result < File > { let mut flags = opts . get_access_mode () ? ; flags = flags | opts . get_creation_mode () ? ; let mode ; if flags & O_CREAT == O_CREAT { mode = opts . mode ; } else { mode = 0 ; } let fd = unsafe { cvt (hermit_abi :: open (path . as_ptr () , flags , mode)) ? } ; Ok (File (unsafe { FileDesc :: from_raw_fd (fd as i32) })) } pub fn file_attr (& self) -> io :: Result < FileAttr > { let mut stat_val : stat_struct = unsafe { mem :: zeroed () } ; self . 0 . fstat (& mut stat_val) ? ; Ok (FileAttr :: from_stat (stat_val)) } pub fn fsync (& self) -> io :: Result < () > { Err (Error :: from_raw_os_error (22)) } pub fn datasync (& self) -> io :: Result < () > { self . fsync () } pub fn lock (& self) -> io :: Result < () > { unsupported () } pub fn lock_shared (& self) -> io :: Result < () > { unsupported () } pub fn try_lock (& self) -> Result < () , TryLockError > { Err (TryLockError :: Error (unsupported_err ())) } pub fn try_lock_shared (& self) -> Result < () , TryLockError > { Err (TryLockError :: Error (unsupported_err ())) } pub fn unlock (& self) -> io :: Result < () > { unsupported () } pub fn truncate (& self , _size : u64) -> io :: Result < () > { Err (Error :: from_raw_os_error (22)) } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } # [inline] pub fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } pub fn read_buf (& self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 . read_buf (cursor) } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . 0 . write_vectored (bufs) } # [inline] pub fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } # [inline] pub fn flush (& self) -> io :: Result < () > { Ok (()) } pub fn seek (& self , pos : SeekFrom) -> io :: Result < u64 > { self . 0 . seek (pos) } pub fn size (& self) -> Option < io :: Result < u64 > > { None } pub fn tell (& self) -> io :: Result < u64 > { self . 0 . tell () } pub fn duplicate (& self) -> io :: Result < File > { Err (Error :: from_raw_os_error (22)) } pub fn set_permissions (& self , _perm : FilePermissions) -> io :: Result < () > { Err (Error :: from_raw_os_error (22)) } pub fn set_times (& self , _times : FileTimes) -> io :: Result < () > { Err (Error :: from_raw_os_error (22)) } }}}
mkitem!{mkimpl!{impl DirBuilder { pub fn new () -> DirBuilder { DirBuilder { mode : 0o777 } } pub fn mkdir (& self , path : & Path) -> io :: Result < () > { run_path_with_cstr (path , & | path | { cvt (unsafe { hermit_abi :: mkdir (path . as_ptr () . cast () , self . mode . into ()) }) . map (| _ | ()) }) } # [allow (dead_code)] pub fn set_mode (& mut self , mode : u32) { self . mode = mode ; } }}}
mkitem!{mkimpl!{impl AsInner < FileDesc > for File { # [inline] fn as_inner (& self) -> & FileDesc { & self . 0 } }}}
mkitem!{mkimpl!{impl AsInnerMut < FileDesc > for File { # [inline] fn as_inner_mut (& mut self) -> & mut FileDesc { & mut self . 0 } }}}
mkitem!{mkimpl!{impl IntoInner < FileDesc > for File { fn into_inner (self) -> FileDesc { self . 0 } }}}
mkitem!{mkimpl!{impl FromInner < FileDesc > for File { fn from_inner (file_desc : FileDesc) -> Self { Self (file_desc) } }}}
mkitem!{mkimpl!{impl AsFd for File { fn as_fd (& self) -> BorrowedFd < '_ > { self . 0 . as_fd () } }}}
mkitem!{mkimpl!{impl AsRawFd for File { # [inline] fn as_raw_fd (& self) -> RawFd { self . 0 . as_raw_fd () } }}}
mkitem!{mkimpl!{impl IntoRawFd for File { fn into_raw_fd (self) -> RawFd { self . 0 . into_raw_fd () } }}}
mkitem!{mkimpl!{impl FromRawFd for File { unsafe fn from_raw_fd (raw_fd : RawFd) -> Self { let file_desc = unsafe { FileDesc :: from_raw_fd (raw_fd) } ; Self (file_desc) } }}}

macro_rules! readdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readdir in module {}", module_path!());
    };
}

mkfn!{
    readdir_introspect!();
    pub fn readdir (path : & Path) -> io :: Result < ReadDir > { let fd_raw = run_path_with_cstr (path , & | path | { cvt (unsafe { hermit_abi :: open (path . as_ptr () , O_RDONLY | O_DIRECTORY , 0) }) }) ? ; let fd = unsafe { FileDesc :: from_raw_fd (fd_raw as i32) } ; let root = path . to_path_buf () ; let mut vec : Vec < u8 > = Vec :: new () ; let mut sz = 512 ; loop { vec . resize (sz , 0) ; let readlen = unsafe { hermit_abi :: getdents64 (fd . as_raw_fd () , vec . as_mut_ptr () as * mut dirent64 , sz) } ; if readlen > 0 { vec . resize (readlen . try_into () . unwrap () , 0) ; break ; } if readlen != (- hermit_abi :: errno :: EINVAL) . into () { return Err (Error :: from_raw_os_error (readlen . try_into () . unwrap ())) ; } sz = sz * 2 ; if sz > 0x100000 { return Err (Error :: from (ErrorKind :: Uncategorized)) ; } } Ok (ReadDir :: new (InnerReadDir :: new (root , vec))) }
}

macro_rules! unlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unlink in module {}", module_path!());
    };
}

mkfn!{
    unlink_introspect!();
    pub fn unlink (path : & Path) -> io :: Result < () > { run_path_with_cstr (path , & | path | cvt (unsafe { hermit_abi :: unlink (path . as_ptr ()) }) . map (| _ | ())) }
}

macro_rules! rename_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rename in module {}", module_path!());
    };
}

mkfn!{
    rename_introspect!();
    pub fn rename (_old : & Path , _new : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! set_perm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_perm in module {}", module_path!());
    };
}

mkfn!{
    set_perm_introspect!();
    pub fn set_perm (_p : & Path , _perm : FilePermissions) -> io :: Result < () > { Err (Error :: from_raw_os_error (22)) }
}

macro_rules! rmdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rmdir in module {}", module_path!());
    };
}

mkfn!{
    rmdir_introspect!();
    pub fn rmdir (path : & Path) -> io :: Result < () > { run_path_with_cstr (path , & | path | cvt (unsafe { hermit_abi :: rmdir (path . as_ptr ()) }) . map (| _ | ())) }
}

macro_rules! remove_dir_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_introspect!();
    pub fn remove_dir_all (_path : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! readlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readlink in module {}", module_path!());
    };
}

mkfn!{
    readlink_introspect!();
    pub fn readlink (_p : & Path) -> io :: Result < PathBuf > { unsupported () }
}

macro_rules! symlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symlink in module {}", module_path!());
    };
}

mkfn!{
    symlink_introspect!();
    pub fn symlink (_original : & Path , _link : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link in module {}", module_path!());
    };
}

mkfn!{
    link_introspect!();
    pub fn link (_original : & Path , _link : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! stat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stat in module {}", module_path!());
    };
}

mkfn!{
    stat_introspect!();
    pub fn stat (path : & Path) -> io :: Result < FileAttr > { run_path_with_cstr (path , & | path | { let mut stat_val : stat_struct = unsafe { mem :: zeroed () } ; cvt (unsafe { hermit_abi :: stat (path . as_ptr () , & mut stat_val) }) ? ; Ok (FileAttr :: from_stat (stat_val)) }) }
}

macro_rules! lstat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lstat in module {}", module_path!());
    };
}

mkfn!{
    lstat_introspect!();
    pub fn lstat (path : & Path) -> io :: Result < FileAttr > { run_path_with_cstr (path , & | path | { let mut stat_val : stat_struct = unsafe { mem :: zeroed () } ; cvt (unsafe { hermit_abi :: lstat (path . as_ptr () , & mut stat_val) }) ? ; Ok (FileAttr :: from_stat (stat_val)) }) }
}

macro_rules! canonicalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function canonicalize in module {}", module_path!());
    };
}

mkfn!{
    canonicalize_introspect!();
    pub fn canonicalize (_p : & Path) -> io :: Result < PathBuf > { unsupported () }
}