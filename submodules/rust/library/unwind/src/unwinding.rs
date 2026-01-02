mkuse!{use core :: ffi :: { c_int , c_void } ;}
mkitem!{pub type _Unwind_Action = c_int ;}
mkitem!{pub const _UA_SEARCH_PHASE : c_int = 1 ;}
mkitem!{pub const _UA_CLEANUP_PHASE : c_int = 2 ;}
mkitem!{pub const _UA_HANDLER_FRAME : c_int = 4 ;}
mkitem!{pub const _UA_FORCE_UNWIND : c_int = 8 ;}
mkitem!{pub const _UA_END_OF_STACK : c_int = 16 ;}
mkitem!{mkenum!{# [repr (C)] # [derive (Debug , Copy , Clone , PartialEq)] pub enum _Unwind_Reason_Code { _URC_NO_REASON = 0 , _URC_FOREIGN_EXCEPTION_CAUGHT = 1 , _URC_FATAL_PHASE2_ERROR = 2 , _URC_FATAL_PHASE1_ERROR = 3 , _URC_NORMAL_STOP = 4 , _URC_END_OF_STACK = 5 , _URC_HANDLER_FOUND = 6 , _URC_INSTALL_CONTEXT = 7 , _URC_CONTINUE_UNWIND = 8 , _URC_FAILURE = 9 , }}}
mkuse!{pub use _Unwind_Reason_Code :: * ;}
mkuse!{pub use unwinding :: abi :: { UnwindContext , UnwindException } ;}
mkitem!{mkenum!{pub enum _Unwind_Context { }}}
mkuse!{pub use unwinding :: custom_eh_frame_finder :: { EhFrameFinder , FrameInfo , FrameInfoKind , set_custom_eh_frame_finder , } ;}
mkitem!{pub type _Unwind_Exception_Class = u64 ;}
mkitem!{pub type _Unwind_Word = * const u8 ;}
mkitem!{pub type _Unwind_Ptr = * const u8 ;}
mkitem!{pub const unwinder_private_data_size : usize = size_of :: < UnwindException > () - size_of :: < _Unwind_Exception_Class > () - size_of :: < _Unwind_Exception_Cleanup_Fn > () ;}
mkitem!{pub type _Unwind_Exception_Cleanup_Fn = Option < extern "C" fn (unwind_code : _Unwind_Reason_Code , exception : * mut _Unwind_Exception) > ;}
mkitem!{mkstruct!{# [repr (C)] pub struct _Unwind_Exception { pub exception_class : _Unwind_Exception_Class , pub exception_cleanup : _Unwind_Exception_Cleanup_Fn , pub private : [_Unwind_Word ; unwinder_private_data_size] , }}}

macro_rules! _Unwind_GetDataRelBase_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_GetDataRelBase in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_GetDataRelBase_introspect!();
    pub unsafe fn _Unwind_GetDataRelBase (ctx : * mut _Unwind_Context) -> _Unwind_Ptr { let ctx = unsafe { & mut * (ctx as * mut UnwindContext < '_ >) } ; unwinding :: abi :: _Unwind_GetDataRelBase (ctx) as _Unwind_Ptr }
}

macro_rules! _Unwind_GetTextRelBase_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_GetTextRelBase in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_GetTextRelBase_introspect!();
    pub unsafe fn _Unwind_GetTextRelBase (ctx : * mut _Unwind_Context) -> _Unwind_Ptr { let ctx = unsafe { & mut * (ctx as * mut UnwindContext < '_ >) } ; unwinding :: abi :: _Unwind_GetTextRelBase (ctx) as _Unwind_Ptr }
}

macro_rules! _Unwind_GetRegionStart_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_GetRegionStart in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_GetRegionStart_introspect!();
    pub unsafe fn _Unwind_GetRegionStart (ctx : * mut _Unwind_Context) -> _Unwind_Ptr { let ctx = unsafe { & mut * (ctx as * mut UnwindContext < '_ >) } ; unwinding :: abi :: _Unwind_GetRegionStart (ctx) as _Unwind_Ptr }
}

macro_rules! _Unwind_SetGR_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_SetGR in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_SetGR_introspect!();
    pub unsafe fn _Unwind_SetGR (ctx : * mut _Unwind_Context , reg_index : c_int , value : _Unwind_Word) { let ctx = unsafe { & mut * (ctx as * mut UnwindContext < '_ >) } ; unwinding :: abi :: _Unwind_SetGR (ctx , reg_index , value as usize) }
}

macro_rules! _Unwind_SetIP_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_SetIP in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_SetIP_introspect!();
    pub unsafe fn _Unwind_SetIP (ctx : * mut _Unwind_Context , value : _Unwind_Word) { let ctx = unsafe { & mut * (ctx as * mut UnwindContext < '_ >) } ; unwinding :: abi :: _Unwind_SetIP (ctx , value as usize) }
}

macro_rules! _Unwind_GetIPInfo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_GetIPInfo in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_GetIPInfo_introspect!();
    pub unsafe fn _Unwind_GetIPInfo (ctx : * mut _Unwind_Context , ip_before_insn : * mut c_int ,) -> _Unwind_Word { let ctx = unsafe { & mut * (ctx as * mut UnwindContext < '_ >) } ; let ip_before_insn = unsafe { & mut * (ip_before_insn as * mut c_int) } ; unsafe { & * (unwinding :: abi :: _Unwind_GetIPInfo (ctx , ip_before_insn) as _Unwind_Word) } }
}

macro_rules! _Unwind_GetLanguageSpecificData_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_GetLanguageSpecificData in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_GetLanguageSpecificData_introspect!();
    pub unsafe fn _Unwind_GetLanguageSpecificData (ctx : * mut _Unwind_Context) -> * mut c_void { let ctx = unsafe { & mut * (ctx as * mut UnwindContext < '_ >) } ; unwinding :: abi :: _Unwind_GetLanguageSpecificData (ctx) }
}

macro_rules! _Unwind_RaiseException_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_RaiseException in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_RaiseException_introspect!();
    pub unsafe fn _Unwind_RaiseException (exception : * mut _Unwind_Exception) -> _Unwind_Reason_Code { let exception = unsafe { & mut * (exception as * mut UnwindException) } ; unsafe { core :: mem :: transmute (unwinding :: abi :: _Unwind_RaiseException (exception)) } }
}

macro_rules! _Unwind_DeleteException_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_DeleteException in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_DeleteException_introspect!();
    pub unsafe fn _Unwind_DeleteException (exception : * mut _Unwind_Exception) { let exception = unsafe { & mut * (exception as * mut UnwindException) } ; unsafe { unwinding :: abi :: _Unwind_DeleteException (exception) } }
}