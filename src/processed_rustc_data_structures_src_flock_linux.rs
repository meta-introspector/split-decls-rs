/* FP:linux.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_linux_USE_0001
/* FP:linux.rs-0002 */ use std :: fs :: { File , OpenOptions } ;
/* FP:linux.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_linux_USE_0002
/* FP:linux.rs-0004 */ use std :: io ;
/* FP:linux.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_linux_USE_0003
/* FP:linux.rs-0006 */ use std :: os :: unix :: prelude :: * ;
/* FP:linux.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_linux_USE_0004
/* FP:linux.rs-0008 */ use std :: path :: Path ;
/* FP:linux.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_linux_STRUCT_0005
/* FP:linux.rs-0010 */ # [derive (Debug)] pub struct Lock { _file : File , }
/* FP:linux.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_linux_IMPL_0006
/* FP:linux.rs-0012 */ impl Lock { pub fn new (p : & Path , wait : bool , create : bool , exclusive : bool) -> io :: Result < Lock > { let file = OpenOptions :: new () . read (true) . write (true) . create (create) . mode (0o600) . open (p) ? ; let mut operation = if exclusive { libc :: LOCK_EX } else { libc :: LOCK_SH } ; if ! wait { operation |= libc :: LOCK_NB } let ret = unsafe { libc :: flock (file . as_raw_fd () , operation) } ; if ret == - 1 { Err (io :: Error :: last_os_error ()) } else { Ok (Lock { _file : file }) } } pub fn error_unsupported (err : & io :: Error) -> bool { matches ! (err . raw_os_error () , Some (libc :: ENOTSUP) | Some (libc :: ENOSYS)) } }