mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: io :: ErrorKind ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: os :: windows :: ffi :: { OsStrExt , OsStringExt } ;}
mkuse!{use crate :: path :: PathBuf ;}
mkuse!{use crate :: sys :: pal :: windows :: api :: wide_str ;}
mkuse!{use crate :: time :: Duration ;}
mkmod!{compat, { 
                getname!(compat);
                getsrc!(compat);
                getpath!(compat);
                get_deps!(compat);
                get_crates!(compat);
                mkinclude!(compat);
                 
            }}
mkmod!{api, { 
                getname!(api);
                getsrc!(api);
                getpath!(api);
                get_deps!(api);
                get_crates!(api);
                mkinclude!(api);
                 
            }}
mkmod!{c, { 
                getname!(c);
                getsrc!(c);
                getpath!(c);
                get_deps!(c);
                get_crates!(c);
                mkinclude!(c);
                 
            }}
mkmod!{futex, { 
                getname!(futex);
                getsrc!(futex);
                getpath!(futex);
                get_deps!(futex);
                get_crates!(futex);
                mkinclude!(futex);
                 
            }}
mkmod!{handle, { 
                getname!(handle);
                getsrc!(handle);
                getpath!(handle);
                get_deps!(handle);
                get_crates!(handle);
                mkinclude!(handle);
                 
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
mkitem!{cfg_select ! { not (target_vendor = "uwp") => { pub mod stack_overflow ; } _ => { pub mod stack_overflow_uwp ; pub use self :: stack_overflow_uwp as stack_overflow ; } }}
mkitem!{mktrait!{# [doc = " Map a [`Result<T, WinError>`] to [`io::Result<T>`](crate::io::Result<T>)."] pub trait IoResult < T > { fn io_result (self) -> crate :: io :: Result < T > ; }}}
mkitem!{mkimpl!{impl < T > IoResult < T > for Result < T , api :: WinError > { fn io_result (self) -> crate :: io :: Result < T > { self . map_err (| e | crate :: io :: Error :: from_raw_os_error (e . code as i32)) } }}}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub unsafe fn init (_argc : isize , _argv : * const * const u8 , _sigpipe : u8) { unsafe { stack_overflow :: init () ; crate :: sys :: thread :: set_name_wide (wide_str ! ("main")) ; } }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub unsafe fn cleanup () { crate :: sys :: net :: cleanup () ; }
}

macro_rules! is_interrupted_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_interrupted in module {}", module_path!());
    };
}

mkfn!{
    is_interrupted_introspect!();
    # [inline] pub fn is_interrupted (_errno : i32) -> bool { false }
}

macro_rules! decode_error_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_error_kind in module {}", module_path!());
    };
}

mkfn!{
    decode_error_kind_introspect!();
    pub fn decode_error_kind (errno : i32) -> ErrorKind { use ErrorKind :: * ; match errno as u32 { c :: ERROR_ACCESS_DENIED => return PermissionDenied , c :: ERROR_ALREADY_EXISTS => return AlreadyExists , c :: ERROR_FILE_EXISTS => return AlreadyExists , c :: ERROR_BROKEN_PIPE => return BrokenPipe , c :: ERROR_FILE_NOT_FOUND | c :: ERROR_PATH_NOT_FOUND | c :: ERROR_INVALID_DRIVE | c :: ERROR_BAD_NETPATH | c :: ERROR_BAD_NET_NAME => return NotFound , c :: ERROR_NO_DATA => return BrokenPipe , c :: ERROR_INVALID_NAME | c :: ERROR_BAD_PATHNAME => return InvalidFilename , c :: ERROR_INVALID_PARAMETER => return InvalidInput , c :: ERROR_NOT_ENOUGH_MEMORY | c :: ERROR_OUTOFMEMORY => return OutOfMemory , c :: ERROR_SEM_TIMEOUT | c :: WAIT_TIMEOUT | c :: ERROR_DRIVER_CANCEL_TIMEOUT | c :: ERROR_OPERATION_ABORTED | c :: ERROR_SERVICE_REQUEST_TIMEOUT | c :: ERROR_COUNTER_TIMEOUT | c :: ERROR_TIMEOUT | c :: ERROR_RESOURCE_CALL_TIMED_OUT | c :: ERROR_CTX_MODEM_RESPONSE_TIMEOUT | c :: ERROR_CTX_CLIENT_QUERY_TIMEOUT | c :: FRS_ERR_SYSVOL_POPULATE_TIMEOUT | c :: ERROR_DS_TIMELIMIT_EXCEEDED | c :: DNS_ERROR_RECORD_TIMED_OUT | c :: ERROR_IPSEC_IKE_TIMED_OUT | c :: ERROR_RUNLEVEL_SWITCH_TIMEOUT | c :: ERROR_RUNLEVEL_SWITCH_AGENT_TIMEOUT => return TimedOut , c :: ERROR_CALL_NOT_IMPLEMENTED => return Unsupported , c :: ERROR_HOST_UNREACHABLE => return HostUnreachable , c :: ERROR_NETWORK_UNREACHABLE => return NetworkUnreachable , c :: ERROR_DIRECTORY => return NotADirectory , c :: ERROR_DIRECTORY_NOT_SUPPORTED => return IsADirectory , c :: ERROR_DIR_NOT_EMPTY => return DirectoryNotEmpty , c :: ERROR_WRITE_PROTECT => return ReadOnlyFilesystem , c :: ERROR_DISK_FULL | c :: ERROR_HANDLE_DISK_FULL => return StorageFull , c :: ERROR_SEEK_ON_DEVICE => return NotSeekable , c :: ERROR_DISK_QUOTA_EXCEEDED => return QuotaExceeded , c :: ERROR_FILE_TOO_LARGE => return FileTooLarge , c :: ERROR_BUSY => return ResourceBusy , c :: ERROR_POSSIBLE_DEADLOCK => return Deadlock , c :: ERROR_NOT_SAME_DEVICE => return CrossesDevices , c :: ERROR_TOO_MANY_LINKS => return TooManyLinks , c :: ERROR_FILENAME_EXCED_RANGE => return InvalidFilename , c :: ERROR_CANT_RESOLVE_FILENAME => return FilesystemLoop , _ => { } } match errno { c :: WSAEACCES => PermissionDenied , c :: WSAEADDRINUSE => AddrInUse , c :: WSAEADDRNOTAVAIL => AddrNotAvailable , c :: WSAECONNABORTED => ConnectionAborted , c :: WSAECONNREFUSED => ConnectionRefused , c :: WSAECONNRESET => ConnectionReset , c :: WSAEINVAL => InvalidInput , c :: WSAENOTCONN => NotConnected , c :: WSAEWOULDBLOCK => WouldBlock , c :: WSAETIMEDOUT => TimedOut , c :: WSAEHOSTUNREACH => HostUnreachable , c :: WSAENETDOWN => NetworkDown , c :: WSAENETUNREACH => NetworkUnreachable , c :: WSAEDQUOT => QuotaExceeded , _ => Uncategorized , } }
}

