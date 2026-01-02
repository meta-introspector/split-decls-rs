mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{# [cfg (all (target_os = "linux" , target_env = "gnu"))] use libc :: c_char ;}
mkuse!{# [cfg (any (all (target_os = "linux" , not (target_env = "musl")) , target_os = "android" , target_os = "fuchsia" , target_os = "hurd" , target_os = "illumos" ,))] use libc :: dirfd ;}
mkuse!{# [cfg (any (target_os = "fuchsia" , target_os = "illumos"))] use libc :: fstatat as fstatat64 ;}
mkuse!{# [cfg (any (all (target_os = "linux" , not (target_env = "musl")) , target_os = "hurd"))] use libc :: fstatat64 ;}
mkuse!{# [cfg (any (target_os = "android" , target_os = "solaris" , target_os = "fuchsia" , target_os = "redox" , target_os = "illumos" , target_os = "aix" , target_os = "nto" , target_os = "vita" , all (target_os = "linux" , target_env = "musl") ,))] use libc :: readdir as readdir64 ;}
mkuse!{# [cfg (not (any (target_os = "android" , target_os = "linux" , target_os = "solaris" , target_os = "illumos" , target_os = "l4re" , target_os = "fuchsia" , target_os = "redox" , target_os = "aix" , target_os = "nto" , target_os = "vita" , target_os = "hurd" ,)))] use libc :: readdir_r as readdir64_r ;}
mkuse!{# [cfg (any (all (target_os = "linux" , not (target_env = "musl")) , target_os = "hurd"))] use libc :: readdir64 ;}
mkuse!{# [cfg (target_os = "l4re")] use libc :: readdir64_r ;}
mkuse!{use libc :: { c_int , mode_t } ;}
mkuse!{# [cfg (target_os = "android")] use libc :: { dirent as dirent64 , fstat as fstat64 , fstatat as fstatat64 , ftruncate64 , lseek64 , lstat as lstat64 , off64_t , open as open64 , stat as stat64 , } ;}
mkuse!{# [cfg (not (any (all (target_os = "linux" , not (target_env = "musl")) , target_os = "l4re" , target_os = "android" , target_os = "hurd" ,)))] use libc :: { dirent as dirent64 , fstat as fstat64 , ftruncate as ftruncate64 , lseek as lseek64 , lstat as lstat64 , off_t as off64_t , open as open64 , stat as stat64 , } ;}
mkuse!{# [cfg (any (all (target_os = "linux" , not (target_env = "musl")) , target_os = "l4re" , target_os = "hurd"))] use libc :: { dirent64 , fstat64 , ftruncate64 , lseek64 , lstat64 , off64_t , open64 , stat64 } ;}
mkuse!{use crate :: ffi :: { CStr , OsStr , OsString } ;}
mkuse!{use crate :: fmt :: { self , Write as _ } ;}
mkuse!{use crate :: fs :: TryLockError ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , Error , IoSlice , IoSliceMut , SeekFrom } ;}
mkuse!{use crate :: os :: unix :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd } ;}
mkuse!{use crate :: os :: unix :: prelude :: * ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sync :: Arc ;}
mkuse!{use crate :: sys :: common :: small_c_string :: run_path_with_cstr ;}
mkuse!{use crate :: sys :: fd :: FileDesc ;}
mkuse!{pub use crate :: sys :: fs :: common :: exists ;}
mkuse!{use crate :: sys :: time :: SystemTime ;}
mkuse!{# [cfg (all (target_os = "linux" , target_env = "gnu"))] use crate :: sys :: weak :: syscall ;}
mkuse!{# [cfg (target_os = "android")] use crate :: sys :: weak :: weak ;}
mkuse!{use crate :: sys :: { cvt , cvt_r } ;}
mkuse!{use crate :: sys_common :: { AsInner , AsInnerMut , FromInner , IntoInner } ;}
mkuse!{use crate :: { mem , ptr } ;}
mkitem!{mkstruct!{pub struct File (FileDesc) ;}}
mkitem!{macro_rules ! cfg_has_statx { ({ $ ($ then_tt : tt) * } else { $ ($ else_tt : tt) * }) => { cfg_select ! { all (target_os = "linux" , target_env = "gnu") => { $ ($ then_tt) * } _ => { $ ($ else_tt) * } } } ; ($ ($ block_inner : tt) *) => { # [cfg (all (target_os = "linux" , target_env = "gnu"))] { $ ($ block_inner) * } } ; }}
mkitem!{cfg_has_statx ! { { # [derive (Clone)] pub struct FileAttr { stat : stat64 , statx_extra_fields : Option < StatxExtraFields >, } # [derive (Clone)] struct StatxExtraFields { stx_mask : u32 , stx_btime : libc :: statx_timestamp , # [cfg (target_pointer_width = "32")] stx_atime : libc :: statx_timestamp , # [cfg (target_pointer_width = "32")] stx_ctime : libc :: statx_timestamp , # [cfg (target_pointer_width = "32")] stx_mtime : libc :: statx_timestamp , } unsafe fn try_statx (fd : c_int , path : * const c_char , flags : i32 , mask : u32 ,) -> Option < io :: Result < FileAttr >> { use crate :: sync :: atomic :: { Atomic , AtomicU8 , Ordering } ; # [repr (u8)] enum STATX_STATE { Unknown = 0 , Present , Unavailable } static STATX_SAVED_STATE : Atomic < u8 > = AtomicU8 :: new (STATX_STATE :: Unknown as u8) ; syscall ! (fn statx (fd : c_int , pathname : * const c_char , flags : c_int , mask : libc :: c_uint , statxbuf : * mut libc :: statx ,) -> c_int ;) ; let statx_availability = STATX_SAVED_STATE . load (Ordering :: Relaxed) ; if statx_availability == STATX_STATE :: Unavailable as u8 { return None ; } let mut buf : libc :: statx = mem :: zeroed () ; if let Err (err) = cvt (statx (fd , path , flags , mask , & mut buf)) { if STATX_SAVED_STATE . load (Ordering :: Relaxed) == STATX_STATE :: Present as u8 { return Some (Err (err)) ; } let err2 = cvt (statx (0 , ptr :: null () , 0 , libc :: STATX_BASIC_STATS | libc :: STATX_BTIME , ptr :: null_mut ())) . err () . and_then (| e | e . raw_os_error ()) ; if err2 == Some (libc :: EFAULT) { STATX_SAVED_STATE . store (STATX_STATE :: Present as u8 , Ordering :: Relaxed) ; return Some (Err (err)) ; } else { STATX_SAVED_STATE . store (STATX_STATE :: Unavailable as u8 , Ordering :: Relaxed) ; return None ; } } if statx_availability == STATX_STATE :: Unknown as u8 { STATX_SAVED_STATE . store (STATX_STATE :: Present as u8 , Ordering :: Relaxed) ; } let mut stat : stat64 = mem :: zeroed () ; stat . st_dev = libc :: makedev (buf . stx_dev_major , buf . stx_dev_minor) as _ ; stat . st_ino = buf . stx_ino as libc :: ino64_t ; stat . st_nlink = buf . stx_nlink as libc :: nlink_t ; stat . st_mode = buf . stx_mode as libc :: mode_t ; stat . st_uid = buf . stx_uid as libc :: uid_t ; stat . st_gid = buf . stx_gid as libc :: gid_t ; stat . st_rdev = libc :: makedev (buf . stx_rdev_major , buf . stx_rdev_minor) as _ ; stat . st_size = buf . stx_size as off64_t ; stat . st_blksize = buf . stx_blksize as libc :: blksize_t ; stat . st_blocks = buf . stx_blocks as libc :: blkcnt64_t ; stat . st_atime = buf . stx_atime . tv_sec as libc :: time_t ; stat . st_atime_nsec = buf . stx_atime . tv_nsec as _ ; stat . st_mtime = buf . stx_mtime . tv_sec as libc :: time_t ; stat . st_mtime_nsec = buf . stx_mtime . tv_nsec as _ ; stat . st_ctime = buf . stx_ctime . tv_sec as libc :: time_t ; stat . st_ctime_nsec = buf . stx_ctime . tv_nsec as _ ; let extra = StatxExtraFields { stx_mask : buf . stx_mask , stx_btime : buf . stx_btime , # [cfg (target_pointer_width = "32")] stx_atime : buf . stx_atime , # [cfg (target_pointer_width = "32")] stx_ctime : buf . stx_ctime , # [cfg (target_pointer_width = "32")] stx_mtime : buf . stx_mtime , } ; Some (Ok (FileAttr { stat , statx_extra_fields : Some (extra) })) } } else { # [derive (Clone)] pub struct FileAttr { stat : stat64 , } } }}
mkitem!{mkstruct!{struct InnerReadDir { dirp : Dir , root : PathBuf , }}}
mkitem!{mkstruct!{pub struct ReadDir { inner : Arc < InnerReadDir > , end_of_stream : bool , }}}
mkitem!{mkimpl!{impl ReadDir { fn new (inner : InnerReadDir) -> Self { Self { inner : Arc :: new (inner) , end_of_stream : false } } }}}
mkitem!{mkstruct!{struct Dir (* mut libc :: DIR) ;}}
mkitem!{mkimpl!{unsafe impl Send for Dir { }}}
mkitem!{mkimpl!{unsafe impl Sync for Dir { }}}
mkitem!{mkstruct!{# [cfg (any (target_os = "android" , target_os = "linux" , target_os = "solaris" , target_os = "illumos" , target_os = "fuchsia" , target_os = "redox" , target_os = "aix" , target_os = "nto" , target_os = "vita" , target_os = "hurd" ,))] pub struct DirEntry { dir : Arc < InnerReadDir > , entry : dirent64_min , name : crate :: ffi :: CString , }}}
mkitem!{mkstruct!{# [cfg (any (target_os = "android" , target_os = "linux" , target_os = "solaris" , target_os = "illumos" , target_os = "fuchsia" , target_os = "redox" , target_os = "aix" , target_os = "nto" , target_os = "vita" , target_os = "hurd" ,))] struct dirent64_min { d_ino : u64 , # [cfg (not (any (target_os = "solaris" , target_os = "illumos" , target_os = "aix" , target_os = "nto" , target_os = "vita" ,)))] d_type : u8 , }}}
mkitem!{mkstruct!{# [cfg (not (any (target_os = "android" , target_os = "linux" , target_os = "solaris" , target_os = "illumos" , target_os = "fuchsia" , target_os = "redox" , target_os = "aix" , target_os = "nto" , target_os = "vita" , target_os = "hurd" ,)))] pub struct DirEntry { dir : Arc < InnerReadDir > , entry : dirent64 , }}}
mkitem!{mkstruct!{# [derive (Clone)] pub struct OpenOptions { read : bool , write : bool , append : bool , truncate : bool , create : bool , create_new : bool , custom_flags : i32 , mode : mode_t , }}}
mkitem!{mkstruct!{# [derive (Clone , PartialEq , Eq)] pub struct FilePermissions { mode : mode_t , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , Default)] pub struct FileTimes { accessed : Option < SystemTime > , modified : Option < SystemTime > , # [cfg (target_vendor = "apple")] created : Option < SystemTime > , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Eq)] pub struct FileType { mode : mode_t , }}}
mkitem!{mkimpl!{impl PartialEq for FileType { fn eq (& self , other : & Self) -> bool { self . masked () == other . masked () } }}}
mkitem!{mkimpl!{impl core :: hash :: Hash for FileType { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . masked () . hash (state) ; } }}}
mkitem!{mkstruct!{pub struct DirBuilder { mode : mode_t , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone)] struct Mode (mode_t) ;}}
mkitem!{cfg_has_statx ! { { impl FileAttr { fn from_stat64 (stat : stat64) -> Self { Self { stat , statx_extra_fields : None } } # [cfg (target_pointer_width = "32")] pub fn stx_mtime (& self) -> Option <& libc :: statx_timestamp > { if let Some (ext) = & self . statx_extra_fields { if (ext . stx_mask & libc :: STATX_MTIME) != 0 { return Some (& ext . stx_mtime) ; } } None } # [cfg (target_pointer_width = "32")] pub fn stx_atime (& self) -> Option <& libc :: statx_timestamp > { if let Some (ext) = & self . statx_extra_fields { if (ext . stx_mask & libc :: STATX_ATIME) != 0 { return Some (& ext . stx_atime) ; } } None } # [cfg (target_pointer_width = "32")] pub fn stx_ctime (& self) -> Option <& libc :: statx_timestamp > { if let Some (ext) = & self . statx_extra_fields { if (ext . stx_mask & libc :: STATX_CTIME) != 0 { return Some (& ext . stx_ctime) ; } } None } } } else { impl FileAttr { fn from_stat64 (stat : stat64) -> Self { Self { stat } } } } }}
mkitem!{mkimpl!{impl FileAttr { pub fn size (& self) -> u64 { self . stat . st_size as u64 } pub fn perm (& self) -> FilePermissions { FilePermissions { mode : (self . stat . st_mode as mode_t) } } pub fn file_type (& self) -> FileType { FileType { mode : self . stat . st_mode as mode_t } } }}}
mkitem!{mkimpl!{# [cfg (target_os = "netbsd")] impl FileAttr { pub fn modified (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_mtime as i64 , self . stat . st_mtimensec as i64) } pub fn accessed (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_atime as i64 , self . stat . st_atimensec as i64) } pub fn created (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_birthtime as i64 , self . stat . st_birthtimensec as i64) } }}}
mkitem!{mkimpl!{# [cfg (target_os = "aix")] impl FileAttr { pub fn modified (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_mtime . tv_sec as i64 , self . stat . st_mtime . tv_nsec as i64) } pub fn accessed (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_atime . tv_sec as i64 , self . stat . st_atime . tv_nsec as i64) } pub fn created (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_ctime . tv_sec as i64 , self . stat . st_ctime . tv_nsec as i64) } }}}
mkitem!{mkimpl!{# [cfg (not (any (target_os = "netbsd" , target_os = "nto" , target_os = "aix")))] impl FileAttr { # [cfg (not (any (target_os = "vxworks" , target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "hurd" , target_os = "rtems" , target_os = "nuttx" ,)))] pub fn modified (& self) -> io :: Result < SystemTime > { # [cfg (target_pointer_width = "32")] cfg_has_statx ! { if let Some (mtime) = self . stx_mtime () { return SystemTime :: new (mtime . tv_sec , mtime . tv_nsec as i64) ; } } SystemTime :: new (self . stat . st_mtime as i64 , self . stat . st_mtime_nsec as i64) } # [cfg (any (target_os = "vxworks" , target_os = "espidf" , target_os = "vita" , target_os = "rtems" ,))] pub fn modified (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_mtime as i64 , 0) } # [cfg (any (target_os = "horizon" , target_os = "hurd" , target_os = "nuttx"))] pub fn modified (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_mtim . tv_sec as i64 , self . stat . st_mtim . tv_nsec as i64) } # [cfg (not (any (target_os = "vxworks" , target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "hurd" , target_os = "rtems" , target_os = "nuttx" ,)))] pub fn accessed (& self) -> io :: Result < SystemTime > { # [cfg (target_pointer_width = "32")] cfg_has_statx ! { if let Some (atime) = self . stx_atime () { return SystemTime :: new (atime . tv_sec , atime . tv_nsec as i64) ; } } SystemTime :: new (self . stat . st_atime as i64 , self . stat . st_atime_nsec as i64) } # [cfg (any (target_os = "vxworks" , target_os = "espidf" , target_os = "vita" , target_os = "rtems"))] pub fn accessed (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_atime as i64 , 0) } # [cfg (any (target_os = "horizon" , target_os = "hurd" , target_os = "nuttx"))] pub fn accessed (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_atim . tv_sec as i64 , self . stat . st_atim . tv_nsec as i64) } # [cfg (any (target_os = "freebsd" , target_os = "openbsd" , target_vendor = "apple" , target_os = "cygwin" ,))] pub fn created (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_birthtime as i64 , self . stat . st_birthtime_nsec as i64) } # [cfg (not (any (target_os = "freebsd" , target_os = "openbsd" , target_os = "vita" , target_vendor = "apple" , target_os = "cygwin" ,)))] pub fn created (& self) -> io :: Result < SystemTime > { cfg_has_statx ! { if let Some (ext) = & self . statx_extra_fields { return if (ext . stx_mask & libc :: STATX_BTIME) != 0 { SystemTime :: new (ext . stx_btime . tv_sec , ext . stx_btime . tv_nsec as i64) } else { Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "creation time is not available for the filesystem" ,)) } ; } } Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "creation time is not available on this platform currently" ,)) } # [cfg (target_os = "vita")] pub fn created (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_ctime as i64 , 0) } }}}
mkitem!{mkimpl!{# [cfg (target_os = "nto")] impl FileAttr { pub fn modified (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_mtim . tv_sec , self . stat . st_mtim . tv_nsec) } pub fn accessed (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_atim . tv_sec , self . stat . st_atim . tv_nsec) } pub fn created (& self) -> io :: Result < SystemTime > { SystemTime :: new (self . stat . st_ctim . tv_sec , self . stat . st_ctim . tv_nsec) } }}}
mkitem!{mkimpl!{impl AsInner < stat64 > for FileAttr { # [inline] fn as_inner (& self) -> & stat64 { & self . stat } }}}
mkitem!{mkimpl!{impl FilePermissions { pub fn readonly (& self) -> bool { self . mode & 0o222 == 0 } pub fn set_readonly (& mut self , readonly : bool) { if readonly { self . mode &= ! 0o222 ; } else { self . mode |= 0o222 ; } } pub fn mode (& self) -> u32 { self . mode as u32 } }}}
mkitem!{mkimpl!{impl FileTimes { pub fn set_accessed (& mut self , t : SystemTime) { self . accessed = Some (t) ; } pub fn set_modified (& mut self , t : SystemTime) { self . modified = Some (t) ; } # [cfg (target_vendor = "apple")] pub fn set_created (& mut self , t : SystemTime) { self . created = Some (t) ; } }}}
mkitem!{mkimpl!{impl FileType { pub fn is_dir (& self) -> bool { self . is (libc :: S_IFDIR) } pub fn is_file (& self) -> bool { self . is (libc :: S_IFREG) } pub fn is_symlink (& self) -> bool { self . is (libc :: S_IFLNK) } pub fn is (& self , mode : mode_t) -> bool { self . masked () == mode } fn masked (& self) -> mode_t { self . mode & libc :: S_IFMT } }}}
mkitem!{mkimpl!{impl fmt :: Debug for FileType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let FileType { mode } = self ; f . debug_struct ("FileType") . field ("mode" , & Mode (* mode)) . finish () } }}}
mkitem!{mkimpl!{impl FromInner < u32 > for FilePermissions { fn from_inner (mode : u32) -> FilePermissions { FilePermissions { mode : mode as mode_t } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for FilePermissions { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let FilePermissions { mode } = self ; f . debug_struct ("FilePermissions") . field ("mode" , & Mode (* mode)) . finish () } }}}
mkitem!{mkimpl!{impl fmt :: Debug for ReadDir { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * self . inner . root , f) } }}}
mkitem!{mkimpl!{impl Iterator for ReadDir { type Item = io :: Result < DirEntry > ; # [cfg (any (target_os = "android" , target_os = "linux" , target_os = "solaris" , target_os = "fuchsia" , target_os = "redox" , target_os = "illumos" , target_os = "aix" , target_os = "nto" , target_os = "vita" , target_os = "hurd" ,))] fn next (& mut self) -> Option < io :: Result < DirEntry > > { use crate :: sys :: os :: { errno , set_errno } ; if self . end_of_stream { return None ; } unsafe { loop { set_errno (0) ; let entry_ptr : * const dirent64 = readdir64 (self . inner . dirp . 0) ; if entry_ptr . is_null () { self . end_of_stream = true ; return match errno () { 0 => None , e => Some (Err (Error :: from_raw_os_error (e))) , } ; } let name = CStr :: from_ptr ((& raw const (* entry_ptr) . d_name) . cast ()) ; let name_bytes = name . to_bytes () ; if name_bytes == b"." || name_bytes == b".." { continue ; } # [cfg (not (target_os = "vita"))] let entry = dirent64_min { d_ino : (* entry_ptr) . d_ino as u64 , # [cfg (not (any (target_os = "solaris" , target_os = "illumos" , target_os = "aix" , target_os = "nto" ,)))] d_type : (* entry_ptr) . d_type as u8 , } ; # [cfg (target_os = "vita")] let entry = dirent64_min { d_ino : 0u64 } ; return Some (Ok (DirEntry { entry , name : name . to_owned () , dir : Arc :: clone (& self . inner) , })) ; } } } # [cfg (not (any (target_os = "android" , target_os = "linux" , target_os = "solaris" , target_os = "fuchsia" , target_os = "redox" , target_os = "illumos" , target_os = "aix" , target_os = "nto" , target_os = "vita" , target_os = "hurd" ,)))] fn next (& mut self) -> Option < io :: Result < DirEntry > > { if self . end_of_stream { return None ; } unsafe { let mut ret = DirEntry { entry : mem :: zeroed () , dir : Arc :: clone (& self . inner) } ; let mut entry_ptr = ptr :: null_mut () ; loop { let err = readdir64_r (self . inner . dirp . 0 , & mut ret . entry , & mut entry_ptr) ; if err != 0 { if entry_ptr . is_null () { self . end_of_stream = true ; } return Some (Err (Error :: from_raw_os_error (err))) ; } if entry_ptr . is_null () { return None ; } if ret . name_bytes () != b"." && ret . name_bytes () != b".." { return Some (Ok (ret)) ; } } } } }}}

macro_rules! debug_assert_fd_is_open_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function debug_assert_fd_is_open in module {}", module_path!());
    };
}

mkfn!{
    debug_assert_fd_is_open_introspect!();
    # [doc = " Aborts the process if a file desceriptor is not open, if debug asserts are enabled"] # [doc = ""] # [doc = " Many IO syscalls can't be fully trusted about EBADF error codes because those"] # [doc = " might get bubbled up from a remote FUSE server rather than the file descriptor"] # [doc = " in the current process being invalid."] # [doc = ""] # [doc = " So we check file flags instead which live on the file descriptor and not the underlying file."] # [doc = " The downside is that it costs an extra syscall, so we only do it for debug."] # [inline] pub (crate) fn debug_assert_fd_is_open (fd : RawFd) { use crate :: sys :: os :: errno ; if core :: ub_checks :: check_library_ub () { if unsafe { libc :: fcntl (fd , libc :: F_GETFD) } == - 1 && errno () == libc :: EBADF { rtabort ! ("IO Safety violation: owned file descriptor already closed") ; } } }
}
mkitem!{mkimpl!{impl Drop for Dir { fn drop (& mut self) { # [cfg (not (any (miri , target_os = "redox" , target_os = "nto" , target_os = "vita" , target_os = "hurd" , target_os = "espidf" , target_os = "horizon" , target_os = "vxworks" , target_os = "rtems" , target_os = "nuttx" ,)))] { let fd = unsafe { libc :: dirfd (self . 0) } ; debug_assert_fd_is_open (fd) ; } let r = unsafe { libc :: closedir (self . 0) } ; assert ! (r == 0 || crate :: io :: Error :: last_os_error () . is_interrupted () , "unexpected error during closedir: {:?}" , crate :: io :: Error :: last_os_error ()) ; } }}}
mkitem!{mkimpl!{impl DirEntry { pub fn path (& self) -> PathBuf { self . dir . root . join (self . file_name_os_str ()) } pub fn file_name (& self) -> OsString { self . file_name_os_str () . to_os_string () } # [cfg (all (any (all (target_os = "linux" , not (target_env = "musl")) , target_os = "android" , target_os = "fuchsia" , target_os = "hurd" , target_os = "illumos" ,) , not (miri)))] pub fn metadata (& self) -> io :: Result < FileAttr > { let fd = cvt (unsafe { dirfd (self . dir . dirp . 0) }) ? ; let name = self . name_cstr () . as_ptr () ; cfg_has_statx ! { if let Some (ret) = unsafe { try_statx (fd , name , libc :: AT_SYMLINK_NOFOLLOW | libc :: AT_STATX_SYNC_AS_STAT , libc :: STATX_BASIC_STATS | libc :: STATX_BTIME ,) } { return ret ; } } let mut stat : stat64 = unsafe { mem :: zeroed () } ; cvt (unsafe { fstatat64 (fd , name , & mut stat , libc :: AT_SYMLINK_NOFOLLOW) }) ? ; Ok (FileAttr :: from_stat64 (stat)) } # [cfg (any (not (any (all (target_os = "linux" , not (target_env = "musl")) , target_os = "android" , target_os = "fuchsia" , target_os = "hurd" , target_os = "illumos" ,)) , miri))] pub fn metadata (& self) -> io :: Result < FileAttr > { run_path_with_cstr (& self . path () , & lstat) } # [cfg (any (target_os = "solaris" , target_os = "illumos" , target_os = "haiku" , target_os = "vxworks" , target_os = "aix" , target_os = "nto" , target_os = "vita" ,))] pub fn file_type (& self) -> io :: Result < FileType > { self . metadata () . map (| m | m . file_type ()) } # [cfg (not (any (target_os = "solaris" , target_os = "illumos" , target_os = "haiku" , target_os = "vxworks" , target_os = "aix" , target_os = "nto" , target_os = "vita" ,)))] pub fn file_type (& self) -> io :: Result < FileType > { match self . entry . d_type { libc :: DT_CHR => Ok (FileType { mode : libc :: S_IFCHR }) , libc :: DT_FIFO => Ok (FileType { mode : libc :: S_IFIFO }) , libc :: DT_LNK => Ok (FileType { mode : libc :: S_IFLNK }) , libc :: DT_REG => Ok (FileType { mode : libc :: S_IFREG }) , libc :: DT_SOCK => Ok (FileType { mode : libc :: S_IFSOCK }) , libc :: DT_DIR => Ok (FileType { mode : libc :: S_IFDIR }) , libc :: DT_BLK => Ok (FileType { mode : libc :: S_IFBLK }) , _ => self . metadata () . map (| m | m . file_type ()) , } } # [cfg (any (target_os = "linux" , target_os = "cygwin" , target_os = "emscripten" , target_os = "android" , target_os = "solaris" , target_os = "illumos" , target_os = "haiku" , target_os = "l4re" , target_os = "fuchsia" , target_os = "redox" , target_os = "vxworks" , target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "aix" , target_os = "nto" , target_os = "hurd" , target_os = "rtems" , target_vendor = "apple" ,))] pub fn ino (& self) -> u64 { self . entry . d_ino as u64 } # [cfg (any (target_os = "freebsd" , target_os = "openbsd" , target_os = "netbsd" , target_os = "dragonfly"))] pub fn ino (& self) -> u64 { self . entry . d_fileno as u64 } # [cfg (target_os = "nuttx")] pub fn ino (& self) -> u64 { 0 } # [cfg (any (target_os = "netbsd" , target_os = "openbsd" , target_os = "freebsd" , target_os = "dragonfly" , target_vendor = "apple" ,))] fn name_bytes (& self) -> & [u8] { use crate :: slice ; unsafe { slice :: from_raw_parts (self . entry . d_name . as_ptr () as * const u8 , self . entry . d_namlen as usize ,) } } # [cfg (not (any (target_os = "netbsd" , target_os = "openbsd" , target_os = "freebsd" , target_os = "dragonfly" , target_vendor = "apple" ,)))] fn name_bytes (& self) -> & [u8] { self . name_cstr () . to_bytes () } # [cfg (not (any (target_os = "android" , target_os = "linux" , target_os = "solaris" , target_os = "illumos" , target_os = "fuchsia" , target_os = "redox" , target_os = "aix" , target_os = "nto" , target_os = "vita" , target_os = "hurd" ,)))] fn name_cstr (& self) -> & CStr { unsafe { CStr :: from_ptr (self . entry . d_name . as_ptr ()) } } # [cfg (any (target_os = "android" , target_os = "linux" , target_os = "solaris" , target_os = "illumos" , target_os = "fuchsia" , target_os = "redox" , target_os = "aix" , target_os = "nto" , target_os = "vita" , target_os = "hurd" ,))] fn name_cstr (& self) -> & CStr { & self . name } pub fn file_name_os_str (& self) -> & OsStr { OsStr :: from_bytes (self . name_bytes ()) } }}}
mkitem!{mkimpl!{impl OpenOptions { pub fn new () -> OpenOptions { OpenOptions { read : false , write : false , append : false , truncate : false , create : false , create_new : false , custom_flags : 0 , mode : 0o666 , } } pub fn read (& mut self , read : bool) { self . read = read ; } pub fn write (& mut self , write : bool) { self . write = write ; } pub fn append (& mut self , append : bool) { self . append = append ; } pub fn truncate (& mut self , truncate : bool) { self . truncate = truncate ; } pub fn create (& mut self , create : bool) { self . create = create ; } pub fn create_new (& mut self , create_new : bool) { self . create_new = create_new ; } pub fn custom_flags (& mut self , flags : i32) { self . custom_flags = flags ; } pub fn mode (& mut self , mode : u32) { self . mode = mode as mode_t ; } fn get_access_mode (& self) -> io :: Result < c_int > { match (self . read , self . write , self . append) { (true , false , false) => Ok (libc :: O_RDONLY) , (false , true , false) => Ok (libc :: O_WRONLY) , (true , true , false) => Ok (libc :: O_RDWR) , (false , _ , true) => Ok (libc :: O_WRONLY | libc :: O_APPEND) , (true , _ , true) => Ok (libc :: O_RDWR | libc :: O_APPEND) , (false , false , false) => { if self . create || self . create_new || self . truncate { Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "creating or truncating a file requires write or append access" ,)) } else { Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "must specify at least one of read, write, or append access" ,)) } } } } fn get_creation_mode (& self) -> io :: Result < c_int > { match (self . write , self . append) { (true , false) => { } (false , false) => { if self . truncate || self . create || self . create_new { return Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "creating or truncating a file requires write or append access" ,)) ; } } (_ , true) => { if self . truncate && ! self . create_new { return Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "creating or truncating a file requires write or append access" ,)) ; } } } Ok (match (self . create , self . truncate , self . create_new) { (false , false , false) => 0 , (true , false , false) => libc :: O_CREAT , (false , true , false) => libc :: O_TRUNC , (true , true , false) => libc :: O_CREAT | libc :: O_TRUNC , (_ , _ , true) => libc :: O_CREAT | libc :: O_EXCL , }) } }}}
mkitem!{mkimpl!{impl fmt :: Debug for OpenOptions { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let OpenOptions { read , write , append , truncate , create , create_new , custom_flags , mode } = self ; f . debug_struct ("OpenOptions") . field ("read" , read) . field ("write" , write) . field ("append" , append) . field ("truncate" , truncate) . field ("create" , create) . field ("create_new" , create_new) . field ("custom_flags" , custom_flags) . field ("mode" , & Mode (* mode)) . finish () } }}}
mkitem!{mkimpl!{impl File { pub fn open (path : & Path , opts : & OpenOptions) -> io :: Result < File > { run_path_with_cstr (path , & | path | File :: open_c (path , opts)) } pub fn open_c (path : & CStr , opts : & OpenOptions) -> io :: Result < File > { let flags = libc :: O_CLOEXEC | opts . get_access_mode () ? | opts . get_creation_mode () ? | (opts . custom_flags as c_int & ! libc :: O_ACCMODE) ; let fd = cvt_r (| | unsafe { open64 (path . as_ptr () , flags , opts . mode as c_int) }) ? ; Ok (File (unsafe { FileDesc :: from_raw_fd (fd) })) } pub fn file_attr (& self) -> io :: Result < FileAttr > { let fd = self . as_raw_fd () ; cfg_has_statx ! { if let Some (ret) = unsafe { try_statx (fd , c"" . as_ptr () as * const c_char , libc :: AT_EMPTY_PATH | libc :: AT_STATX_SYNC_AS_STAT , libc :: STATX_BASIC_STATS | libc :: STATX_BTIME ,) } { return ret ; } } let mut stat : stat64 = unsafe { mem :: zeroed () } ; cvt (unsafe { fstat64 (fd , & mut stat) }) ? ; Ok (FileAttr :: from_stat64 (stat)) } pub fn fsync (& self) -> io :: Result < () > { cvt_r (| | unsafe { os_fsync (self . as_raw_fd ()) }) ? ; return Ok (()) ; # [cfg (target_vendor = "apple")] unsafe fn os_fsync (fd : c_int) -> c_int { libc :: fcntl (fd , libc :: F_FULLFSYNC) } # [cfg (not (target_vendor = "apple"))] unsafe fn os_fsync (fd : c_int) -> c_int { libc :: fsync (fd) } } pub fn datasync (& self) -> io :: Result < () > { cvt_r (| | unsafe { os_datasync (self . as_raw_fd ()) }) ? ; return Ok (()) ; # [cfg (target_vendor = "apple")] unsafe fn os_datasync (fd : c_int) -> c_int { libc :: fcntl (fd , libc :: F_FULLFSYNC) } # [cfg (any (target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" , target_os = "cygwin" , target_os = "android" , target_os = "netbsd" , target_os = "openbsd" , target_os = "nto" , target_os = "hurd" ,))] unsafe fn os_datasync (fd : c_int) -> c_int { libc :: fdatasync (fd) } # [cfg (not (any (target_os = "android" , target_os = "fuchsia" , target_os = "freebsd" , target_os = "linux" , target_os = "cygwin" , target_os = "netbsd" , target_os = "openbsd" , target_os = "nto" , target_os = "hurd" , target_vendor = "apple" ,)))] unsafe fn os_datasync (fd : c_int) -> c_int { libc :: fsync (fd) } } # [cfg (any (target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "illumos" , target_vendor = "apple" ,))] pub fn lock (& self) -> io :: Result < () > { cvt (unsafe { libc :: flock (self . as_raw_fd () , libc :: LOCK_EX) }) ? ; return Ok (()) ; } # [cfg (target_os = "solaris")] pub fn lock (& self) -> io :: Result < () > { let mut flock : libc :: flock = unsafe { mem :: zeroed () } ; flock . l_type = libc :: F_WRLCK as libc :: c_short ; flock . l_whence = libc :: SEEK_SET as libc :: c_short ; cvt (unsafe { libc :: fcntl (self . as_raw_fd () , libc :: F_SETLKW , & flock) }) ? ; Ok (()) } # [cfg (not (any (target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "solaris" , target_os = "illumos" , target_vendor = "apple" ,)))] pub fn lock (& self) -> io :: Result < () > { Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "lock() not supported")) } # [cfg (any (target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "illumos" , target_vendor = "apple" ,))] pub fn lock_shared (& self) -> io :: Result < () > { cvt (unsafe { libc :: flock (self . as_raw_fd () , libc :: LOCK_SH) }) ? ; return Ok (()) ; } # [cfg (target_os = "solaris")] pub fn lock_shared (& self) -> io :: Result < () > { let mut flock : libc :: flock = unsafe { mem :: zeroed () } ; flock . l_type = libc :: F_RDLCK as libc :: c_short ; flock . l_whence = libc :: SEEK_SET as libc :: c_short ; cvt (unsafe { libc :: fcntl (self . as_raw_fd () , libc :: F_SETLKW , & flock) }) ? ; Ok (()) } # [cfg (not (any (target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "solaris" , target_os = "illumos" , target_vendor = "apple" ,)))] pub fn lock_shared (& self) -> io :: Result < () > { Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "lock_shared() not supported")) } # [cfg (any (target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "illumos" , target_vendor = "apple" ,))] pub fn try_lock (& self) -> Result < () , TryLockError > { let result = cvt (unsafe { libc :: flock (self . as_raw_fd () , libc :: LOCK_EX | libc :: LOCK_NB) }) ; if let Err (err) = result { if err . kind () == io :: ErrorKind :: WouldBlock { Err (TryLockError :: WouldBlock) } else { Err (TryLockError :: Error (err)) } } else { Ok (()) } } # [cfg (target_os = "solaris")] pub fn try_lock (& self) -> Result < () , TryLockError > { let mut flock : libc :: flock = unsafe { mem :: zeroed () } ; flock . l_type = libc :: F_WRLCK as libc :: c_short ; flock . l_whence = libc :: SEEK_SET as libc :: c_short ; let result = cvt (unsafe { libc :: fcntl (self . as_raw_fd () , libc :: F_SETLK , & flock) }) ; if let Err (err) = result { if err . kind () == io :: ErrorKind :: WouldBlock { Err (TryLockError :: WouldBlock) } else { Err (TryLockError :: Error (err)) } } else { Ok (()) } } # [cfg (not (any (target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "solaris" , target_os = "illumos" , target_vendor = "apple" ,)))] pub fn try_lock (& self) -> Result < () , TryLockError > { Err (TryLockError :: Error (io :: const_error ! (io :: ErrorKind :: Unsupported , "try_lock() not supported"))) } # [cfg (any (target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "illumos" , target_vendor = "apple" ,))] pub fn try_lock_shared (& self) -> Result < () , TryLockError > { let result = cvt (unsafe { libc :: flock (self . as_raw_fd () , libc :: LOCK_SH | libc :: LOCK_NB) }) ; if let Err (err) = result { if err . kind () == io :: ErrorKind :: WouldBlock { Err (TryLockError :: WouldBlock) } else { Err (TryLockError :: Error (err)) } } else { Ok (()) } } # [cfg (target_os = "solaris")] pub fn try_lock_shared (& self) -> Result < () , TryLockError > { let mut flock : libc :: flock = unsafe { mem :: zeroed () } ; flock . l_type = libc :: F_RDLCK as libc :: c_short ; flock . l_whence = libc :: SEEK_SET as libc :: c_short ; let result = cvt (unsafe { libc :: fcntl (self . as_raw_fd () , libc :: F_SETLK , & flock) }) ; if let Err (err) = result { if err . kind () == io :: ErrorKind :: WouldBlock { Err (TryLockError :: WouldBlock) } else { Err (TryLockError :: Error (err)) } } else { Ok (()) } } # [cfg (not (any (target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "solaris" , target_os = "illumos" , target_vendor = "apple" ,)))] pub fn try_lock_shared (& self) -> Result < () , TryLockError > { Err (TryLockError :: Error (io :: const_error ! (io :: ErrorKind :: Unsupported , "try_lock_shared() not supported"))) } # [cfg (any (target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "illumos" , target_vendor = "apple" ,))] pub fn unlock (& self) -> io :: Result < () > { cvt (unsafe { libc :: flock (self . as_raw_fd () , libc :: LOCK_UN) }) ? ; return Ok (()) ; } # [cfg (target_os = "solaris")] pub fn unlock (& self) -> io :: Result < () > { let mut flock : libc :: flock = unsafe { mem :: zeroed () } ; flock . l_type = libc :: F_UNLCK as libc :: c_short ; flock . l_whence = libc :: SEEK_SET as libc :: c_short ; cvt (unsafe { libc :: fcntl (self . as_raw_fd () , libc :: F_SETLKW , & flock) }) ? ; Ok (()) } # [cfg (not (any (target_os = "freebsd" , target_os = "fuchsia" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "solaris" , target_os = "illumos" , target_vendor = "apple" ,)))] pub fn unlock (& self) -> io :: Result < () > { Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "unlock() not supported")) } pub fn truncate (& self , size : u64) -> io :: Result < () > { let size : off64_t = size . try_into () . map_err (| e | io :: Error :: new (io :: ErrorKind :: InvalidInput , e)) ? ; cvt_r (| | unsafe { ftruncate64 (self . as_raw_fd () , size) }) . map (drop) } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } # [inline] pub fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } pub fn read_at (& self , buf : & mut [u8] , offset : u64) -> io :: Result < usize > { self . 0 . read_at (buf , offset) } pub fn read_buf (& self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 . read_buf (cursor) } pub fn read_buf_at (& self , cursor : BorrowedCursor < '_ > , offset : u64) -> io :: Result < () > { self . 0 . read_buf_at (cursor , offset) } pub fn read_vectored_at (& self , bufs : & mut [IoSliceMut < '_ >] , offset : u64) -> io :: Result < usize > { self . 0 . read_vectored_at (bufs , offset) } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . 0 . write_vectored (bufs) } # [inline] pub fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } pub fn write_at (& self , buf : & [u8] , offset : u64) -> io :: Result < usize > { self . 0 . write_at (buf , offset) } pub fn write_vectored_at (& self , bufs : & [IoSlice < '_ >] , offset : u64) -> io :: Result < usize > { self . 0 . write_vectored_at (bufs , offset) } # [inline] pub fn flush (& self) -> io :: Result < () > { Ok (()) } pub fn seek (& self , pos : SeekFrom) -> io :: Result < u64 > { let (whence , pos) = match pos { SeekFrom :: Start (off) => (libc :: SEEK_SET , off as i64) , SeekFrom :: End (off) => (libc :: SEEK_END , off) , SeekFrom :: Current (off) => (libc :: SEEK_CUR , off) , } ; let n = cvt (unsafe { lseek64 (self . as_raw_fd () , pos as off64_t , whence) }) ? ; Ok (n as u64) } pub fn size (& self) -> Option < io :: Result < u64 > > { match self . file_attr () . map (| attr | attr . size ()) { Ok (0) => None , result => Some (result) , } } pub fn tell (& self) -> io :: Result < u64 > { self . seek (SeekFrom :: Current (0)) } pub fn duplicate (& self) -> io :: Result < File > { self . 0 . duplicate () . map (File) } pub fn set_permissions (& self , perm : FilePermissions) -> io :: Result < () > { cvt_r (| | unsafe { libc :: fchmod (self . as_raw_fd () , perm . mode) }) ? ; Ok (()) } pub fn set_times (& self , times : FileTimes) -> io :: Result < () > { # [cfg (not (any (target_os = "redox" , target_os = "espidf" , target_os = "horizon" , target_os = "nuttx" ,)))] let to_timespec = | time : Option < SystemTime > | match time { Some (time) if let Some (ts) = time . t . to_timespec () => Ok (ts) , Some (time) if time > crate :: sys :: time :: UNIX_EPOCH => Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "timestamp is too large to set as a file time" ,)) , Some (_) => Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "timestamp is too small to set as a file time" ,)) , None => Ok (libc :: timespec { tv_sec : 0 , tv_nsec : libc :: UTIME_OMIT as _ }) , } ; cfg_select ! { any (target_os = "redox" , target_os = "espidf" , target_os = "horizon" , target_os = "nuttx") => { let _ = times ; Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "setting file times not supported" ,)) } target_vendor = "apple" => { let mut buf = [mem :: MaybeUninit ::< libc :: timespec >:: uninit () ; 3] ; let mut num_times = 0 ; let mut attrlist : libc :: attrlist = unsafe { mem :: zeroed () } ; attrlist . bitmapcount = libc :: ATTR_BIT_MAP_COUNT ; if times . created . is_some () { buf [num_times] . write (to_timespec (times . created) ?) ; num_times += 1 ; attrlist . commonattr |= libc :: ATTR_CMN_CRTIME ; } if times . modified . is_some () { buf [num_times] . write (to_timespec (times . modified) ?) ; num_times += 1 ; attrlist . commonattr |= libc :: ATTR_CMN_MODTIME ; } if times . accessed . is_some () { buf [num_times] . write (to_timespec (times . accessed) ?) ; num_times += 1 ; attrlist . commonattr |= libc :: ATTR_CMN_ACCTIME ; } cvt (unsafe { libc :: fsetattrlist (self . as_raw_fd () , (& raw const attrlist) . cast ::< libc :: c_void > () . cast_mut () , buf . as_ptr () . cast ::< libc :: c_void > () . cast_mut () , num_times * size_of ::< libc :: timespec > () , 0) }) ?; Ok (()) } target_os = "android" => { let times = [to_timespec (times . accessed) ?, to_timespec (times . modified) ?] ; cvt (unsafe { weak ! (fn futimens (fd : c_int , times : * const libc :: timespec) -> c_int ;) ; match futimens . get () { Some (futimens) => futimens (self . as_raw_fd () , times . as_ptr ()) , None => return Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "setting file times requires Android API level >= 19" ,)) , } }) ?; Ok (()) } _ => { # [cfg (all (target_os = "linux" , target_env = "gnu" , target_pointer_width = "32" , not (target_arch = "riscv32")))] { use crate :: sys :: { time :: __timespec64 , weak :: weak } ; weak ! (fn __futimens64 (fd : c_int , times : * const __timespec64) -> c_int ;) ; if let Some (futimens64) = __futimens64 . get () { let to_timespec = | time : Option < SystemTime >| time . map (| time | time . t . to_timespec64 ()) . unwrap_or (__timespec64 :: new (0 , libc :: UTIME_OMIT as _)) ; let times = [to_timespec (times . accessed) , to_timespec (times . modified)] ; cvt (unsafe { futimens64 (self . as_raw_fd () , times . as_ptr ()) }) ?; return Ok (()) ; } } let times = [to_timespec (times . accessed) ?, to_timespec (times . modified) ?] ; cvt (unsafe { libc :: futimens (self . as_raw_fd () , times . as_ptr ()) }) ?; Ok (()) } } } }}}
mkitem!{mkimpl!{impl DirBuilder { pub fn new () -> DirBuilder { DirBuilder { mode : 0o777 } } pub fn mkdir (& self , p : & Path) -> io :: Result < () > { run_path_with_cstr (p , & | p | cvt (unsafe { libc :: mkdir (p . as_ptr () , self . mode) }) . map (| _ | ())) } pub fn set_mode (& mut self , mode : u32) { self . mode = mode as mode_t ; } }}}
mkitem!{mkimpl!{impl fmt :: Debug for DirBuilder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let DirBuilder { mode } = self ; f . debug_struct ("DirBuilder") . field ("mode" , & Mode (* mode)) . finish () } }}}
mkitem!{mkimpl!{impl AsInner < FileDesc > for File { # [inline] fn as_inner (& self) -> & FileDesc { & self . 0 } }}}
mkitem!{mkimpl!{impl AsInnerMut < FileDesc > for File { # [inline] fn as_inner_mut (& mut self) -> & mut FileDesc { & mut self . 0 } }}}
mkitem!{mkimpl!{impl IntoInner < FileDesc > for File { fn into_inner (self) -> FileDesc { self . 0 } }}}
mkitem!{mkimpl!{impl FromInner < FileDesc > for File { fn from_inner (file_desc : FileDesc) -> Self { Self (file_desc) } }}}
mkitem!{mkimpl!{impl AsFd for File { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { self . 0 . as_fd () } }}}
mkitem!{mkimpl!{impl AsRawFd for File { # [inline] fn as_raw_fd (& self) -> RawFd { self . 0 . as_raw_fd () } }}}
mkitem!{mkimpl!{impl IntoRawFd for File { fn into_raw_fd (self) -> RawFd { self . 0 . into_raw_fd () } }}}
mkitem!{mkimpl!{impl FromRawFd for File { unsafe fn from_raw_fd (raw_fd : RawFd) -> Self { Self (FromRawFd :: from_raw_fd (raw_fd)) } }}}
mkitem!{mkimpl!{impl fmt :: Debug for File { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (any (target_os = "linux" , target_os = "illumos" , target_os = "solaris"))] fn get_path (fd : c_int) -> Option < PathBuf > { let mut p = PathBuf :: from ("/proc/self/fd") ; p . push (& fd . to_string ()) ; run_path_with_cstr (& p , & readlink) . ok () } # [cfg (any (target_vendor = "apple" , target_os = "netbsd"))] fn get_path (fd : c_int) -> Option < PathBuf > { let mut buf = vec ! [0 ; libc :: PATH_MAX as usize] ; let n = unsafe { libc :: fcntl (fd , libc :: F_GETPATH , buf . as_ptr ()) } ; if n == - 1 { cfg_select ! { target_os = "netbsd" => { let mut p = PathBuf :: from ("/proc/self/fd") ; p . push (& fd . to_string ()) ; return run_path_with_cstr (& p , & readlink) . ok () } _ => { return None ; } } } let l = buf . iter () . position (| & c | c == 0) . unwrap () ; buf . truncate (l as usize) ; buf . shrink_to_fit () ; Some (PathBuf :: from (OsString :: from_vec (buf))) } # [cfg (target_os = "freebsd")] fn get_path (fd : c_int) -> Option < PathBuf > { let info = Box :: < libc :: kinfo_file > :: new_zeroed () ; let mut info = unsafe { info . assume_init () } ; info . kf_structsize = size_of :: < libc :: kinfo_file > () as libc :: c_int ; let n = unsafe { libc :: fcntl (fd , libc :: F_KINFO , & mut * info) } ; if n == - 1 { return None ; } let buf = unsafe { CStr :: from_ptr (info . kf_path . as_mut_ptr ()) . to_bytes () . to_vec () } ; Some (PathBuf :: from (OsString :: from_vec (buf))) } # [cfg (target_os = "vxworks")] fn get_path (fd : c_int) -> Option < PathBuf > { let mut buf = vec ! [0 ; libc :: PATH_MAX as usize] ; let n = unsafe { libc :: ioctl (fd , libc :: FIOGETNAME , buf . as_ptr ()) } ; if n == - 1 { return None ; } let l = buf . iter () . position (| & c | c == 0) . unwrap () ; buf . truncate (l as usize) ; Some (PathBuf :: from (OsString :: from_vec (buf))) } # [cfg (not (any (target_os = "linux" , target_os = "vxworks" , target_os = "freebsd" , target_os = "netbsd" , target_os = "illumos" , target_os = "solaris" , target_vendor = "apple" ,)))] fn get_path (_fd : c_int) -> Option < PathBuf > { None } fn get_mode (fd : c_int) -> Option < (bool , bool) > { let mode = unsafe { libc :: fcntl (fd , libc :: F_GETFL) } ; if mode == - 1 { return None ; } match mode & libc :: O_ACCMODE { libc :: O_RDONLY => Some ((true , false)) , libc :: O_RDWR => Some ((true , true)) , libc :: O_WRONLY => Some ((false , true)) , _ => None , } } let fd = self . as_raw_fd () ; let mut b = f . debug_struct ("File") ; b . field ("fd" , & fd) ; if let Some (path) = get_path (fd) { b . field ("path" , & path) ; } if let Some ((read , write)) = get_mode (fd) { b . field ("read" , & read) . field ("write" , & write) ; } b . finish () } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Mode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self (mode) = * self ; write ! (f , "0o{mode:06o}") ? ; let entry_type = match mode & libc :: S_IFMT { libc :: S_IFDIR => 'd' , libc :: S_IFBLK => 'b' , libc :: S_IFCHR => 'c' , libc :: S_IFLNK => 'l' , libc :: S_IFIFO => 'p' , libc :: S_IFREG => '-' , _ => return Ok (()) , } ; f . write_str (" (") ? ; f . write_char (entry_type) ? ; f . write_char (if mode & libc :: S_IRUSR != 0 { 'r' } else { '-' }) ? ; f . write_char (if mode & libc :: S_IWUSR != 0 { 'w' } else { '-' }) ? ; let owner_executable = mode & libc :: S_IXUSR != 0 ; let setuid = mode as c_int & libc :: S_ISUID as c_int != 0 ; f . write_char (match (owner_executable , setuid) { (true , true) => 's' , (false , true) => 'S' , (true , false) => 'x' , (false , false) => '-' , }) ? ; f . write_char (if mode & libc :: S_IRGRP != 0 { 'r' } else { '-' }) ? ; f . write_char (if mode & libc :: S_IWGRP != 0 { 'w' } else { '-' }) ? ; let group_executable = mode & libc :: S_IXGRP != 0 ; let setgid = mode as c_int & libc :: S_ISGID as c_int != 0 ; f . write_char (match (group_executable , setgid) { (true , true) => 's' , (false , true) => 'S' , (true , false) => 'x' , (false , false) => '-' , }) ? ; f . write_char (if mode & libc :: S_IROTH != 0 { 'r' } else { '-' }) ? ; f . write_char (if mode & libc :: S_IWOTH != 0 { 'w' } else { '-' }) ? ; let other_executable = mode & libc :: S_IXOTH != 0 ; let sticky = mode as c_int & libc :: S_ISVTX as c_int != 0 ; f . write_char (match (entry_type , other_executable , sticky) { ('d' , true , true) => 't' , ('d' , false , true) => 'T' , (_ , true , _) => 'x' , (_ , false , _) => '-' , }) ? ; f . write_char (')') } }}}

macro_rules! readdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readdir in module {}", module_path!());
    };
}

mkfn!{
    readdir_introspect!();
    pub fn readdir (path : & Path) -> io :: Result < ReadDir > { let ptr = run_path_with_cstr (path , & | p | unsafe { Ok (libc :: opendir (p . as_ptr ())) }) ? ; if ptr . is_null () { Err (Error :: last_os_error ()) } else { let root = path . to_path_buf () ; let inner = InnerReadDir { dirp : Dir (ptr) , root } ; Ok (ReadDir :: new (inner)) } }
}

macro_rules! unlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unlink in module {}", module_path!());
    };
}

mkfn!{
    unlink_introspect!();
    pub fn unlink (p : & CStr) -> io :: Result < () > { cvt (unsafe { libc :: unlink (p . as_ptr ()) }) . map (| _ | ()) }
}

macro_rules! rename_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rename in module {}", module_path!());
    };
}

mkfn!{
    rename_introspect!();
    pub fn rename (old : & CStr , new : & CStr) -> io :: Result < () > { cvt (unsafe { libc :: rename (old . as_ptr () , new . as_ptr ()) }) . map (| _ | ()) }
}

macro_rules! set_perm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_perm in module {}", module_path!());
    };
}

mkfn!{
    set_perm_introspect!();
    pub fn set_perm (p : & CStr , perm : FilePermissions) -> io :: Result < () > { cvt_r (| | unsafe { libc :: chmod (p . as_ptr () , perm . mode) }) . map (| _ | ()) }
}

macro_rules! rmdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rmdir in module {}", module_path!());
    };
}

mkfn!{
    rmdir_introspect!();
    pub fn rmdir (p : & CStr) -> io :: Result < () > { cvt (unsafe { libc :: rmdir (p . as_ptr ()) }) . map (| _ | ()) }
}

macro_rules! readlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readlink in module {}", module_path!());
    };
}

mkfn!{
    readlink_introspect!();
    pub fn readlink (c_path : & CStr) -> io :: Result < PathBuf > { let p = c_path . as_ptr () ; let mut buf = Vec :: with_capacity (256) ; loop { let buf_read = cvt (unsafe { libc :: readlink (p , buf . as_mut_ptr () as * mut _ , buf . capacity ()) }) ? as usize ; unsafe { buf . set_len (buf_read) ; } if buf_read != buf . capacity () { buf . shrink_to_fit () ; return Ok (PathBuf :: from (OsString :: from_vec (buf))) ; } buf . reserve (1) ; } }
}

macro_rules! symlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symlink in module {}", module_path!());
    };
}

mkfn!{
    symlink_introspect!();
    pub fn symlink (original : & CStr , link : & CStr) -> io :: Result < () > { cvt (unsafe { libc :: symlink (original . as_ptr () , link . as_ptr ()) }) . map (| _ | ()) }
}

macro_rules! link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link in module {}", module_path!());
    };
}

mkfn!{
    link_introspect!();
    pub fn link (original : & CStr , link : & CStr) -> io :: Result < () > { cfg_select ! { any (target_os = "vxworks" , target_os = "redox" , target_os = "android" , target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_env = "nto70") => { cvt (unsafe { libc :: link (original . as_ptr () , link . as_ptr ()) }) ?; } _ => { cvt (unsafe { libc :: linkat (libc :: AT_FDCWD , original . as_ptr () , libc :: AT_FDCWD , link . as_ptr () , 0) }) ?; } } Ok (()) }
}

macro_rules! stat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stat in module {}", module_path!());
    };
}

mkfn!{
    stat_introspect!();
    pub fn stat (p : & CStr) -> io :: Result < FileAttr > { cfg_has_statx ! { if let Some (ret) = unsafe { try_statx (libc :: AT_FDCWD , p . as_ptr () , libc :: AT_STATX_SYNC_AS_STAT , libc :: STATX_BASIC_STATS | libc :: STATX_BTIME ,) } { return ret ; } } let mut stat : stat64 = unsafe { mem :: zeroed () } ; cvt (unsafe { stat64 (p . as_ptr () , & mut stat) }) ? ; Ok (FileAttr :: from_stat64 (stat)) }
}

macro_rules! lstat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lstat in module {}", module_path!());
    };
}

mkfn!{
    lstat_introspect!();
    pub fn lstat (p : & CStr) -> io :: Result < FileAttr > { cfg_has_statx ! { if let Some (ret) = unsafe { try_statx (libc :: AT_FDCWD , p . as_ptr () , libc :: AT_SYMLINK_NOFOLLOW | libc :: AT_STATX_SYNC_AS_STAT , libc :: STATX_BASIC_STATS | libc :: STATX_BTIME ,) } { return ret ; } } let mut stat : stat64 = unsafe { mem :: zeroed () } ; cvt (unsafe { lstat64 (p . as_ptr () , & mut stat) }) ? ; Ok (FileAttr :: from_stat64 (stat)) }
}

macro_rules! canonicalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function canonicalize in module {}", module_path!());
    };
}

mkfn!{
    canonicalize_introspect!();
    pub fn canonicalize (path : & CStr) -> io :: Result < PathBuf > { let r = unsafe { libc :: realpath (path . as_ptr () , ptr :: null_mut ()) } ; if r . is_null () { return Err (io :: Error :: last_os_error ()) ; } Ok (PathBuf :: from (OsString :: from_vec (unsafe { let buf = CStr :: from_ptr (r) . to_bytes () . to_vec () ; libc :: free (r as * mut _) ; buf }))) }
}

macro_rules! open_from_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function open_from in module {}", module_path!());
    };
}

mkfn!{
    open_from_introspect!();
    fn open_from (from : & Path) -> io :: Result < (crate :: fs :: File , crate :: fs :: Metadata) > { use crate :: fs :: File ; use crate :: sys :: fs :: common :: NOT_FILE_ERROR ; let reader = File :: open (from) ? ; let metadata = reader . metadata () ? ; if ! metadata . is_file () { return Err (NOT_FILE_ERROR) ; } Ok ((reader , metadata)) }
}

macro_rules! open_to_and_set_permissions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function open_to_and_set_permissions in module {}", module_path!());
    };
}

mkfn!{
    open_to_and_set_permissions_introspect!();
    # [cfg (target_os = "espidf")] fn open_to_and_set_permissions (to : & Path , _reader_metadata : & crate :: fs :: Metadata ,) -> io :: Result < (crate :: fs :: File , crate :: fs :: Metadata) > { use crate :: fs :: OpenOptions ; let writer = OpenOptions :: new () . open (to) ? ; let writer_metadata = writer . metadata () ? ; Ok ((writer , writer_metadata)) }
}

macro_rules! open_to_and_set_permissions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function open_to_and_set_permissions in module {}", module_path!());
    };
}

mkfn!{
    open_to_and_set_permissions_introspect!();
    # [cfg (not (target_os = "espidf"))] fn open_to_and_set_permissions (to : & Path , reader_metadata : & crate :: fs :: Metadata ,) -> io :: Result < (crate :: fs :: File , crate :: fs :: Metadata) > { use crate :: fs :: OpenOptions ; use crate :: os :: unix :: fs :: { OpenOptionsExt , PermissionsExt } ; let perm = reader_metadata . permissions () ; let writer = OpenOptions :: new () . mode (perm . mode ()) . write (true) . create (true) . truncate (true) . open (to) ? ; let writer_metadata = writer . metadata () ? ; # [cfg (not (target_os = "vita"))] if writer_metadata . is_file () { writer . set_permissions (perm) ? ; } Ok ((writer , writer_metadata)) }
}
mkmod!{cfm, { 
                getname!(cfm);
                getsrc!(cfm);
                getpath!(cfm);
                get_deps!(cfm);
                get_crates!(cfm);
                mkinclude!(cfm);
                mkuse!{use crate :: fs :: { File , Metadata } ;}
mkuse!{use crate :: io :: { BorrowedCursor , IoSlice , IoSliceMut , Read , Result , Write } ;}
mkitem!{mkstruct!{# [allow (dead_code)] pub struct CachedFileMetadata (pub File , pub Metadata) ;}}
mkitem!{mkimpl!{impl Read for CachedFileMetadata { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { self . 0 . read (buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> Result < usize > { self . 0 . read_vectored (bufs) } fn read_buf (& mut self , cursor : BorrowedCursor < '_ >) -> Result < () > { self . 0 . read_buf (cursor) } # [inline] fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> Result < usize > { self . 0 . read_to_end (buf) } fn read_to_string (& mut self , buf : & mut String) -> Result < usize > { self . 0 . read_to_string (buf) } }}}
mkitem!{mkimpl!{impl Write for CachedFileMetadata { fn write (& mut self , buf : & [u8]) -> Result < usize > { self . 0 . write (buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> Result < usize > { self . 0 . write_vectored (bufs) } # [inline] fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } # [inline] fn flush (& mut self) -> Result < () > { self . 0 . flush () } }}} 
            }}
mkuse!{# [cfg (any (target_os = "linux" , target_os = "android"))] pub (crate) use cfm :: CachedFileMetadata ;}

macro_rules! copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy in module {}", module_path!());
    };
}

mkfn!{
    copy_introspect!();
    # [cfg (not (target_vendor = "apple"))] pub fn copy (from : & Path , to : & Path) -> io :: Result < u64 > { let (reader , reader_metadata) = open_from (from) ? ; let (writer , writer_metadata) = open_to_and_set_permissions (to , & reader_metadata) ? ; io :: copy (& mut cfm :: CachedFileMetadata (reader , reader_metadata) , & mut cfm :: CachedFileMetadata (writer , writer_metadata) ,) }
}

macro_rules! copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy in module {}", module_path!());
    };
}

mkfn!{
    copy_introspect!();
    # [cfg (target_vendor = "apple")] pub fn copy (from : & Path , to : & Path) -> io :: Result < u64 > { const COPYFILE_ALL : libc :: copyfile_flags_t = libc :: COPYFILE_METADATA | libc :: COPYFILE_DATA ; struct FreeOnDrop (libc :: copyfile_state_t) ; impl Drop for FreeOnDrop { fn drop (& mut self) { unsafe { libc :: copyfile_state_free (self . 0) ; } } } let (reader , reader_metadata) = open_from (from) ? ; let clonefile_result = run_path_with_cstr (to , & | to | { cvt (unsafe { libc :: fclonefileat (reader . as_raw_fd () , libc :: AT_FDCWD , to . as_ptr () , 0) }) }) ; match clonefile_result { Ok (_) => return Ok (reader_metadata . len ()) , Err (e) => match e . raw_os_error () { Some (libc :: ENOTSUP) | Some (libc :: EEXIST) | Some (libc :: EXDEV) => () , _ => return Err (e) , } , } let (writer , writer_metadata) = open_to_and_set_permissions (to , & reader_metadata) ? ; let state = unsafe { let state = libc :: copyfile_state_alloc () ; if state . is_null () { return Err (crate :: io :: Error :: last_os_error ()) ; } FreeOnDrop (state) } ; let flags = if writer_metadata . is_file () { COPYFILE_ALL } else { libc :: COPYFILE_DATA } ; cvt (unsafe { libc :: fcopyfile (reader . as_raw_fd () , writer . as_raw_fd () , state . 0 , flags) }) ? ; let mut bytes_copied : libc :: off_t = 0 ; cvt (unsafe { libc :: copyfile_state_get (state . 0 , libc :: COPYFILE_STATE_COPIED as u32 , (& raw mut bytes_copied) as * mut libc :: c_void ,) }) ? ; Ok (bytes_copied as u64) }
}

macro_rules! chown_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function chown in module {}", module_path!());
    };
}

mkfn!{
    chown_introspect!();
    pub fn chown (path : & Path , uid : u32 , gid : u32) -> io :: Result < () > { run_path_with_cstr (path , & | path | { cvt (unsafe { libc :: chown (path . as_ptr () , uid as libc :: uid_t , gid as libc :: gid_t) }) . map (| _ | ()) }) }
}

macro_rules! fchown_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fchown in module {}", module_path!());
    };
}

mkfn!{
    fchown_introspect!();
    pub fn fchown (fd : c_int , uid : u32 , gid : u32) -> io :: Result < () > { cvt (unsafe { libc :: fchown (fd , uid as libc :: uid_t , gid as libc :: gid_t) }) ? ; Ok (()) }
}

macro_rules! lchown_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lchown in module {}", module_path!());
    };
}

mkfn!{
    lchown_introspect!();
    # [cfg (not (target_os = "vxworks"))] pub fn lchown (path : & Path , uid : u32 , gid : u32) -> io :: Result < () > { run_path_with_cstr (path , & | path | { cvt (unsafe { libc :: lchown (path . as_ptr () , uid as libc :: uid_t , gid as libc :: gid_t) }) . map (| _ | ()) }) }
}

macro_rules! lchown_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lchown in module {}", module_path!());
    };
}

mkfn!{
    lchown_introspect!();
    # [cfg (target_os = "vxworks")] pub fn lchown (path : & Path , uid : u32 , gid : u32) -> io :: Result < () > { let (_ , _ , _) = (path , uid , gid) ; Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "lchown not supported by vxworks")) }
}

macro_rules! chroot_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function chroot in module {}", module_path!());
    };
}

mkfn!{
    chroot_introspect!();
    # [cfg (not (any (target_os = "fuchsia" , target_os = "vxworks")))] pub fn chroot (dir : & Path) -> io :: Result < () > { run_path_with_cstr (dir , & | dir | cvt (unsafe { libc :: chroot (dir . as_ptr ()) }) . map (| _ | ())) }
}

macro_rules! chroot_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function chroot in module {}", module_path!());
    };
}

mkfn!{
    chroot_introspect!();
    # [cfg (target_os = "vxworks")] pub fn chroot (dir : & Path) -> io :: Result < () > { let _ = dir ; Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "chroot not supported by vxworks")) }
}

macro_rules! mkfifo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mkfifo in module {}", module_path!());
    };
}

mkfn!{
    mkfifo_introspect!();
    pub fn mkfifo (path : & Path , mode : u32) -> io :: Result < () > { run_path_with_cstr (path , & | path | { cvt (unsafe { libc :: mkfifo (path . as_ptr () , mode . try_into () . unwrap ()) }) . map (| _ | ()) }) }
}
mkuse!{pub use remove_dir_impl :: remove_dir_all ;}
mkmod!{remove_dir_impl, { 
                getname!(remove_dir_impl);
                getsrc!(remove_dir_impl);
                getpath!(remove_dir_impl);
                get_deps!(remove_dir_impl);
                get_crates!(remove_dir_impl);
                mkinclude!(remove_dir_impl);
                mkuse!{pub use crate :: sys :: fs :: common :: remove_dir_all ;} 
            }}
mkmod!{remove_dir_impl, { 
                getname!(remove_dir_impl);
                getsrc!(remove_dir_impl);
                getpath!(remove_dir_impl);
                get_deps!(remove_dir_impl);
                get_crates!(remove_dir_impl);
                mkinclude!(remove_dir_impl);
                mkuse!{# [cfg (not (all (target_os = "linux" , target_env = "gnu")))] use libc :: { fdopendir , openat , unlinkat } ;}
mkuse!{# [cfg (all (target_os = "linux" , target_env = "gnu"))] use libc :: { fdopendir , openat64 as openat , unlinkat } ;}
mkuse!{use super :: { Dir , DirEntry , InnerReadDir , ReadDir , lstat } ;}
mkuse!{use crate :: ffi :: CStr ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: os :: unix :: io :: { AsRawFd , FromRawFd , IntoRawFd } ;}
mkuse!{use crate :: os :: unix :: prelude :: { OwnedFd , RawFd } ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sys :: common :: small_c_string :: run_path_with_cstr ;}
mkuse!{use crate :: sys :: { cvt , cvt_r } ;}
mkuse!{use crate :: sys_common :: ignore_notfound ;}

macro_rules! openat_nofollow_dironly_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function openat_nofollow_dironly in module {}", module_path!());
    };
}

mkfn!{
    openat_nofollow_dironly_introspect!();
    pub fn openat_nofollow_dironly (parent_fd : Option < RawFd > , p : & CStr) -> io :: Result < OwnedFd > { let fd = cvt_r (| | unsafe { openat (parent_fd . unwrap_or (libc :: AT_FDCWD) , p . as_ptr () , libc :: O_CLOEXEC | libc :: O_RDONLY | libc :: O_NOFOLLOW | libc :: O_DIRECTORY ,) }) ? ; Ok (unsafe { OwnedFd :: from_raw_fd (fd) }) }
}

macro_rules! fdreaddir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fdreaddir in module {}", module_path!());
    };
}

mkfn!{
    fdreaddir_introspect!();
    fn fdreaddir (dir_fd : OwnedFd) -> io :: Result < (ReadDir , RawFd) > { let ptr = unsafe { fdopendir (dir_fd . as_raw_fd ()) } ; if ptr . is_null () { return Err (io :: Error :: last_os_error ()) ; } let dirp = Dir (ptr) ; let new_parent_fd = dir_fd . into_raw_fd () ; let dummy_root = PathBuf :: new () ; let inner = InnerReadDir { dirp , root : dummy_root } ; Ok ((ReadDir :: new (inner) , new_parent_fd)) }
}

macro_rules! is_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_dir in module {}", module_path!());
    };
}

mkfn!{
    is_dir_introspect!();
    # [cfg (any (target_os = "solaris" , target_os = "illumos" , target_os = "haiku" , target_os = "vxworks" , target_os = "aix" ,))] fn is_dir (_ent : & DirEntry) -> Option < bool > { None }
}

macro_rules! is_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_dir in module {}", module_path!());
    };
}

mkfn!{
    is_dir_introspect!();
    # [cfg (not (any (target_os = "solaris" , target_os = "illumos" , target_os = "haiku" , target_os = "vxworks" , target_os = "aix" ,)))] fn is_dir (ent : & DirEntry) -> Option < bool > { match ent . entry . d_type { libc :: DT_UNKNOWN => None , libc :: DT_DIR => Some (true) , _ => Some (false) , } }
}

macro_rules! is_enoent_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_enoent in module {}", module_path!());
    };
}

mkfn!{
    is_enoent_introspect!();
    fn is_enoent (result : & io :: Result < () >) -> bool { if let Err (err) = result && matches ! (err . raw_os_error () , Some (libc :: ENOENT)) { true } else { false } }
}

macro_rules! remove_dir_all_recursive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all_recursive in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_recursive_introspect!();
    fn remove_dir_all_recursive (parent_fd : Option < RawFd > , path : & CStr) -> io :: Result < () > { let fd = match openat_nofollow_dironly (parent_fd , & path) { Err (err) if matches ! (err . raw_os_error () , Some (libc :: ENOTDIR | libc :: ELOOP)) => { return match parent_fd { Some (parent_fd) => { cvt (unsafe { unlinkat (parent_fd , path . as_ptr () , 0) }) . map (drop) } None => Err (err) , } ; } result => result ? , } ; let (dir , fd) = fdreaddir (fd) ? ; for child in dir { let child = child ? ; let child_name = child . name_cstr () ; let result : io :: Result < () > = try { match is_dir (& child) { Some (true) => { remove_dir_all_recursive (Some (fd) , child_name) ? ; } Some (false) => { cvt (unsafe { unlinkat (fd , child_name . as_ptr () , 0) }) ? ; } None => { remove_dir_all_recursive (Some (fd) , child_name) ? ; } } } ; if result . is_err () && ! is_enoent (& result) { return result ; } } ignore_notfound (cvt (unsafe { unlinkat (parent_fd . unwrap_or (libc :: AT_FDCWD) , path . as_ptr () , libc :: AT_REMOVEDIR) })) ? ; Ok (()) }
}

macro_rules! remove_dir_all_modern_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all_modern in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_modern_introspect!();
    fn remove_dir_all_modern (p : & CStr) -> io :: Result < () > { let attr = lstat (p) ? ; if attr . file_type () . is_symlink () { super :: unlink (p) } else { remove_dir_all_recursive (None , & p) } }
}

macro_rules! remove_dir_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_introspect!();
    pub fn remove_dir_all (p : & Path) -> io :: Result < () > { run_path_with_cstr (p , & remove_dir_all_modern) }
} 
            }}