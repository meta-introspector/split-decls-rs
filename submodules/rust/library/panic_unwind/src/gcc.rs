mkuse!{use alloc :: boxed :: Box ;}
mkuse!{use core :: any :: Any ;}
mkuse!{use core :: ptr ;}
mkuse!{use unwind as uw ;}
mkitem!{static CANARY : u8 = 0 ;}
mkitem!{mkstruct!{# [repr (C)] struct Exception { _uwe : uw :: _Unwind_Exception , canary : * const u8 , cause : Box < dyn Any + Send > , }}}

macro_rules! panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic in module {}", module_path!());
    };
}

mkfn!{
    panic_introspect!();
    pub (crate) unsafe fn panic (data : Box < dyn Any + Send >) -> u32 { let exception = Box :: new (Exception { _uwe : uw :: _Unwind_Exception { exception_class : RUST_EXCEPTION_CLASS , exception_cleanup : Some (exception_cleanup) , private : [core :: ptr :: null () ; uw :: unwinder_private_data_size] , } , canary : & CANARY , cause : data , }) ; let exception_param = Box :: into_raw (exception) as * mut uw :: _Unwind_Exception ; return unsafe { uw :: _Unwind_RaiseException (exception_param) as u32 } ; extern "C" fn exception_cleanup (_unwind_code : uw :: _Unwind_Reason_Code , exception : * mut uw :: _Unwind_Exception ,) { unsafe { let _ : Box < Exception > = Box :: from_raw (exception as * mut Exception) ; super :: __rust_drop_panic () ; } } }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub (crate) unsafe fn cleanup (ptr : * mut u8) -> Box < dyn Any + Send > { unsafe { let exception = ptr as * mut uw :: _Unwind_Exception ; if (* exception) . exception_class != RUST_EXCEPTION_CLASS { uw :: _Unwind_DeleteException (exception) ; super :: __rust_foreign_exception () ; } let exception = exception . cast :: < Exception > () ; let canary = (& raw const (* exception) . canary) . read () ; if ! ptr :: eq (canary , & CANARY) { super :: __rust_foreign_exception () ; } let exception = Box :: from_raw (exception as * mut Exception) ; exception . cause } }
}
mkitem!{const RUST_EXCEPTION_CLASS : uw :: _Unwind_Exception_Class = u64 :: from_ne_bytes (* b"MOZ\0RUST") ;}