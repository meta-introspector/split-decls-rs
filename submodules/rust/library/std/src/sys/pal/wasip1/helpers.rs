mkuse!{use crate :: io as std_io ;}

macro_rules! is_interrupted_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_interrupted in module {}", module_path!());
    };
}

mkfn!{
    is_interrupted_introspect!();
    # [inline] pub fn is_interrupted (errno : i32) -> bool { errno == wasi :: ERRNO_INTR . raw () . into () }
}

macro_rules! decode_error_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_error_kind in module {}", module_path!());
    };
}

mkfn!{
    decode_error_kind_introspect!();
    pub fn decode_error_kind (errno : i32) -> std_io :: ErrorKind { use std_io :: ErrorKind ; let Ok (errno) = u16 :: try_from (errno) else { return ErrorKind :: Uncategorized ; } ; macro_rules ! match_errno { ($ ($ ($ errno : ident) |+ => $ errkind : ident) ,*, _ => $ wildcard : ident $ (,) ?) => { match errno { $ (e if $ (e == :: wasi ::$ errno . raw ()) ||+ => ErrorKind ::$ errkind) ,*, _ => ErrorKind ::$ wildcard , } } ; } match_errno ! { ERRNO_2BIG => ArgumentListTooLong , ERRNO_ACCES => PermissionDenied , ERRNO_ADDRINUSE => AddrInUse , ERRNO_ADDRNOTAVAIL => AddrNotAvailable , ERRNO_AFNOSUPPORT => Unsupported , ERRNO_AGAIN => WouldBlock , ERRNO_BUSY => ResourceBusy , ERRNO_CONNABORTED => ConnectionAborted , ERRNO_CONNREFUSED => ConnectionRefused , ERRNO_CONNRESET => ConnectionReset , ERRNO_DEADLK => Deadlock , ERRNO_DOM => InvalidInput , ERRNO_EXIST => AlreadyExists , ERRNO_FBIG => FileTooLarge , ERRNO_HOSTUNREACH => HostUnreachable , ERRNO_INTR => Interrupted , ERRNO_INVAL => InvalidInput , ERRNO_IO => Uncategorized , ERRNO_ISDIR => IsADirectory , ERRNO_LOOP => FilesystemLoop , ERRNO_MLINK => TooManyLinks , ERRNO_NAMETOOLONG => InvalidFilename , ERRNO_NETDOWN => NetworkDown , ERRNO_NETUNREACH => NetworkUnreachable , ERRNO_NODEV => NotFound , ERRNO_NOENT => NotFound , ERRNO_NOMEM => OutOfMemory , ERRNO_NOSPC => StorageFull , ERRNO_NOSYS => Unsupported , ERRNO_NOTCONN => NotConnected , ERRNO_NOTDIR => NotADirectory , ERRNO_NOTEMPTY => DirectoryNotEmpty , ERRNO_NOTSUP => Unsupported , ERRNO_NXIO => NotFound , ERRNO_PERM => PermissionDenied , ERRNO_PIPE => BrokenPipe , ERRNO_PROTONOSUPPORT => Unsupported , ERRNO_ROFS => ReadOnlyFilesystem , ERRNO_SPIPE => NotSeekable , ERRNO_SRCH => NotFound , ERRNO_TIMEDOUT => TimedOut , ERRNO_TXTBSY => ResourceBusy , ERRNO_XDEV => CrossesDevices , ERRNO_NOTCAPABLE => PermissionDenied , _ => Uncategorized , } }
}

macro_rules! abort_internal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort_internal in module {}", module_path!());
    };
}

mkfn!{
    abort_internal_introspect!();
    pub fn abort_internal () -> ! { unsafe { libc :: abort () } }
}

macro_rules! err2io_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function err2io in module {}", module_path!());
    };
}

mkfn!{
    err2io_introspect!();
    # [inline] pub (crate) fn err2io (err : wasi :: Errno) -> std_io :: Error { std_io :: Error :: from_raw_os_error (err . raw () . into ()) }
}