mkmod!{helpers, { 
                getname!(helpers);
                getsrc!(helpers);
                getpath!(helpers);
                get_deps!(helpers);
                get_crates!(helpers);
                mkinclude!(helpers);
                 
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
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{pub type RawOsError = usize ;}
mkuse!{use crate :: io as std_io ;}
mkuse!{use crate :: os :: uefi ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicPtr , Ordering } ;}
mkitem!{static EXIT_BOOT_SERVICE_EVENT : Atomic < * mut crate :: ffi :: c_void > = AtomicPtr :: new (crate :: ptr :: null_mut ()) ;}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    # [doc = " # SAFETY"] # [doc = " - must be called only once during runtime initialization."] # [doc = " - argc must be 2."] # [doc = " - argv must be &[Handle, *mut SystemTable]."] pub (crate) unsafe fn init (argc : isize , argv : * const * const u8 , _sigpipe : u8) { assert_eq ! (argc , 2) ; let image_handle = unsafe { NonNull :: new (* argv as * mut crate :: ffi :: c_void) . unwrap () } ; let system_table = unsafe { NonNull :: new (* argv . add (1) as * mut crate :: ffi :: c_void) . unwrap () } ; unsafe { uefi :: env :: init_globals (image_handle , system_table) } ; match helpers :: OwnedEvent :: new (r_efi :: efi :: EVT_SIGNAL_EXIT_BOOT_SERVICES , r_efi :: efi :: TPL_NOTIFY , Some (exit_boot_service_handler) , None ,) { Ok (x) => { if EXIT_BOOT_SERVICE_EVENT . compare_exchange (crate :: ptr :: null_mut () , x . into_raw () , Ordering :: Release , Ordering :: Acquire ,) . is_err () { abort_internal () ; } ; } Err (_) => abort_internal () , } }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    # [doc = " # SAFETY"] # [doc = " this is not guaranteed to run, for example when the program aborts."] # [doc = " - must be called only once during runtime cleanup."] pub unsafe fn cleanup () { if let Some (exit_boot_service_event) = NonNull :: new (EXIT_BOOT_SERVICE_EVENT . swap (crate :: ptr :: null_mut () , Ordering :: Acquire)) { let _ = unsafe { helpers :: OwnedEvent :: from_raw (exit_boot_service_event . as_ptr ()) } ; } }
}

macro_rules! unsupported_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsupported in module {}", module_path!());
    };
}

mkfn!{
    unsupported_introspect!();
    # [inline] pub const fn unsupported < T > () -> std_io :: Result < T > { Err (unsupported_err ()) }
}

macro_rules! unsupported_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsupported_err in module {}", module_path!());
    };
}

mkfn!{
    unsupported_err_introspect!();
    # [inline] pub const fn unsupported_err () -> std_io :: Error { std_io :: const_error ! (std_io :: ErrorKind :: Unsupported , "operation not supported on UEFI") }
}

macro_rules! decode_error_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_error_kind in module {}", module_path!());
    };
}

mkfn!{
    decode_error_kind_introspect!();
    pub fn decode_error_kind (code : RawOsError) -> crate :: io :: ErrorKind { use r_efi :: efi :: Status ; use crate :: io :: ErrorKind ; match r_efi :: efi :: Status :: from_usize (code) { Status :: ALREADY_STARTED | Status :: COMPROMISED_DATA | Status :: CONNECTION_FIN | Status :: CRC_ERROR | Status :: DEVICE_ERROR | Status :: END_OF_MEDIA | Status :: HTTP_ERROR | Status :: ICMP_ERROR | Status :: INCOMPATIBLE_VERSION | Status :: LOAD_ERROR | Status :: MEDIA_CHANGED | Status :: NO_MAPPING | Status :: NO_MEDIA | Status :: NOT_STARTED | Status :: PROTOCOL_ERROR | Status :: PROTOCOL_UNREACHABLE | Status :: TFTP_ERROR | Status :: VOLUME_CORRUPTED => ErrorKind :: Other , Status :: BAD_BUFFER_SIZE | Status :: INVALID_LANGUAGE => ErrorKind :: InvalidData , Status :: ABORTED => ErrorKind :: ConnectionAborted , Status :: ACCESS_DENIED => ErrorKind :: PermissionDenied , Status :: BUFFER_TOO_SMALL => ErrorKind :: FileTooLarge , Status :: CONNECTION_REFUSED => ErrorKind :: ConnectionRefused , Status :: CONNECTION_RESET => ErrorKind :: ConnectionReset , Status :: END_OF_FILE => ErrorKind :: UnexpectedEof , Status :: HOST_UNREACHABLE => ErrorKind :: HostUnreachable , Status :: INVALID_PARAMETER => ErrorKind :: InvalidInput , Status :: IP_ADDRESS_CONFLICT => ErrorKind :: AddrInUse , Status :: NETWORK_UNREACHABLE => ErrorKind :: NetworkUnreachable , Status :: NO_RESPONSE => ErrorKind :: HostUnreachable , Status :: NOT_FOUND => ErrorKind :: NotFound , Status :: NOT_READY => ErrorKind :: ResourceBusy , Status :: OUT_OF_RESOURCES => ErrorKind :: OutOfMemory , Status :: SECURITY_VIOLATION => ErrorKind :: PermissionDenied , Status :: TIMEOUT => ErrorKind :: TimedOut , Status :: UNSUPPORTED => ErrorKind :: Unsupported , Status :: VOLUME_FULL => ErrorKind :: StorageFull , Status :: WRITE_PROTECTED => ErrorKind :: ReadOnlyFilesystem , _ => ErrorKind :: Uncategorized , } }
}

macro_rules! abort_internal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort_internal in module {}", module_path!());
    };
}

mkfn!{
    abort_internal_introspect!();
    pub fn abort_internal () -> ! { if let Some (exit_boot_service_event) = NonNull :: new (EXIT_BOOT_SERVICE_EVENT . load (Ordering :: Acquire)) { let _ = unsafe { helpers :: OwnedEvent :: from_raw (exit_boot_service_event . as_ptr ()) } ; } if let (Some (boot_services) , Some (handle)) = (uefi :: env :: boot_services () , uefi :: env :: try_image_handle ()) { let boot_services : NonNull < r_efi :: efi :: BootServices > = boot_services . cast () ; let _ = unsafe { ((* boot_services . as_ptr ()) . exit) (handle . as_ptr () , r_efi :: efi :: Status :: ABORTED , 0 , crate :: ptr :: null_mut () ,) } ; } core :: intrinsics :: abort () ; }
}

macro_rules! exit_boot_service_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exit_boot_service_handler in module {}", module_path!());
    };
}

mkfn!{
    exit_boot_service_handler_introspect!();
    # [doc = " Disable access to BootServices if `EVT_SIGNAL_EXIT_BOOT_SERVICES` is signaled"] extern "efiapi" fn exit_boot_service_handler (_e : r_efi :: efi :: Event , _ctx : * mut crate :: ffi :: c_void) { uefi :: env :: disable_boot_services () ; }
}

macro_rules! is_interrupted_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_interrupted in module {}", module_path!());
    };
}

mkfn!{
    is_interrupted_introspect!();
    pub fn is_interrupted (_code : RawOsError) -> bool { false }
}