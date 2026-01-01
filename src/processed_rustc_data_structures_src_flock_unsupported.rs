/* FP:unsupported.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_unsupported_USE_0001
/* FP:unsupported.rs-0002 */ use std :: io ;
/* FP:unsupported.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_unsupported_USE_0002
/* FP:unsupported.rs-0004 */ use std :: path :: Path ;
/* FP:unsupported.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_unsupported_STRUCT_0003
/* FP:unsupported.rs-0006 */ # [derive (Debug)] pub struct Lock (()) ;
/* FP:unsupported.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flock_unsupported_IMPL_0004
/* FP:unsupported.rs-0008 */ impl Lock { pub fn new (_p : & Path , _wait : bool , _create : bool , _exclusive : bool) -> io :: Result < Lock > { let msg = "file locks not supported on this platform" ; Err (io :: Error :: new (io :: ErrorKind :: Other , msg)) } pub fn error_unsupported (_err : & io :: Error) -> bool { true } }