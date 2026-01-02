mkuse!{use alloc :: boxed :: Box ;}
mkuse!{use core :: any :: Any ;}
mkuse!{use core :: sync :: atomic :: { AtomicBool , Ordering } ;}
mkuse!{use core :: { intrinsics , ptr } ;}
mkuse!{use unwind as uw ;}
mkitem!{mkstruct!{# [repr (C)] struct TypeInfo { vtable : * const usize , name : * const u8 , }}}
mkitem!{mkimpl!{unsafe impl Sync for TypeInfo { }}}
mkitem!{unsafe extern "C" { # [link_name = "\x01_ZTVN10__cxxabiv117__class_type_infoE"] static CLASS_TYPE_INFO_VTABLE : [usize ; 3] ; }}
mkitem!{# [lang = "eh_catch_typeinfo"] static EXCEPTION_TYPE_INFO : TypeInfo = TypeInfo { vtable : unsafe { & CLASS_TYPE_INFO_VTABLE [2] } , name : b"rust_panic\0" . as_ptr () , } ;}
mkitem!{mkstruct!{# [repr (C)] struct Exception { canary : * const TypeInfo , caught : AtomicBool , data : Option < Box < dyn Any + Send > > , }}}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub (crate) unsafe fn cleanup (ptr : * mut u8) -> Box < dyn Any + Send > { # [repr (C)] struct CatchData { ptr : * mut u8 , is_rust_panic : bool , } unsafe { let catch_data = & * (ptr as * mut CatchData) ; let adjusted_ptr = __cxa_begin_catch (catch_data . ptr as * mut libc :: c_void) as * mut Exception ; if ! catch_data . is_rust_panic { super :: __rust_foreign_exception () ; } let canary = (& raw const (* adjusted_ptr) . canary) . read () ; if ! ptr :: eq (canary , & EXCEPTION_TYPE_INFO) { super :: __rust_foreign_exception () ; } let was_caught = (* adjusted_ptr) . caught . swap (true , Ordering :: Relaxed) ; if was_caught { intrinsics :: abort () ; } let out = (* adjusted_ptr) . data . take () . unwrap () ; __cxa_end_catch () ; out } }
}

macro_rules! panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic in module {}", module_path!());
    };
}

mkfn!{
    panic_introspect!();
    pub (crate) unsafe fn panic (data : Box < dyn Any + Send >) -> u32 { unsafe { let exception = __cxa_allocate_exception (size_of :: < Exception > ()) as * mut Exception ; if exception . is_null () { return uw :: _URC_FATAL_PHASE1_ERROR as u32 ; } ptr :: write (exception , Exception { canary : & EXCEPTION_TYPE_INFO , caught : AtomicBool :: new (false) , data : Some (data) , } ,) ; __cxa_throw (exception as * mut _ , & EXCEPTION_TYPE_INFO , exception_cleanup) ; } }
}

macro_rules! exception_cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exception_cleanup in module {}", module_path!());
    };
}

mkfn!{
    exception_cleanup_introspect!();
    extern "C" fn exception_cleanup (ptr : * mut libc :: c_void) -> * mut libc :: c_void { unsafe { if let Some (b) = (ptr as * mut Exception) . read () . data { drop (b) ; super :: __rust_drop_panic () ; } ptr } }
}
mkitem!{unsafe extern "C" { fn __cxa_allocate_exception (thrown_size : libc :: size_t) -> * mut libc :: c_void ; fn __cxa_begin_catch (thrown_exception : * mut libc :: c_void) -> * mut libc :: c_void ; fn __cxa_end_catch () ; fn __cxa_throw (thrown_exception : * mut libc :: c_void , tinfo : * const TypeInfo , dest : extern "C" fn (* mut libc :: c_void) -> * mut libc :: c_void ,) -> ! ; }}