macro_rules! unrolled_find_u16s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unrolled_find_u16s in module {}", module_path!());
    };
}

mkfn!{
    unrolled_find_u16s_introspect!();
    pub fn unrolled_find_u16s (needle : u16 , haystack : & [u16]) -> Option < usize > { let ptr = haystack . as_ptr () ; let mut start = haystack ; while start . len () >= 8 { macro_rules ! if_return { ($ ($ n : literal ,) +) => { $ (if start [$ n] == needle { return Some (((& start [$ n] as * const u16) . addr () - ptr . addr ()) / 2) ; }) + } } if_return ! (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 ,) ; start = & start [8 ..] ; } for c in start { if * c == needle { return Some (((c as * const u16) . addr () - ptr . addr ()) / 2) ; } } None }
}

macro_rules! to_u16s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_u16s in module {}", module_path!());
    };
}

mkfn!{
    to_u16s_introspect!();
    pub fn to_u16s < S : AsRef < OsStr > > (s : S) -> crate :: io :: Result < Vec < u16 > > { fn inner (s : & OsStr) -> crate :: io :: Result < Vec < u16 > > { let mut maybe_result = Vec :: with_capacity (s . len () + 1) ; maybe_result . extend (s . encode_wide ()) ; if unrolled_find_u16s (0 , & maybe_result) . is_some () { return Err (crate :: io :: const_error ! (ErrorKind :: InvalidInput , "strings passed to WinAPI cannot contain NULs" ,)) ; } maybe_result . push (0) ; Ok (maybe_result) } inner (s . as_ref ()) }
}

macro_rules! fill_utf16_buf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_utf16_buf in module {}", module_path!());
    };
}

mkfn!{
    fill_utf16_buf_introspect!();
    pub fn fill_utf16_buf < F1 , F2 , T > (mut f1 : F1 , f2 : F2) -> crate :: io :: Result < T > where F1 : FnMut (* mut u16 , u32) -> u32 , F2 : FnOnce (& [u16]) -> T , { let mut stack_buf : [MaybeUninit < u16 > ; 512] = [MaybeUninit :: uninit () ; 512] ; let mut heap_buf : Vec < MaybeUninit < u16 > > = Vec :: new () ; unsafe { let mut n = stack_buf . len () ; loop { let buf = if n <= stack_buf . len () { & mut stack_buf [..] } else { let extra = n - heap_buf . len () ; heap_buf . reserve (extra) ; n = heap_buf . capacity () . min (u32 :: MAX as usize) ; heap_buf . set_len (n) ; & mut heap_buf [..] } ; c :: SetLastError (0) ; let k = match f1 (buf . as_mut_ptr () . cast :: < u16 > () , n as u32) { 0 if api :: get_last_error () . code == 0 => 0 , 0 => return Err (crate :: io :: Error :: last_os_error ()) , n => n , } as usize ; if k == n && api :: get_last_error () . code == c :: ERROR_INSUFFICIENT_BUFFER { n = n . saturating_mul (2) . min (u32 :: MAX as usize) ; } else if k > n { n = k ; } else if k == n { unreachable ! () ; } else { let slice : & [u16] = buf [.. k] . assume_init_ref () ; return Ok (f2 (slice)) ; } } } }
}

