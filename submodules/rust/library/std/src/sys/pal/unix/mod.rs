mkuse!{use crate :: io :: ErrorKind ;}
mkmod!{weak, { 
                getname!(weak);
                getsrc!(weak);
                getpath!(weak);
                get_deps!(weak);
                get_crates!(weak);
                mkinclude!(weak);
                 
            }}
mkmod!{fuchsia, { 
                getname!(fuchsia);
                getsrc!(fuchsia);
                getpath!(fuchsia);
                get_deps!(fuchsia);
                get_crates!(fuchsia);
                mkinclude!(fuchsia);
                 
            }}
mkmod!{futex, { 
                getname!(futex);
                getsrc!(futex);
                getpath!(futex);
                get_deps!(futex);
                get_crates!(futex);
                mkinclude!(futex);
                 
            }}
mkmod!{kernel_copy, { 
                getname!(kernel_copy);
                getsrc!(kernel_copy);
                getpath!(kernel_copy);
                get_deps!(kernel_copy);
                get_crates!(kernel_copy);
                mkinclude!(kernel_copy);
                 
            }}
mkmod!{linux, { 
                getname!(linux);
                getsrc!(linux);
                getpath!(linux);
                get_deps!(linux);
                get_crates!(linux);
                mkinclude!(linux);
                 
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
mkmod!{stack_overflow, { 
                getname!(stack_overflow);
                getsrc!(stack_overflow);
                getpath!(stack_overflow);
                get_deps!(stack_overflow);
                get_crates!(stack_overflow);
                mkinclude!(stack_overflow);
                 
            }}
mkmod!{sync, { 
                getname!(sync);
                getsrc!(sync);
                getpath!(sync);
                get_deps!(sync);
                get_crates!(sync);
                mkinclude!(sync);
                 
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

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    # [cfg (target_os = "espidf")] pub fn init (_argc : isize , _argv : * const * const u8 , _sigpipe : u8) { }
}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    # [cfg (not (target_os = "espidf"))] # [cfg_attr (target_os = "vita" , allow (unused_variables))] pub unsafe fn init (argc : isize , argv : * const * const u8 , sigpipe : u8) { sanitize_standard_fds () ; reset_sigpipe (sigpipe) ; stack_overflow :: init () ; # [cfg (not (target_os = "vita"))] crate :: sys :: args :: init (argc , argv) ; if cfg ! (target_vendor = "apple") { crate :: sys :: thread :: set_name (c"main") ; } unsafe fn sanitize_standard_fds () { # [allow (dead_code , unused_variables , unused_mut)] let mut opened_devnull = - 1 ; # [allow (dead_code , unused_variables , unused_mut)] let mut open_devnull = | | { # [cfg (not (all (target_os = "linux" , target_env = "gnu")))] use libc :: open ; # [cfg (all (target_os = "linux" , target_env = "gnu"))] use libc :: open64 as open ; if opened_devnull != - 1 { if libc :: dup (opened_devnull) != - 1 { return ; } } opened_devnull = open (c"/dev/null" . as_ptr () , libc :: O_RDWR , 0) ; if opened_devnull == - 1 { libc :: abort () ; } } ; # [cfg (not (any (miri , target_os = "emscripten" , target_os = "fuchsia" , target_os = "vxworks" , target_os = "redox" , target_os = "l4re" , target_os = "horizon" , target_os = "vita" , target_os = "rtems" , target_vendor = "apple" ,)))] 'poll : { use crate :: sys :: os :: errno ; let pfds : & mut [_] = & mut [libc :: pollfd { fd : 0 , events : 0 , revents : 0 } , libc :: pollfd { fd : 1 , events : 0 , revents : 0 } , libc :: pollfd { fd : 2 , events : 0 , revents : 0 } ,] ; while libc :: poll (pfds . as_mut_ptr () , 3 , 0) == - 1 { match errno () { libc :: EINTR => continue , # [cfg (target_vendor = "unikraft")] libc :: ENOSYS => { break 'poll ; } libc :: EINVAL | libc :: EAGAIN | libc :: ENOMEM => { break 'poll ; } _ => libc :: abort () , } } for pfd in pfds { if pfd . revents & libc :: POLLNVAL == 0 { continue ; } open_devnull () ; } return ; } # [cfg (not (any (miri , target_os = "emscripten" , target_os = "fuchsia" , target_os = "vxworks" , target_os = "l4re" , target_os = "horizon" , target_os = "vita" ,)))] { use crate :: sys :: os :: errno ; for fd in 0 .. 3 { if libc :: fcntl (fd , libc :: F_GETFD) == - 1 && errno () == libc :: EBADF { open_devnull () ; } } } } unsafe fn reset_sigpipe (# [allow (unused_variables)] sigpipe : u8) { # [cfg (not (any (target_os = "emscripten" , target_os = "fuchsia" , target_os = "horizon" , target_os = "vxworks" , target_os = "vita" , target_vendor = "unikraft" ,)))] { mod sigpipe { pub const DEFAULT : u8 = 0 ; pub const INHERIT : u8 = 1 ; pub const SIG_IGN : u8 = 2 ; pub const SIG_DFL : u8 = 3 ; } let (sigpipe_attr_specified , handler) = match sigpipe { sigpipe :: DEFAULT => (false , Some (libc :: SIG_IGN)) , sigpipe :: INHERIT => (true , None) , sigpipe :: SIG_IGN => (true , Some (libc :: SIG_IGN)) , sigpipe :: SIG_DFL => (true , Some (libc :: SIG_DFL)) , _ => unreachable ! () , } ; if sigpipe_attr_specified { ON_BROKEN_PIPE_FLAG_USED . store (true , crate :: sync :: atomic :: Ordering :: Relaxed) ; } if let Some (handler) = handler { rtassert ! (signal (libc :: SIGPIPE , handler) != libc :: SIG_ERR) ; # [cfg (target_os = "hurd")] { rtassert ! (signal (libc :: SIGLOST , handler) != libc :: SIG_ERR) ; } } } } }
}
mkitem!{# [cfg (not (any (target_os = "espidf" , target_os = "emscripten" , target_os = "fuchsia" , target_os = "horizon" , target_os = "vxworks" , target_os = "vita" ,)))] static ON_BROKEN_PIPE_FLAG_USED : crate :: sync :: atomic :: Atomic < bool > = crate :: sync :: atomic :: AtomicBool :: new (false) ;}

macro_rules! on_broken_pipe_flag_used_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function on_broken_pipe_flag_used in module {}", module_path!());
    };
}

mkfn!{
    on_broken_pipe_flag_used_introspect!();
    # [cfg (not (any (target_os = "espidf" , target_os = "emscripten" , target_os = "fuchsia" , target_os = "horizon" , target_os = "vxworks" , target_os = "vita" , target_os = "nuttx" ,)))] pub (crate) fn on_broken_pipe_flag_used () -> bool { ON_BROKEN_PIPE_FLAG_USED . load (crate :: sync :: atomic :: Ordering :: Relaxed) }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub unsafe fn cleanup () { stack_overflow :: cleanup () ; }
}
mkuse!{# [allow (unused_imports)] pub use libc :: signal ;}

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
    pub fn decode_error_kind (errno : i32) -> ErrorKind { use ErrorKind :: * ; match errno as libc :: c_int { libc :: E2BIG => ArgumentListTooLong , libc :: EADDRINUSE => AddrInUse , libc :: EADDRNOTAVAIL => AddrNotAvailable , libc :: EBUSY => ResourceBusy , libc :: ECONNABORTED => ConnectionAborted , libc :: ECONNREFUSED => ConnectionRefused , libc :: ECONNRESET => ConnectionReset , libc :: EDEADLK => Deadlock , libc :: EDQUOT => QuotaExceeded , libc :: EEXIST => AlreadyExists , libc :: EFBIG => FileTooLarge , libc :: EHOSTUNREACH => HostUnreachable , libc :: EINTR => Interrupted , libc :: EINVAL => InvalidInput , libc :: EISDIR => IsADirectory , libc :: ELOOP => FilesystemLoop , libc :: ENOENT => NotFound , libc :: ENOMEM => OutOfMemory , libc :: ENOSPC => StorageFull , libc :: ENOSYS => Unsupported , libc :: EMLINK => TooManyLinks , libc :: ENAMETOOLONG => InvalidFilename , libc :: ENETDOWN => NetworkDown , libc :: ENETUNREACH => NetworkUnreachable , libc :: ENOTCONN => NotConnected , libc :: ENOTDIR => NotADirectory , # [cfg (not (target_os = "aix"))] libc :: ENOTEMPTY => DirectoryNotEmpty , libc :: EPIPE => BrokenPipe , libc :: EROFS => ReadOnlyFilesystem , libc :: ESPIPE => NotSeekable , libc :: ESTALE => StaleNetworkFileHandle , libc :: ETIMEDOUT => TimedOut , libc :: ETXTBSY => ExecutableFileBusy , libc :: EXDEV => CrossesDevices , libc :: EINPROGRESS => InProgress , libc :: EOPNOTSUPP => Unsupported , libc :: EACCES | libc :: EPERM => PermissionDenied , x if x == libc :: EAGAIN || x == libc :: EWOULDBLOCK => WouldBlock , _ => Uncategorized , } }
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
    # [doc = " Converts native return values to Result using the *-1 means error is in `errno`*  convention."] # [doc = " Non-error values are `Ok`-wrapped."] pub fn cvt < T : IsMinusOne > (t : T) -> crate :: io :: Result < T > { if t . is_minus_one () { Err (crate :: io :: Error :: last_os_error ()) } else { Ok (t) } }
}

macro_rules! cvt_r_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt_r in module {}", module_path!());
    };
}

mkfn!{
    cvt_r_introspect!();
    # [doc = " `-1` → look at `errno` → retry on `EINTR`. Otherwise `Ok()`-wrap the closure return value."] pub fn cvt_r < T , F > (mut f : F) -> crate :: io :: Result < T > where T : IsMinusOne , F : FnMut () -> T , { loop { match cvt (f ()) { Err (ref e) if e . is_interrupted () => { } other => return other , } } }
}

macro_rules! cvt_nz_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt_nz in module {}", module_path!());
    };
}

