mkuse!{use crate :: io :: ErrorKind ;}
mkuse!{use crate :: os :: hermit :: hermit_abi ;}
mkuse!{use crate :: os :: raw :: c_char ;}
mkuse!{use crate :: sys :: env ;}
mkmod!{futex, { 
                getname!(futex);
                getsrc!(futex);
                getpath!(futex);
                get_deps!(futex);
                get_crates!(futex);
                mkinclude!(futex);
                 
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
mkmod!{time, { 
                getname!(time);
                getsrc!(time);
                getpath!(time);
                get_deps!(time);
                get_crates!(time);
                mkinclude!(time);
                 
            }}

macro_rules! unsupported_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsupported in module {}", module_path!());
    };
}

mkfn!{
    unsupported_introspect!();
    pub fn unsupported < T > () -> crate :: io :: Result < T > { Err (unsupported_err ()) }
}

macro_rules! unsupported_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsupported_err in module {}", module_path!());
    };
}

mkfn!{
    unsupported_err_introspect!();
    pub fn unsupported_err () -> crate :: io :: Error { crate :: io :: const_error ! (crate :: io :: ErrorKind :: Unsupported , "operation not supported on HermitCore yet" ,) }
}

macro_rules! abort_internal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort_internal in module {}", module_path!());
    };
}

mkfn!{
    abort_internal_introspect!();
    pub fn abort_internal () -> ! { unsafe { hermit_abi :: abort () } }
}

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

macro_rules! runtime_entry_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function runtime_entry in module {}", module_path!());
    };
}

mkfn!{
    runtime_entry_introspect!();
    # [cfg (not (test))] # [unsafe (no_mangle)] pub unsafe extern "C" fn runtime_entry (argc : i32 , argv : * const * const c_char , env : * const * const c_char ,) -> ! { unsafe extern "C" { fn main (argc : isize , argv : * const * const c_char) -> i32 ; } env :: init (env) ; let result = unsafe { main (argc as isize , argv) } ; unsafe { crate :: sys :: thread_local :: destructors :: run () ; } crate :: rt :: thread_cleanup () ; unsafe { hermit_abi :: exit (result) ; } }
}

macro_rules! is_interrupted_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_interrupted in module {}", module_path!());
    };
}

mkfn!{
    is_interrupted_introspect!();
    # [inline] pub (crate) fn is_interrupted (errno : i32) -> bool { errno == hermit_abi :: errno :: EINTR }
}

macro_rules! decode_error_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_error_kind in module {}", module_path!());
    };
}

mkfn!{
    decode_error_kind_introspect!();
    pub fn decode_error_kind (errno : i32) -> ErrorKind { match errno { hermit_abi :: errno :: EACCES => ErrorKind :: PermissionDenied , hermit_abi :: errno :: EADDRINUSE => ErrorKind :: AddrInUse , hermit_abi :: errno :: EADDRNOTAVAIL => ErrorKind :: AddrNotAvailable , hermit_abi :: errno :: EAGAIN => ErrorKind :: WouldBlock , hermit_abi :: errno :: ECONNABORTED => ErrorKind :: ConnectionAborted , hermit_abi :: errno :: ECONNREFUSED => ErrorKind :: ConnectionRefused , hermit_abi :: errno :: ECONNRESET => ErrorKind :: ConnectionReset , hermit_abi :: errno :: EEXIST => ErrorKind :: AlreadyExists , hermit_abi :: errno :: EINTR => ErrorKind :: Interrupted , hermit_abi :: errno :: EINVAL => ErrorKind :: InvalidInput , hermit_abi :: errno :: ENOENT => ErrorKind :: NotFound , hermit_abi :: errno :: ENOTCONN => ErrorKind :: NotConnected , hermit_abi :: errno :: EPERM => ErrorKind :: PermissionDenied , hermit_abi :: errno :: EPIPE => ErrorKind :: BrokenPipe , hermit_abi :: errno :: ETIMEDOUT => ErrorKind :: TimedOut , _ => ErrorKind :: Uncategorized , } }
}
mkitem!{mktrait!{# [doc (hidden)] pub trait IsNegative { fn is_negative (& self) -> bool ; fn negate (& self) -> i32 ; }}}
mkitem!{macro_rules ! impl_is_negative { ($ ($ t : ident) *) => ($ (impl IsNegative for $ t { fn is_negative (& self) -> bool { * self < 0 } fn negate (& self) -> i32 { i32 :: try_from (- (* self)) . unwrap () } }) *) }}
mkitem!{mkimpl!{impl IsNegative for i32 { fn is_negative (& self) -> bool { * self < 0 } fn negate (& self) -> i32 { - (* self) } }}}
mkitem!{impl_is_negative ! { i8 i16 i64 isize }}

macro_rules! cvt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt in module {}", module_path!());
    };
}

mkfn!{
    cvt_introspect!();
    pub fn cvt < T : IsNegative > (t : T) -> crate :: io :: Result < T > { if t . is_negative () { let e = decode_error_kind (t . negate ()) ; Err (crate :: io :: Error :: from (e)) } else { Ok (t) } }
}

macro_rules! cvt_r_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt_r in module {}", module_path!());
    };
}

mkfn!{
    cvt_r_introspect!();
    pub fn cvt_r < T , F > (mut f : F) -> crate :: io :: Result < T > where T : IsNegative , F : FnMut () -> T , { loop { match cvt (f ()) { Err (ref e) if e . is_interrupted () => { } other => return other , } } }
}