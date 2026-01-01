/* FP:windows.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_windows_USE_0001
/* FP:windows.rs-0002 */ use std :: fs :: { File , OpenOptions } ;
/* FP:windows.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_windows_USE_0002
/* FP:windows.rs-0004 */ use std :: io ;
/* FP:windows.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_windows_USE_0003
/* FP:windows.rs-0006 */ use std :: os :: windows :: prelude :: * ;
/* FP:windows.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_windows_USE_0004
/* FP:windows.rs-0008 */ use std :: path :: Path ;
/* FP:windows.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_windows_USE_0005
/* FP:windows.rs-0010 */ use tracing :: debug ;
/* FP:windows.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_windows_USE_0006
/* FP:windows.rs-0012 */ use windows :: Win32 :: Foundation :: { ERROR_INVALID_FUNCTION , HANDLE } ;
/* FP:windows.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_windows_USE_0007
/* FP:windows.rs-0014 */ use windows :: Win32 :: Storage :: FileSystem :: { FILE_SHARE_DELETE , FILE_SHARE_READ , FILE_SHARE_WRITE , LOCK_FILE_FLAGS , LOCKFILE_EXCLUSIVE_LOCK , LOCKFILE_FAIL_IMMEDIATELY , LockFileEx , } ;
/* FP:windows.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_windows_USE_0008
/* FP:windows.rs-0016 */ use windows :: Win32 :: System :: IO :: OVERLAPPED ;
/* FP:windows.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_windows_STRUCT_0009
/* FP:windows.rs-0018 */ # [derive (Debug)] pub struct Lock { _file : File , }
/* FP:windows.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_windows_IMPL_0010
/* FP:windows.rs-0020 */ impl Lock { pub fn new (p : & Path , wait : bool , create : bool , exclusive : bool) -> io :: Result < Lock > { assert ! (p . parent () . unwrap () . exists () , "Parent directory of lock-file must exist: {}" , p . display ()) ; let share_mode = FILE_SHARE_DELETE | FILE_SHARE_READ | FILE_SHARE_WRITE ; let mut open_options = OpenOptions :: new () ; open_options . read (true) . share_mode (share_mode . 0) ; if create { open_options . create (true) . write (true) ; } debug ! ("attempting to open lock file `{}`" , p . display ()) ; let file = match open_options . open (p) { Ok (file) => { debug ! ("lock file opened successfully") ; file } Err (err) => { debug ! ("error opening lock file: {}" , err) ; return Err (err) ; } } ; let mut flags = LOCK_FILE_FLAGS :: default () ; if ! wait { flags |= LOCKFILE_FAIL_IMMEDIATELY ; } if exclusive { flags |= LOCKFILE_EXCLUSIVE_LOCK ; } let mut overlapped = OVERLAPPED :: default () ; debug ! ("attempting to acquire lock on lock file `{}`" , p . display ()) ; unsafe { LockFileEx (HANDLE (file . as_raw_handle ()) , flags , None , u32 :: MAX , u32 :: MAX , & mut overlapped ,) } . map_err (| e | { let err = io :: Error :: from_raw_os_error (e . code () . 0) ; debug ! ("failed acquiring file lock: {}" , err) ; err }) ? ; debug ! ("successfully acquired lock") ; Ok (Lock { _file : file }) } pub fn error_unsupported (err : & io :: Error) -> bool { err . raw_os_error () == Some (ERROR_INVALID_FUNCTION . 0 as i32) } }