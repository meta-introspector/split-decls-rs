mkuse!{use crate :: ffi :: { CStr , CString , OsStr , OsString } ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: fs :: TryLockError ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut , SeekFrom } ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: os :: raw :: { c_int , c_short } ;}
mkuse!{use crate :: os :: solid :: ffi :: OsStrExt ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sync :: Arc ;}
mkuse!{pub use crate :: sys :: fs :: common :: exists ;}
mkuse!{use crate :: sys :: pal :: { abi , error } ;}
mkuse!{use crate :: sys :: time :: SystemTime ;}
mkuse!{use crate :: sys :: { unsupported , unsupported_err } ;}
mkuse!{use crate :: sys_common :: ignore_notfound ;}
mkitem!{type CIntNotMinusOne = core :: num :: niche_types :: NotAllOnes < c_int > ;}
mkitem!{mkstruct!{# [doc = " A file descriptor."] # [derive (Clone , Copy)] struct FileDesc { fd : CIntNotMinusOne , }}}
mkitem!{mkimpl!{impl FileDesc { # [inline] # [track_caller] fn new (fd : c_int) -> FileDesc { FileDesc { fd : CIntNotMinusOne :: new (fd) . expect ("fd != -1") } } # [inline] fn raw (& self) -> c_int { self . fd . as_inner () } }}}
mkitem!{mkstruct!{pub struct File { fd : FileDesc , }}}
mkitem!{mkstruct!{# [derive (Clone)] pub struct FileAttr { stat : abi :: stat , }}}
mkitem!{mkstruct!{struct InnerReadDir { dirp : abi :: S_DIR , root : PathBuf , }}}
mkitem!{mkstruct!{pub struct ReadDir { inner : Arc < InnerReadDir > , }}}
mkitem!{mkstruct!{pub struct DirEntry { entry : abi :: dirent , inner : Arc < InnerReadDir > , }}}
mkitem!{mkstruct!{# [derive (Clone , Debug)] pub struct OpenOptions { read : bool , write : bool , append : bool , truncate : bool , create : bool , create_new : bool , custom_flags : i32 , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , Default)] pub struct FileTimes { }}}
mkitem!{mkstruct!{# [derive (Clone , PartialEq , Eq , Debug)] pub struct FilePermissions (c_short) ;}}
mkitem!{mkstruct!{# [derive (Copy , Clone , PartialEq , Eq , Hash , Debug)] pub struct FileType (c_short) ;}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct DirBuilder { }}}
mkitem!{mkimpl!{impl FileAttr { pub fn size (& self) -> u64 { self . stat . st_size as u64 } pub fn perm (& self) -> FilePermissions { FilePermissions (self . stat . st_mode) } pub fn file_type (& self) -> FileType { FileType (self . stat . st_mode) } pub fn modified (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: from_time_t (self . stat . st_mtime)) } pub fn accessed (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: from_time_t (self . stat . st_atime)) } pub fn created (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: from_time_t (self . stat . st_ctime)) } }}}
mkitem!{mkimpl!{impl FilePermissions { pub fn readonly (& self) -> bool { (self . 0 & abi :: S_IWRITE) == 0 } pub fn set_readonly (& mut self , readonly : bool) { if readonly { self . 0 &= ! abi :: S_IWRITE ; } else { self . 0 |= abi :: S_IWRITE ; } } }}}
mkitem!{mkimpl!{impl FileTimes { pub fn set_accessed (& mut self , _t : SystemTime) { } pub fn set_modified (& mut self , _t : SystemTime) { } }}}
mkitem!{mkimpl!{impl FileType { pub fn is_dir (& self) -> bool { self . is (abi :: S_IFDIR) } pub fn is_file (& self) -> bool { self . is (abi :: S_IFREG) } pub fn is_symlink (& self) -> bool { false } pub fn is (& self , mode : c_short) -> bool { self . 0 & abi :: S_IFMT == mode } }}}

macro_rules! readdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readdir in module {}", module_path!());
    };
}

mkfn!{
    readdir_introspect!();
    pub fn readdir (p : & Path) -> io :: Result < ReadDir > { unsafe { let mut dir = MaybeUninit :: uninit () ; error :: SolidError :: err_if_negative (abi :: SOLID_FS_OpenDir (cstr (p) ? . as_ptr () , dir . as_mut_ptr () ,)) . map_err (| e | e . as_io_error ()) ? ; let inner = Arc :: new (InnerReadDir { dirp : dir . assume_init () , root : p . to_owned () }) ; Ok (ReadDir { inner }) } }
}
mkitem!{mkimpl!{impl fmt :: Debug for ReadDir { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * self . inner . root , f) } }}}
mkitem!{mkimpl!{impl Iterator for ReadDir { type Item = io :: Result < DirEntry > ; fn next (& mut self) -> Option < io :: Result < DirEntry > > { let entry = unsafe { let mut out_entry = MaybeUninit :: uninit () ; match error :: SolidError :: err_if_negative (abi :: SOLID_FS_ReadDir (self . inner . dirp , out_entry . as_mut_ptr () ,)) { Ok (_) => out_entry . assume_init () , Err (e) if e . as_raw () == abi :: SOLID_ERR_NOTFOUND => return None , Err (e) => return Some (Err (e . as_io_error ())) , } } ; (entry . d_name [0] != 0) . then (| | Ok (DirEntry { entry , inner : Arc :: clone (& self . inner) })) } }}}
mkitem!{mkimpl!{impl Drop for InnerReadDir { fn drop (& mut self) { unsafe { abi :: SOLID_FS_CloseDir (self . dirp) } ; } }}}
mkitem!{mkimpl!{impl DirEntry { pub fn path (& self) -> PathBuf { self . inner . root . join (OsStr :: from_bytes (unsafe { CStr :: from_ptr (self . entry . d_name . as_ptr ()) } . to_bytes () ,)) } pub fn file_name (& self) -> OsString { OsStr :: from_bytes (unsafe { CStr :: from_ptr (self . entry . d_name . as_ptr ()) } . to_bytes ()) . to_os_string () } pub fn metadata (& self) -> io :: Result < FileAttr > { lstat (& self . path ()) } pub fn file_type (& self) -> io :: Result < FileType > { match self . entry . d_type { abi :: DT_CHR => Ok (FileType (abi :: S_IFCHR)) , abi :: DT_FIFO => Ok (FileType (abi :: S_IFIFO)) , abi :: DT_REG => Ok (FileType (abi :: S_IFREG)) , abi :: DT_DIR => Ok (FileType (abi :: S_IFDIR)) , abi :: DT_BLK => Ok (FileType (abi :: S_IFBLK)) , _ => lstat (& self . path ()) . map (| m | m . file_type ()) , } } }}}
mkitem!{mkimpl!{impl OpenOptions { pub fn new () -> OpenOptions { OpenOptions { read : false , write : false , append : false , truncate : false , create : false , create_new : false , custom_flags : 0 , } } pub fn read (& mut self , read : bool) { self . read = read ; } pub fn write (& mut self , write : bool) { self . write = write ; } pub fn append (& mut self , append : bool) { self . append = append ; } pub fn truncate (& mut self , truncate : bool) { self . truncate = truncate ; } pub fn create (& mut self , create : bool) { self . create = create ; } pub fn create_new (& mut self , create_new : bool) { self . create_new = create_new ; } pub fn custom_flags (& mut self , flags : i32) { self . custom_flags = flags ; } pub fn mode (& mut self , _mode : u32) { } fn get_access_mode (& self) -> io :: Result < c_int > { match (self . read , self . write , self . append) { (true , false , false) => Ok (abi :: O_RDONLY) , (false , true , false) => Ok (abi :: O_WRONLY) , (true , true , false) => Ok (abi :: O_RDWR) , (false , _ , true) => Ok (abi :: O_WRONLY | abi :: O_APPEND) , (true , _ , true) => Ok (abi :: O_RDWR | abi :: O_APPEND) , (false , false , false) => Err (io :: Error :: from_raw_os_error (libc :: EINVAL)) , } } fn get_creation_mode (& self) -> io :: Result < c_int > { match (self . write , self . append) { (true , false) => { } (false , false) => { if self . truncate || self . create || self . create_new { return Err (io :: Error :: from_raw_os_error (libc :: EINVAL)) ; } } (_ , true) => { if self . truncate && ! self . create_new { return Err (io :: Error :: from_raw_os_error (libc :: EINVAL)) ; } } } Ok (match (self . create , self . truncate , self . create_new) { (false , false , false) => 0 , (true , false , false) => abi :: O_CREAT , (false , true , false) => abi :: O_TRUNC , (true , true , false) => abi :: O_CREAT | abi :: O_TRUNC , (_ , _ , true) => abi :: O_CREAT | abi :: O_EXCL , }) } }}}

macro_rules! cstr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cstr in module {}", module_path!());
    };
}

mkfn!{
    cstr_introspect!();
    fn cstr (path : & Path) -> io :: Result < CString > { let path = path . as_os_str () . as_bytes () ; if ! path . starts_with (br"\") { return Err (crate :: io :: const_error ! (crate :: io :: ErrorKind :: Unsupported , "relative path is not supported on this platform" ,)) ; } const SAFE_PREFIX : & [u8] = br"\TS" ; let wrapped_path = [SAFE_PREFIX , & path , & [0]] . concat () ; CString :: from_vec_with_nul (wrapped_path) . map_err (| _ | { crate :: io :: const_error ! (io :: ErrorKind :: InvalidInput , "path provided contains a nul byte") }) }
}
mkitem!{mkimpl!{impl File { pub fn open (path : & Path , opts : & OpenOptions) -> io :: Result < File > { let flags = opts . get_access_mode () ? | opts . get_creation_mode () ? | (opts . custom_flags as c_int & ! abi :: O_ACCMODE) ; unsafe { let mut fd = MaybeUninit :: uninit () ; error :: SolidError :: err_if_negative (abi :: SOLID_FS_Open (fd . as_mut_ptr () , cstr (path) ? . as_ptr () , flags ,)) . map_err (| e | e . as_io_error ()) ? ; Ok (File { fd : FileDesc :: new (fd . assume_init ()) }) } } pub fn file_attr (& self) -> io :: Result < FileAttr > { unsupported () } pub fn fsync (& self) -> io :: Result < () > { self . flush () } pub fn datasync (& self) -> io :: Result < () > { self . flush () } pub fn lock (& self) -> io :: Result < () > { unsupported () } pub fn lock_shared (& self) -> io :: Result < () > { unsupported () } pub fn try_lock (& self) -> Result < () , TryLockError > { Err (TryLockError :: Error (unsupported_err ())) } pub fn try_lock_shared (& self) -> Result < () , TryLockError > { Err (TryLockError :: Error (unsupported_err ())) } pub fn unlock (& self) -> io :: Result < () > { unsupported () } pub fn truncate (& self , _size : u64) -> io :: Result < () > { unsupported () } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { unsafe { let mut out_num_bytes = MaybeUninit :: uninit () ; error :: SolidError :: err_if_negative (abi :: SOLID_FS_Read (self . fd . raw () , buf . as_mut_ptr () , buf . len () , out_num_bytes . as_mut_ptr () ,)) . map_err (| e | e . as_io_error ()) ? ; Ok (out_num_bytes . assume_init ()) } } pub fn read_buf (& self , mut cursor : BorrowedCursor < '_ >) -> io :: Result < () > { unsafe { let len = cursor . capacity () ; let mut out_num_bytes = MaybeUninit :: uninit () ; error :: SolidError :: err_if_negative (abi :: SOLID_FS_Read (self . fd . raw () , cursor . as_mut () . as_mut_ptr () as * mut u8 , len , out_num_bytes . as_mut_ptr () ,)) . map_err (| e | e . as_io_error ()) ? ; let num_bytes_read = out_num_bytes . assume_init () ; cursor . advance_unchecked (num_bytes_read) ; Ok (()) } } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { crate :: io :: default_read_vectored (| buf | self . read (buf) , bufs) } pub fn is_read_vectored (& self) -> bool { false } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { unsafe { let mut out_num_bytes = MaybeUninit :: uninit () ; error :: SolidError :: err_if_negative (abi :: SOLID_FS_Write (self . fd . raw () , buf . as_ptr () , buf . len () , out_num_bytes . as_mut_ptr () ,)) . map_err (| e | e . as_io_error ()) ? ; Ok (out_num_bytes . assume_init ()) } } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { crate :: io :: default_write_vectored (| buf | self . write (buf) , bufs) } pub fn is_write_vectored (& self) -> bool { false } pub fn flush (& self) -> io :: Result < () > { error :: SolidError :: err_if_negative (unsafe { abi :: SOLID_FS_Sync (self . fd . raw ()) }) . map_err (| e | e . as_io_error ()) ? ; Ok (()) } pub fn seek (& self , pos : SeekFrom) -> io :: Result < u64 > { let (whence , pos) = match pos { SeekFrom :: Start (off) => (abi :: SEEK_SET , off as i64) , SeekFrom :: End (off) => (abi :: SEEK_END , off) , SeekFrom :: Current (off) => (abi :: SEEK_CUR , off) , } ; error :: SolidError :: err_if_negative (unsafe { abi :: SOLID_FS_Lseek (self . fd . raw () , pos , whence) }) . map_err (| e | e . as_io_error ()) ? ; self . tell () } pub fn size (& self) -> Option < io :: Result < u64 > > { None } pub fn tell (& self) -> io :: Result < u64 > { unsafe { let mut out_offset = MaybeUninit :: uninit () ; error :: SolidError :: err_if_negative (abi :: SOLID_FS_Ftell (self . fd . raw () , out_offset . as_mut_ptr () ,)) . map_err (| e | e . as_io_error ()) ? ; Ok (out_offset . assume_init () as u64) } } pub fn duplicate (& self) -> io :: Result < File > { unsupported () } pub fn set_permissions (& self , _perm : FilePermissions) -> io :: Result < () > { unsupported () } pub fn set_times (& self , _times : FileTimes) -> io :: Result < () > { unsupported () } }}}
mkitem!{mkimpl!{impl Drop for File { fn drop (& mut self) { unsafe { abi :: SOLID_FS_Close (self . fd . raw ()) } ; } }}}
mkitem!{mkimpl!{impl DirBuilder { pub fn new () -> DirBuilder { DirBuilder { } } pub fn mkdir (& self , p : & Path) -> io :: Result < () > { error :: SolidError :: err_if_negative (unsafe { abi :: SOLID_FS_Mkdir (cstr (p) ? . as_ptr ()) }) . map_err (| e | e . as_io_error ()) ? ; Ok (()) } }}}
mkitem!{mkimpl!{impl fmt :: Debug for File { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("File") . field ("fd" , & self . fd . raw ()) . finish () } }}}

macro_rules! unlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unlink in module {}", module_path!());
    };
}

mkfn!{
    unlink_introspect!();
    pub fn unlink (p : & Path) -> io :: Result < () > { if stat (p) ? . file_type () . is_dir () { Err (io :: const_error ! (io :: ErrorKind :: IsADirectory , "is a directory")) } else { error :: SolidError :: err_if_negative (unsafe { abi :: SOLID_FS_Unlink (cstr (p) ? . as_ptr ()) }) . map_err (| e | e . as_io_error ()) ? ; Ok (()) } }
}

macro_rules! rename_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rename in module {}", module_path!());
    };
}

mkfn!{
    rename_introspect!();
    pub fn rename (old : & Path , new : & Path) -> io :: Result < () > { error :: SolidError :: err_if_negative (unsafe { abi :: SOLID_FS_Rename (cstr (old) ? . as_ptr () , cstr (new) ? . as_ptr ()) }) . map_err (| e | e . as_io_error ()) ? ; Ok (()) }
}

macro_rules! set_perm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_perm in module {}", module_path!());
    };
}

mkfn!{
    set_perm_introspect!();
    pub fn set_perm (p : & Path , perm : FilePermissions) -> io :: Result < () > { error :: SolidError :: err_if_negative (unsafe { abi :: SOLID_FS_Chmod (cstr (p) ? . as_ptr () , perm . 0 . into ()) }) . map_err (| e | e . as_io_error ()) ? ; Ok (()) }
}

macro_rules! rmdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rmdir in module {}", module_path!());
    };
}

mkfn!{
    rmdir_introspect!();
    pub fn rmdir (p : & Path) -> io :: Result < () > { if stat (p) ? . file_type () . is_dir () { error :: SolidError :: err_if_negative (unsafe { abi :: SOLID_FS_Unlink (cstr (p) ? . as_ptr ()) }) . map_err (| e | e . as_io_error ()) ? ; Ok (()) } else { Err (io :: const_error ! (io :: ErrorKind :: NotADirectory , "not a directory")) } }
}

macro_rules! remove_dir_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_introspect!();
    pub fn remove_dir_all (path : & Path) -> io :: Result < () > { for child in readdir (path) ? { let result : io :: Result < () > = try { let child = child ? ; let child_type = child . file_type () ? ; if child_type . is_dir () { remove_dir_all (& child . path ()) ? ; } else { unlink (& child . path ()) ? ; } } ; if let Err (err) = & result && err . kind () != io :: ErrorKind :: NotFound { return result ; } } ignore_notfound (rmdir (path)) }
}

