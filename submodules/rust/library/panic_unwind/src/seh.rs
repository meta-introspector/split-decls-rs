mkuse!{use alloc :: boxed :: Box ;}
mkuse!{use core :: any :: Any ;}
mkuse!{use core :: ffi :: { c_int , c_uint , c_void } ;}
mkuse!{use core :: mem :: ManuallyDrop ;}
mkitem!{mkstruct!{# [repr (C)] struct Exception { canary : * const _TypeDescriptor , data : Option < Box < dyn Any + Send > > , }}}
mkmod!{imp, { 
                getname!(imp);
                getsrc!(imp);
                getpath!(imp);
                get_deps!(imp);
                get_crates!(imp);
                mkinclude!(imp);
                mkitem!{mkstruct!{# [repr (transparent)] # [derive (Copy , Clone)] pub (super) struct ptr_t (* mut u8) ;}}
mkitem!{mkimpl!{impl ptr_t { pub (super) const fn null () -> Self { Self (core :: ptr :: null_mut ()) } pub (super) const fn new (ptr : * mut u8) -> Self { Self (ptr) } pub (super) const fn raw (self) -> * mut u8 { self . 0 } }}} 
            }}
mkmod!{imp, { 
                getname!(imp);
                getsrc!(imp);
                getpath!(imp);
                get_deps!(imp);
                get_crates!(imp);
                mkinclude!(imp);
                mkitem!{mkstruct!{# [repr (transparent)] # [derive (Copy , Clone)] pub (super) struct ptr_t (u32) ;}}
mkitem!{unsafe extern "C" { static __ImageBase : u8 ; }}
mkitem!{mkimpl!{impl ptr_t { pub (super) const fn null () -> Self { Self (0) } pub (super) fn new (ptr : * mut u8) -> Self { let addr : usize = ptr . expose_provenance () ; let image_base = (& raw const __ImageBase) . addr () ; let offset : usize = addr - image_base ; Self (offset as u32) } pub (super) const fn raw (self) -> u32 { self . 0 } }}} 
            }}
mkuse!{use imp :: ptr_t ;}
mkitem!{mkstruct!{# [repr (C)] struct _ThrowInfo { pub attributes : c_uint , pub pmfnUnwind : ptr_t , pub pForwardCompat : ptr_t , pub pCatchableTypeArray : ptr_t , }}}
mkitem!{mkstruct!{# [repr (C)] struct _CatchableTypeArray { pub nCatchableTypes : c_int , pub arrayOfCatchableTypes : [ptr_t ; 1] , }}}
mkitem!{mkstruct!{# [repr (C)] struct _CatchableType { pub properties : c_uint , pub pType : ptr_t , pub thisDisplacement : _PMD , pub sizeOrOffset : c_int , pub copyFunction : ptr_t , }}}
mkitem!{mkstruct!{# [repr (C)] struct _PMD { pub mdisp : c_int , pub pdisp : c_int , pub vdisp : c_int , }}}
mkitem!{mkstruct!{# [repr (C)] struct _TypeDescriptor { pub pVFTable : * const u8 , pub spare : * mut u8 , pub name : [u8 ; 11] , }}}
mkitem!{const TYPE_NAME : [u8 ; 11] = * b"rust_panic\0" ;}
mkitem!{static mut THROW_INFO : _ThrowInfo = _ThrowInfo { attributes : 0 , pmfnUnwind : ptr_t :: null () , pForwardCompat : ptr_t :: null () , pCatchableTypeArray : ptr_t :: null () , } ;}
mkitem!{static mut CATCHABLE_TYPE_ARRAY : _CatchableTypeArray = _CatchableTypeArray { nCatchableTypes : 1 , arrayOfCatchableTypes : [ptr_t :: null ()] } ;}
mkitem!{static mut CATCHABLE_TYPE : _CatchableType = _CatchableType { properties : 0 , pType : ptr_t :: null () , thisDisplacement : _PMD { mdisp : 0 , pdisp : - 1 , vdisp : 0 } , sizeOrOffset : size_of :: < Exception > () as c_int , copyFunction : ptr_t :: null () , } ;}
mkitem!{unsafe extern "C" { # [link_name = "\x01??_7type_info@@6B@"] static TYPE_INFO_VTABLE : * const u8 ; }}
mkitem!{static mut TYPE_DESCRIPTOR : _TypeDescriptor = _TypeDescriptor { pVFTable : (& raw const TYPE_INFO_VTABLE) as * const _ , spare : core :: ptr :: null_mut () , name : TYPE_NAME , } ;}
mkitem!{macro_rules ! define_cleanup { ($ abi : tt $ abi2 : tt) => { unsafe extern $ abi fn exception_cleanup (e : * mut Exception) { unsafe { if let Exception { data : Some (b) , .. } = e . read () { drop (b) ; super :: __rust_drop_panic () ; } } } unsafe extern $ abi2 fn exception_copy (_dest : * mut Exception , _src : * mut Exception) -> * mut Exception { unsafe { throw_exception (None) ; } } } }}
mkitem!{cfg_select ! { target_arch = "x86" => { define_cleanup ! ("thiscall" "thiscall-unwind") ; } _ => { define_cleanup ! ("C" "C-unwind") ; } }}

macro_rules! panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic in module {}", module_path!());
    };
}

mkfn!{
    panic_introspect!();
    pub (crate) unsafe fn panic (data : Box < dyn Any + Send >) -> u32 { unsafe { throw_exception (Some (data)) } }
}

macro_rules! throw_exception_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function throw_exception in module {}", module_path!());
    };
}

mkfn!{
    throw_exception_introspect!();
    unsafe fn throw_exception (data : Option < Box < dyn Any + Send > >) -> ! { use core :: intrinsics :: { AtomicOrdering , atomic_store } ; let mut exception = ManuallyDrop :: new (Exception { canary : (& raw const TYPE_DESCRIPTOR) , data }) ; let throw_ptr = (& raw mut exception) as * mut _ ; unsafe { atomic_store :: < _ , { AtomicOrdering :: SeqCst } > ((& raw mut THROW_INFO . pmfnUnwind) . cast () , ptr_t :: new (exception_cleanup as * mut u8) . raw () ,) ; atomic_store :: < _ , { AtomicOrdering :: SeqCst } > ((& raw mut THROW_INFO . pCatchableTypeArray) . cast () , ptr_t :: new ((& raw mut CATCHABLE_TYPE_ARRAY) . cast ()) . raw () ,) ; atomic_store :: < _ , { AtomicOrdering :: SeqCst } > ((& raw mut CATCHABLE_TYPE_ARRAY . arrayOfCatchableTypes [0]) . cast () , ptr_t :: new ((& raw mut CATCHABLE_TYPE) . cast ()) . raw () ,) ; atomic_store :: < _ , { AtomicOrdering :: SeqCst } > ((& raw mut CATCHABLE_TYPE . pType) . cast () , ptr_t :: new ((& raw mut TYPE_DESCRIPTOR) . cast ()) . raw () ,) ; atomic_store :: < _ , { AtomicOrdering :: SeqCst } > ((& raw mut CATCHABLE_TYPE . copyFunction) . cast () , ptr_t :: new (exception_copy as * mut u8) . raw () ,) ; } unsafe extern "system-unwind" { fn _CxxThrowException (pExceptionObject : * mut c_void , pThrowInfo : * mut u8) -> ! ; } unsafe { _CxxThrowException (throw_ptr , (& raw mut THROW_INFO) as * mut _) ; } }
}

macro_rules! cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cleanup in module {}", module_path!());
    };
}

mkfn!{
    cleanup_introspect!();
    pub (crate) unsafe fn cleanup (payload : * mut u8) -> Box < dyn Any + Send > { unsafe { if payload . is_null () { super :: __rust_foreign_exception () ; } let exception = payload as * mut Exception ; let canary = (& raw const (* exception) . canary) . read () ; if ! core :: ptr :: eq (canary , & raw const TYPE_DESCRIPTOR) { super :: __rust_foreign_exception () ; } (* exception) . data . take () . unwrap () } }
}