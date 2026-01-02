mkuse!{use core :: ffi :: { CStr , c_uint , c_ulong , c_ushort , c_void } ;}
mkuse!{use core :: ptr ;}
mkmod!{windows_sys, { 
                getname!(windows_sys);
                getsrc!(windows_sys);
                getpath!(windows_sys);
                get_deps!(windows_sys);
                get_crates!(windows_sys);
                mkinclude!(windows_sys);
                 
            }}
mkuse!{pub use windows_sys :: * ;}
mkitem!{pub type WCHAR = u16 ;}
mkitem!{pub const INVALID_HANDLE_VALUE : HANDLE = :: core :: ptr :: without_provenance_mut (- 1i32 as _) ;}
mkitem!{pub const EXIT_SUCCESS : u32 = 0 ;}
mkitem!{pub const EXIT_FAILURE : u32 = 1 ;}
mkitem!{# [cfg (target_vendor = "win7")] pub const CONDITION_VARIABLE_INIT : CONDITION_VARIABLE = CONDITION_VARIABLE { Ptr : ptr :: null_mut () } ;}
mkitem!{# [cfg (target_vendor = "win7")] pub const SRWLOCK_INIT : SRWLOCK = SRWLOCK { Ptr : ptr :: null_mut () } ;}
mkitem!{# [cfg (not (target_thread_local))] pub const INIT_ONCE_STATIC_INIT : INIT_ONCE = INIT_ONCE { Ptr : ptr :: null_mut () } ;}
mkitem!{pub const OBJ_DONT_REPARSE : u32 = windows_sys :: OBJ_DONT_REPARSE as u32 ;}
mkitem!{pub const FRS_ERR_SYSVOL_POPULATE_TIMEOUT : u32 = windows_sys :: FRS_ERR_SYSVOL_POPULATE_TIMEOUT as u32 ;}

macro_rules! nt_success_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nt_success in module {}", module_path!());
    };
}

mkfn!{
    nt_success_introspect!();
    pub fn nt_success (status : NTSTATUS) -> bool { status >= 0 }
}
mkitem!{mkimpl!{impl OBJECT_ATTRIBUTES { pub fn with_length () -> Self { Self { Length : size_of :: < Self > () as _ , RootDirectory : ptr :: null_mut () , ObjectName : ptr :: null_mut () , Attributes : 0 , SecurityDescriptor : ptr :: null_mut () , SecurityQualityOfService : ptr :: null_mut () , } } }}}
mkitem!{mkimpl!{impl IO_STATUS_BLOCK { pub const PENDING : Self = IO_STATUS_BLOCK { Anonymous : IO_STATUS_BLOCK_0 { Status : STATUS_PENDING } , Information : 0 } ; pub fn status (& self) -> NTSTATUS { unsafe { self . Anonymous . Status } } }}}
mkitem!{mkstruct!{# [doc = " NB: Use carefully! In general using this as a reference is likely to get the"] # [doc = " provenance wrong for the `rest` field!"] # [repr (C)] pub struct REPARSE_DATA_BUFFER { pub ReparseTag : c_uint , pub ReparseDataLength : c_ushort , pub Reserved : c_ushort , pub rest : () , }}}
mkitem!{mkstruct!{# [doc = " NB: Use carefully! In general using this as a reference is likely to get the"] # [doc = " provenance wrong for the `PathBuffer` field!"] # [repr (C)] pub struct SYMBOLIC_LINK_REPARSE_BUFFER { pub SubstituteNameOffset : c_ushort , pub SubstituteNameLength : c_ushort , pub PrintNameOffset : c_ushort , pub PrintNameLength : c_ushort , pub Flags : c_ulong , pub PathBuffer : WCHAR , }}}
mkitem!{mkstruct!{# [repr (C)] pub struct MOUNT_POINT_REPARSE_BUFFER { pub SubstituteNameOffset : c_ushort , pub SubstituteNameLength : c_ushort , pub PrintNameOffset : c_ushort , pub PrintNameLength : c_ushort , pub PathBuffer : WCHAR , }}}
mkitem!{# [cfg (not (target_vendor = "uwp"))] pub const EXCEPTION_CONTINUE_SEARCH : i32 = 0 ;}
mkitem!{# [cfg (not (target_vendor = "win7"))] # [cfg_attr (target_arch = "x86" , link (name = "bcryptprimitives" , kind = "raw-dylib" , import_name_type = "undecorated"))] # [cfg_attr (not (target_arch = "x86") , link (name = "bcryptprimitives" , kind = "raw-dylib"))] unsafe extern "system" { pub fn ProcessPrng (pbdata : * mut u8 , cbdata : usize) -> BOOL ; }}
mkitem!{windows_targets :: link ! ("ntdll.dll" "system" fn NtCreateNamedPipeFile (filehandle : * mut HANDLE , desiredaccess : FILE_ACCESS_RIGHTS , objectattributes : * const OBJECT_ATTRIBUTES , iostatusblock : * mut IO_STATUS_BLOCK , shareaccess : FILE_SHARE_MODE , createdisposition : NTCREATEFILE_CREATE_DISPOSITION , createoptions : NTCREATEFILE_CREATE_OPTIONS , namedpipetype : u32 , readmode : u32 , completionmode : u32 , maximuminstances : u32 , inboundquota : u32 , outboundquota : u32 , defaulttimeout : * const u64 ,) -> NTSTATUS) ;}
mkitem!{compat_fn_with_fallback ! { pub static KERNEL32 : & CStr = c"kernel32" ; pub fn SetThreadDescription (hthread : HANDLE , lpthreaddescription : PCWSTR) -> HRESULT { unsafe { SetLastError (ERROR_CALL_NOT_IMPLEMENTED as u32) ; E_NOTIMPL } } pub fn GetThreadDescription (hthread : HANDLE , lpthreaddescription : * mut PWSTR) -> HRESULT { unsafe { SetLastError (ERROR_CALL_NOT_IMPLEMENTED as u32) ; E_NOTIMPL } } # [cfg (target_vendor = "win7")] pub fn GetSystemTimePreciseAsFileTime (lpsystemtimeasfiletime : * mut FILETIME) -> () { unsafe { GetSystemTimeAsFileTime (lpsystemtimeasfiletime) } } pub fn GetTempPath2W (bufferlength : u32 , buffer : PWSTR) -> u32 { unsafe { GetTempPathW (bufferlength , buffer) } } }}
mkitem!{# [cfg (not (target_vendor = "win7"))] # [cfg_attr (target_arch = "x86" , link (name = "api-ms-win-core-synch-l1-2-0" , kind = "raw-dylib" , import_name_type = "undecorated"))] # [cfg_attr (not (target_arch = "x86") , link (name = "api-ms-win-core-synch-l1-2-0" , kind = "raw-dylib"))] unsafe extern "system" { pub fn WaitOnAddress (address : * const c_void , compareaddress : * const c_void , addresssize : usize , dwmilliseconds : u32 ,) -> BOOL ; pub fn WakeByAddressSingle (address : * const c_void) ; pub fn WakeByAddressAll (address : * const c_void) ; }}
mkitem!{# [cfg (target_vendor = "win7")] compat_fn_optional ! { pub fn WaitOnAddress (address : * const c_void , compareaddress : * const c_void , addresssize : usize , dwmilliseconds : u32) -> BOOL ; pub fn WakeByAddressSingle (address : * const c_void) ; }}
mkitem!{# [cfg (any (target_vendor = "win7"))] compat_fn_with_fallback ! { pub static NTDLL : & CStr = c"ntdll" ; # [cfg (target_vendor = "win7")] pub fn NtCreateKeyedEvent (KeyedEventHandle : * mut HANDLE , DesiredAccess : u32 , ObjectAttributes : * mut c_void , Flags : u32) -> NTSTATUS { panic ! ("keyed events not available") } # [cfg (target_vendor = "win7")] pub fn NtReleaseKeyedEvent (EventHandle : HANDLE , Key : * const c_void , Alertable : bool , Timeout : * mut i64) -> NTSTATUS { panic ! ("keyed events not available") } # [cfg (target_vendor = "win7")] pub fn NtWaitForKeyedEvent (EventHandle : HANDLE , Key : * const c_void , Alertable : bool , Timeout : * mut i64) -> NTSTATUS { panic ! ("keyed events not available") } }}
mkitem!{cfg_select ! { target_vendor = "uwp" => { windows_targets :: link_raw_dylib ! ("ntdll.dll" "system" fn NtCreateFile (filehandle : * mut HANDLE , desiredaccess : FILE_ACCESS_RIGHTS , objectattributes : * const OBJECT_ATTRIBUTES , iostatusblock : * mut IO_STATUS_BLOCK , allocationsize : * const i64 , fileattributes : FILE_FLAGS_AND_ATTRIBUTES , shareaccess : FILE_SHARE_MODE , createdisposition : NTCREATEFILE_CREATE_DISPOSITION , createoptions : NTCREATEFILE_CREATE_OPTIONS , eabuffer : * const core :: ffi :: c_void , ealength : u32) -> NTSTATUS) ; windows_targets :: link_raw_dylib ! ("ntdll.dll" "system" fn NtOpenFile (filehandle : * mut HANDLE , desiredaccess : u32 , objectattributes : * const OBJECT_ATTRIBUTES , iostatusblock : * mut IO_STATUS_BLOCK , shareaccess : u32 , openoptions : u32) -> NTSTATUS) ; windows_targets :: link_raw_dylib ! ("ntdll.dll" "system" fn NtReadFile (filehandle : HANDLE , event : HANDLE , apcroutine : PIO_APC_ROUTINE , apccontext : * const core :: ffi :: c_void , iostatusblock : * mut IO_STATUS_BLOCK , buffer : * mut core :: ffi :: c_void , length : u32 , byteoffset : * const i64 , key : * const u32) -> NTSTATUS) ; windows_targets :: link_raw_dylib ! ("ntdll.dll" "system" fn NtWriteFile (filehandle : HANDLE , event : HANDLE , apcroutine : PIO_APC_ROUTINE , apccontext : * const core :: ffi :: c_void , iostatusblock : * mut IO_STATUS_BLOCK , buffer : * const core :: ffi :: c_void , length : u32 , byteoffset : * const i64 , key : * const u32) -> NTSTATUS) ; windows_targets :: link_raw_dylib ! ("ntdll.dll" "system" fn RtlNtStatusToDosError (status : NTSTATUS) -> u32) ; } _ => { } }}