macro_rules! os2path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function os2path in module {}", module_path!());
    };
}

mkfn!{
    os2path_introspect!();
    pub fn os2path (s : & [u16]) -> PathBuf { PathBuf :: from (OsString :: from_wide (s)) }
}

macro_rules! truncate_utf16_at_nul_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function truncate_utf16_at_nul in module {}", module_path!());
    };
}

mkfn!{
    truncate_utf16_at_nul_introspect!();
    pub fn truncate_utf16_at_nul (v : & [u16]) -> & [u16] { match unrolled_find_u16s (0 , v) { Some (i) => & v [.. i] , None => v , } }
}

macro_rules! ensure_no_nuls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ensure_no_nuls in module {}", module_path!());
    };
}

mkfn!{
    ensure_no_nuls_introspect!();
    pub fn ensure_no_nuls < T : AsRef < OsStr > > (s : T) -> crate :: io :: Result < T > { if s . as_ref () . encode_wide () . any (| b | b == 0) { Err (crate :: io :: const_error ! (ErrorKind :: InvalidInput , "nul byte found in provided data")) } else { Ok (s) } }
}
mkitem!{mktrait!{pub trait IsZero { fn is_zero (& self) -> bool ; }}}
mkitem!{macro_rules ! impl_is_zero { ($ ($ t : ident) *) => ($ (impl IsZero for $ t { fn is_zero (& self) -> bool { * self == 0 } }) *) }}
mkitem!{impl_is_zero ! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }}

macro_rules! cvt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cvt in module {}", module_path!());
    };
}

mkfn!{
    cvt_introspect!();
    pub fn cvt < I : IsZero > (i : I) -> crate :: io :: Result < I > { if i . is_zero () { Err (crate :: io :: Error :: last_os_error ()) } else { Ok (i) } }
}

macro_rules! dur2timeout_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dur2timeout in module {}", module_path!());
    };
}

mkfn!{
    dur2timeout_introspect!();
    pub fn dur2timeout (dur : Duration) -> u32 { dur . as_secs () . checked_mul (1000) . and_then (| ms | ms . checked_add ((dur . subsec_nanos () as u64) / 1_000_000)) . and_then (| ms | ms . checked_add (if dur . subsec_nanos () % 1_000_000 > 0 { 1 } else { 0 })) . map (| ms | if ms > < u32 > :: MAX as u64 { c :: INFINITE } else { ms as u32 }) . unwrap_or (c :: INFINITE) }
}

macro_rules! abort_internal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort_internal in module {}", module_path!());
    };
}

mkfn!{
    abort_internal_introspect!();
    # [doc = " Use `__fastfail` to abort the process"] # [doc = ""] # [doc = " In Windows 8 and later, this will terminate the process immediately without"] # [doc = " running any in-process exception handlers. In earlier versions of Windows,"] # [doc = " this sequence of instructions will be treated as an access violation, which"] # [doc = " will still terminate the process but might run some exception handlers."] # [doc = ""] # [doc = " https://docs.microsoft.com/en-us/cpp/intrinsics/fastfail"] # [cfg (not (miri))] pub fn abort_internal () -> ! { unsafe { cfg_select ! { any (target_arch = "x86" , target_arch = "x86_64") => { core :: arch :: asm ! ("int $$0x29" , in ("ecx") c :: FAST_FAIL_FATAL_APP_EXIT , options (noreturn , nostack)) ; } all (target_arch = "arm" , target_feature = "thumb-mode") => { core :: arch :: asm ! (".inst 0xDEFB" , in ("r0") c :: FAST_FAIL_FATAL_APP_EXIT , options (noreturn , nostack)) ; } any (target_arch = "aarch64" , target_arch = "arm64ec") => { core :: arch :: asm ! ("brk 0xF003" , in ("x0") c :: FAST_FAIL_FATAL_APP_EXIT , options (noreturn , nostack)) ; } _ => { core :: intrinsics :: abort () ; } } } }
}

macro_rules! abort_internal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort_internal in module {}", module_path!());
    };
}

mkfn!{
    abort_internal_introspect!();
    # [cfg (miri)] # [track_caller] pub fn abort_internal () -> ! { crate :: intrinsics :: abort () ; }
}
mkitem!{mkstruct!{# [doc = " Align the inner value to 8 bytes."] # [doc = ""] # [doc = " This is enough for almost all of the buffers we're likely to work with in"] # [doc = " the Windows APIs we use."] # [repr (C , align (8))] # [derive (Copy , Clone)] pub (crate) struct Align8 < T : ? Sized > (pub T) ;}}