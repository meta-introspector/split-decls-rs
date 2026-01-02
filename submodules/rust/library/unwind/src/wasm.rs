mkitem!{mkenum!{# [repr (C)] # [derive (Debug , Copy , Clone , PartialEq)] pub enum _Unwind_Reason_Code { _URC_NO_REASON = 0 , _URC_FOREIGN_EXCEPTION_CAUGHT = 1 , _URC_FATAL_PHASE2_ERROR = 2 , _URC_FATAL_PHASE1_ERROR = 3 , _URC_NORMAL_STOP = 4 , _URC_END_OF_STACK = 5 , _URC_HANDLER_FOUND = 6 , _URC_INSTALL_CONTEXT = 7 , _URC_CONTINUE_UNWIND = 8 , _URC_FAILURE = 9 , }}}
mkuse!{pub use _Unwind_Reason_Code :: * ;}
mkitem!{pub type _Unwind_Exception_Class = u64 ;}
mkitem!{pub type _Unwind_Word = * const u8 ;}
mkitem!{pub const unwinder_private_data_size : usize = 2 ;}
mkitem!{mkstruct!{# [repr (C)] pub struct _Unwind_Exception { pub exception_class : _Unwind_Exception_Class , pub exception_cleanup : _Unwind_Exception_Cleanup_Fn , pub private : [_Unwind_Word ; unwinder_private_data_size] , }}}
mkitem!{pub type _Unwind_Exception_Cleanup_Fn = Option < extern "C" fn (unwind_code : _Unwind_Reason_Code , exception : * mut _Unwind_Exception) > ;}

macro_rules! _Unwind_DeleteException_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_DeleteException in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_DeleteException_introspect!();
    pub unsafe fn _Unwind_DeleteException (exception : * mut _Unwind_Exception) { if let Some (exception_cleanup) = unsafe { (* exception) . exception_cleanup } { exception_cleanup (_URC_FOREIGN_EXCEPTION_CAUGHT , exception) ; } }
}

macro_rules! _Unwind_RaiseException_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_RaiseException in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_RaiseException_introspect!();
    pub unsafe fn _Unwind_RaiseException (exception : * mut _Unwind_Exception) -> _Unwind_Reason_Code { cfg_select ! { panic = "unwind" => { const CPP_EXCEPTION_TAG : i32 = 0 ; core :: arch :: wasm :: throw ::< CPP_EXCEPTION_TAG > (exception . cast ()) } _ => { let _ = exception ; core :: arch :: wasm :: unreachable () } } }
}