mkfn!{
    cvt_nz_introspect!();
    # [allow (dead_code)] # [doc = " Zero means `Ok()`, all other values are treated as raw OS errors. Does not look at `errno`."] pub fn cvt_nz (error : libc :: c_int) -> crate :: io :: Result < () > { if error == 0 { Ok (()) } else { Err (crate :: io :: Error :: from_raw_os_error (error)) } }
}

macro_rules! abort_internal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort_internal in module {}", module_path!());
    };
}

mkfn!{
    abort_internal_introspect!();
    # [cfg_attr (miri , track_caller)] pub fn abort_internal () -> ! { unsafe { libc :: abort () } }
}
mkitem!{cfg_select ! { target_os = "android" => { # [link (name = "dl" , kind = "static" , modifiers = "-bundle" , cfg (target_feature = "crt-static"))] # [link (name = "dl" , cfg (not (target_feature = "crt-static")))] # [link (name = "log" , cfg (not (target_feature = "crt-static")))] unsafe extern "C" { } } target_os = "freebsd" => { # [link (name = "execinfo")] # [link (name = "pthread")] unsafe extern "C" { } } target_os = "netbsd" => { # [link (name = "execinfo")] # [link (name = "pthread")] # [link (name = "rt")] unsafe extern "C" { } } any (target_os = "dragonfly" , target_os = "openbsd" , target_os = "cygwin") => { # [link (name = "pthread")] unsafe extern "C" { } } target_os = "solaris" => { # [link (name = "socket")] # [link (name = "posix4")] # [link (name = "pthread")] # [link (name = "resolv")] unsafe extern "C" { } } target_os = "illumos" => { # [link (name = "socket")] # [link (name = "posix4")] # [link (name = "pthread")] # [link (name = "resolv")] # [link (name = "nsl")] # [link (name = "umem")] unsafe extern "C" { } } target_vendor = "apple" => { # [link (name = "System")] unsafe extern "C" { } } target_os = "fuchsia" => { # [link (name = "zircon")] # [link (name = "fdio")] unsafe extern "C" { } } all (target_os = "linux" , target_env = "uclibc") => { # [link (name = "dl")] unsafe extern "C" { } } target_os = "vita" => { # [link (name = "pthread" , kind = "static" , modifiers = "-bundle")] unsafe extern "C" { } } _ => { } }}
mkmod!{unsupported, { 
                getname!(unsupported);
                getsrc!(unsupported);
                getpath!(unsupported);
                get_deps!(unsupported);
                get_crates!(unsupported);
                mkinclude!(unsupported);
                mkuse!{use crate :: io ;}

macro_rules! unsupported_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsupported in module {}", module_path!());
    };
}

mkfn!{
    unsupported_introspect!();
    pub fn unsupported < T > () -> io :: Result < T > { Err (unsupported_err ()) }
}

macro_rules! unsupported_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsupported_err in module {}", module_path!());
    };
}

mkfn!{
    unsupported_err_introspect!();
    pub fn unsupported_err () -> io :: Error { io :: Error :: UNSUPPORTED_PLATFORM }
} 
            }}