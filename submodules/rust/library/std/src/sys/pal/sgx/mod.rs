mkuse!{use crate :: io :: ErrorKind ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicBool , Ordering } ;}
mkmod!{abi, { 
                getname!(abi);
                getsrc!(abi);
                getpath!(abi);
                get_deps!(abi);
                get_crates!(abi);
                mkinclude!(abi);
                 
            }}
mkmod!{libunwind_integration, { 
                getname!(libunwind_integration);
                getsrc!(libunwind_integration);
                getpath!(libunwind_integration);
                get_deps!(libunwind_integration);
                get_crates!(libunwind_integration);
                mkinclude!(libunwind_integration);
                 
            }}
mkmod!{os, { 
                getname!(os);
                getsrc!(os);
                getpath!(os);
                get_deps!(os);
                get_crates!(os);
                mkinclude!(os);
                 
            }}
mkmod!{pipe, { 
                getname!(pipe);
                getsrc!(pipe);
                getpath!(pipe);
                get_deps!(pipe);
                get_crates!(pipe);
                mkinclude!(pipe);
                 
            }}
mkmod!{thread_parking, { 
                getname!(thread_parking);
                getsrc!(thread_parking);
                getpath!(thread_parking);
                get_deps!(thread_parking);
                get_crates!(thread_parking);
                mkinclude!(thread_parking);
                 
            }}
mkmod!{time, { 
                getname!(time);
                getsrc!(time);
                getpath!(time);
                get_deps!(time);
                get_crates!(time);
                mkinclude!(time);
                 
            }}
mkmod!{waitqueue, { 
                getname!(waitqueue);
                getsrc!(waitqueue);
                getpath!(waitqueue);
                get_deps!(waitqueue);
                get_crates!(waitqueue);
                mkinclude!(waitqueue);
                 
            }}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub unsafe fn init (argc : isize , argv : * const * const u8 , _sigpipe : u8) { unsafe { crate :: sys :: args :: init (argc , argv) ; } }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub unsafe fn cleanup () { }
}

macro_rules! unsupported_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsupported in module {}", module_path!());
    };
}

mkfn!{
    unsupported_introspect!();
    # [doc = " This function is used to implement functionality that simply doesn't exist."] # [doc = " Programs relying on this functionality will need to deal with the error."] pub fn unsupported < T > () -> crate :: io :: Result < T > { Err (unsupported_err ()) }
}

macro_rules! unsupported_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsupported_err in module {}", module_path!());
    };
}

mkfn!{
    unsupported_err_introspect!();
    pub fn unsupported_err () -> crate :: io :: Error { crate :: io :: const_error ! (ErrorKind :: Unsupported , "operation not supported on SGX yet") }
}

macro_rules! sgx_ineffective_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sgx_ineffective in module {}", module_path!());
    };
}

mkfn!{
    sgx_ineffective_introspect!();
    # [doc = " This function is used to implement various functions that doesn't exist,"] # [doc = " but the lack of which might not be reason for error. If no error is"] # [doc = " returned, the program might very well be able to function normally. This is"] # [doc = " what happens when `SGX_INEFFECTIVE_ERROR` is set to `true`. If it is"] # [doc = " `false`, the behavior is the same as `unsupported`."] pub fn sgx_ineffective < T > (v : T) -> crate :: io :: Result < T > { static SGX_INEFFECTIVE_ERROR : Atomic < bool > = AtomicBool :: new (false) ; if SGX_INEFFECTIVE_ERROR . load (Ordering :: Relaxed) { Err (crate :: io :: const_error ! (ErrorKind :: Uncategorized , "operation can't be trusted to have any effect on SGX" ,)) } else { Ok (v) } }
}

macro_rules! is_interrupted_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_interrupted in module {}", module_path!());
    };
}

mkfn!{
    is_interrupted_introspect!();
    # [inline] pub fn is_interrupted (code : i32) -> bool { code == fortanix_sgx_abi :: Error :: Interrupted as _ }
}

macro_rules! decode_error_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_error_kind in module {}", module_path!());
    };
}

mkfn!{
    decode_error_kind_introspect!();
    pub fn decode_error_kind (code : i32) -> ErrorKind { use fortanix_sgx_abi :: Error ; if code == Error :: NotFound as _ { ErrorKind :: NotFound } else if code == Error :: PermissionDenied as _ { ErrorKind :: PermissionDenied } else if code == Error :: ConnectionRefused as _ { ErrorKind :: ConnectionRefused } else if code == Error :: ConnectionReset as _ { ErrorKind :: ConnectionReset } else if code == Error :: ConnectionAborted as _ { ErrorKind :: ConnectionAborted } else if code == Error :: NotConnected as _ { ErrorKind :: NotConnected } else if code == Error :: AddrInUse as _ { ErrorKind :: AddrInUse } else if code == Error :: AddrNotAvailable as _ { ErrorKind :: AddrNotAvailable } else if code == Error :: BrokenPipe as _ { ErrorKind :: BrokenPipe } else if code == Error :: AlreadyExists as _ { ErrorKind :: AlreadyExists } else if code == Error :: WouldBlock as _ { ErrorKind :: WouldBlock } else if code == Error :: InvalidInput as _ { ErrorKind :: InvalidInput } else if code == Error :: InvalidData as _ { ErrorKind :: InvalidData } else if code == Error :: TimedOut as _ { ErrorKind :: TimedOut } else if code == Error :: WriteZero as _ { ErrorKind :: WriteZero } else if code == Error :: Interrupted as _ { ErrorKind :: Interrupted } else if code == Error :: Other as _ { ErrorKind :: Uncategorized } else if code == Error :: UnexpectedEof as _ { ErrorKind :: UnexpectedEof } else { ErrorKind :: Uncategorized } }
}

macro_rules! abort_internal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort_internal in module {}", module_path!());
    };
}

mkfn!{
    abort_internal_introspect!();
    pub fn abort_internal () -> ! { abi :: usercalls :: exit (true) }
}

macro_rules! __rust_abort_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_abort in module {}", module_path!());
    };
}

mkfn!{
    __rust_abort_introspect!();
    # [cfg (not (test))] # [unsafe (no_mangle)] pub extern "C" fn __rust_abort () { abort_internal () ; }
}
mkuse!{pub use crate :: sys_common :: { AsInner , FromInner , IntoInner } ;}
mkitem!{mktrait!{pub trait TryIntoInner < Inner > : Sized { fn try_into_inner (self) -> Result < Inner , Self > ; }}}