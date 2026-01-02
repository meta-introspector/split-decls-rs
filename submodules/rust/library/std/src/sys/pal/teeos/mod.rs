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
mkmod!{sync, { 
                getname!(sync);
                getsrc!(sync);
                getpath!(sync);
                get_deps!(sync);
                get_crates!(sync);
                mkinclude!(sync);
                mkmod!{condvar, { 
                getname!(condvar);
                getsrc!(condvar);
                getpath!(condvar);
                get_deps!(condvar);
                get_crates!(condvar);
                mkinclude!(condvar);
                 
            }}
mkmod!{mutex, { 
                getname!(mutex);
                getsrc!(mutex);
                getpath!(mutex);
                get_deps!(mutex);
                get_crates!(mutex);
                mkinclude!(mutex);
                 
            }}
mkuse!{pub use condvar :: Condvar ;}
mkuse!{pub use mutex :: Mutex ;} 
            }}
mkuse!{use crate :: io :: ErrorKind ;}

macro_rules! abort_internal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort_internal in module {}", module_path!());
    };
}

mkfn!{
    abort_internal_introspect!();
    pub fn abort_internal () -> ! { unsafe { libc :: abort () } }
}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub fn init (argc : isize , argv : * const * const u8 , sigpipe : u8) { }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub unsafe fn cleanup () { unimplemented ! () }
}

macro_rules! is_interrupted_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_interrupted in module {}", module_path!());
    };
}

mkfn!{
    is_interrupted_introspect!();
    # [inline] pub (crate) fn is_interrupted (errno : i32) -> bool { errno == libc :: EINTR }
}

macro_rules! decode_error_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_error_kind in module {}", module_path!());
    };
}

mkfn!{
    decode_error_kind_introspect!();
    pub fn decode_error_kind (errno : i32) -> ErrorKind { use ErrorKind :: * ; match errno as libc :: c_int { libc :: E2BIG => ArgumentListTooLong , libc :: EADDRINUSE => AddrInUse , libc :: EADDRNOTAVAIL => AddrNotAvailable , libc :: EBUSY => ResourceBusy , libc :: ECONNABORTED => ConnectionAborted , libc :: ECONNREFUSED => ConnectionRefused , libc :: ECONNRESET => ConnectionReset , libc :: EDEADLK => Deadlock , libc :: EDQUOT => QuotaExceeded , libc :: EEXIST => AlreadyExists , libc :: EFBIG => FileTooLarge , libc :: EHOSTUNREACH => HostUnreachable , libc :: EINTR => Interrupted , libc :: EINVAL => InvalidInput , libc :: EISDIR => IsADirectory , libc :: ELOOP => FilesystemLoop , libc :: ENOENT => NotFound , libc :: ENOMEM => OutOfMemory , libc :: ENOSPC => StorageFull , libc :: ENOSYS => Unsupported , libc :: EMLINK => TooManyLinks , libc :: ENAMETOOLONG => InvalidFilename , libc :: ENETDOWN => NetworkDown , libc :: ENETUNREACH => NetworkUnreachable , libc :: ENOTCONN => NotConnected , libc :: ENOTDIR => NotADirectory , libc :: ENOTEMPTY => DirectoryNotEmpty , libc :: EPIPE => BrokenPipe , libc :: EROFS => ReadOnlyFilesystem , libc :: ESPIPE => NotSeekable , libc :: ESTALE => StaleNetworkFileHandle , libc :: ETIMEDOUT => TimedOut , libc :: ETXTBSY => ExecutableFileBusy , libc :: EXDEV => CrossesDevices , libc :: EACCES | libc :: EPERM => PermissionDenied , x if x == libc :: EAGAIN || x == libc :: EWOULDBLOCK => WouldBlock , _ => Uncategorized , } }
}
mkitem!{mktrait!{# [doc (hidden)] pub trait IsMinusOne { fn is_minus_one (& self) -> bool ; }}}
mkitem!{macro_rules ! impl_is_minus_one { ($ ($ t : ident) *) => ($ (impl IsMinusOne for $ t { fn is_minus_one (& self) -> bool { * self == - 1 } }) *) }}
mkitem!{impl_is_minus_one ! { i8 i16 i32 i64 isize }}

macro_rules! cvt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt in module {}", module_path!());
    };
}

mkfn!{
    cvt_introspect!();
    pub fn cvt < T : IsMinusOne > (t : T) -> crate :: io :: Result < T > { if t . is_minus_one () { Err (crate :: io :: Error :: last_os_error ()) } else { Ok (t) } }
}

macro_rules! cvt_r_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt_r in module {}", module_path!());
    };
}

mkfn!{
    cvt_r_introspect!();
    pub fn cvt_r < T , F > (mut f : F) -> crate :: io :: Result < T > where T : IsMinusOne , F : FnMut () -> T , { loop { match cvt (f ()) { Err (ref e) if e . kind () == ErrorKind :: Interrupted => { } other => return other , } } }
}

macro_rules! cvt_nz_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt_nz in module {}", module_path!());
    };
}

mkfn!{
    cvt_nz_introspect!();
    pub fn cvt_nz (error : libc :: c_int) -> crate :: io :: Result < () > { if error == 0 { Ok (()) } else { Err (crate :: io :: Error :: from_raw_os_error (error)) } }
}
mkuse!{use crate :: io as std_io ;}

macro_rules! unsupported_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsupported in module {}", module_path!());
    };
}

mkfn!{
    unsupported_introspect!();
    pub fn unsupported < T > () -> std_io :: Result < T > { Err (unsupported_err ()) }
}

macro_rules! unsupported_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsupported_err in module {}", module_path!());
    };
}

mkfn!{
    unsupported_err_introspect!();
    pub fn unsupported_err () -> std_io :: Error { std_io :: Error :: UNSUPPORTED_PLATFORM }
}