macro_rules! readlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readlink in module {}", module_path!());
    };
}

mkfn!{
    readlink_introspect!();
    pub fn readlink (p : & Path) -> io :: Result < PathBuf > { stat (p) ? ; Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "not a symbolic link")) }
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
    pub fn link (_src : & Path , _dst : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! stat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stat in module {}", module_path!());
    };
}

mkfn!{
    stat_introspect!();
    pub fn stat (p : & Path) -> io :: Result < FileAttr > { lstat (p) }
}

macro_rules! lstat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lstat in module {}", module_path!());
    };
}

mkfn!{
    lstat_introspect!();
    pub fn lstat (p : & Path) -> io :: Result < FileAttr > { unsafe { let mut out_stat = MaybeUninit :: uninit () ; error :: SolidError :: err_if_negative (abi :: SOLID_FS_Stat (cstr (p) ? . as_ptr () , out_stat . as_mut_ptr () ,)) . map_err (| e | e . as_io_error ()) ? ; Ok (FileAttr { stat : out_stat . assume_init () }) } }
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

macro_rules! copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy in module {}", module_path!());
    };
}

mkfn!{
    copy_introspect!();
    pub fn copy (from : & Path , to : & Path) -> io :: Result < u64 > { use crate :: fs :: File ; let mut reader = File :: open (from) ? ; let mut writer = File :: create (to) ? ; io :: copy (& mut reader , & mut writer) }
}