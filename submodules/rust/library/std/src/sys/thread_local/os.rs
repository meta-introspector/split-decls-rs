mkuse!{use super :: key :: { Key , LazyKey , get , set } ;}
mkuse!{use super :: { abort_on_dtor_unwind , guard } ;}
mkuse!{use crate :: cell :: Cell ;}
mkuse!{use crate :: marker :: PhantomData ;}
mkuse!{use crate :: ptr ;}
mkitem!{# [doc (hidden)] # [allow_internal_unstable (thread_local_internals)] # [allow_internal_unsafe] # [unstable (feature = "thread_local_internals" , issue = "none")] # [rustc_macro_transparency = "semitransparent"] pub macro thread_local_inner { (@ key $ t : ty , const $ init : expr) => { $ crate :: thread :: local_impl :: thread_local_inner ! (@ key $ t , { const INIT_EXPR : $ t = $ init ; INIT_EXPR }) } , (@ key $ t : ty , $ init : expr) => { { # [inline] fn __init () -> $ t { $ init } unsafe { $ crate :: thread :: LocalKey :: new (| init | { static VAL : $ crate :: thread :: local_impl :: Storage <$ t > = $ crate :: thread :: local_impl :: Storage :: new () ; VAL . get (init , __init) }) } } } , ($ (# [$ attr : meta]) * $ vis : vis $ name : ident , $ t : ty , $ ($ init : tt) *) => { $ (# [$ attr]) * $ vis const $ name : $ crate :: thread :: LocalKey <$ t > = $ crate :: thread :: local_impl :: thread_local_inner ! (@ key $ t , $ ($ init) *) ; } , }}
mkitem!{mkstruct!{# [doc = " Use a regular global static to store this key; the state provided will then be"] # [doc = " thread-local."] # [allow (missing_debug_implementations)] pub struct Storage < T > { key : LazyKey , marker : PhantomData < Cell < T > > , }}}
mkitem!{mkimpl!{unsafe impl < T > Sync for Storage < T > { }}}
mkitem!{mkstruct!{struct Value < T : 'static > { value : T , key : Key , }}}
mkitem!{mkimpl!{impl < T : 'static > Storage < T > { pub const fn new () -> Storage < T > { Storage { key : LazyKey :: new (Some (destroy_value :: < T >)) , marker : PhantomData } } # [doc = " Gets a pointer to the TLS value, potentially initializing it with the"] # [doc = " provided parameters. If the TLS variable has been destroyed, a null"] # [doc = " pointer is returned."] # [doc = ""] # [doc = " The resulting pointer may not be used after reentrant inialialization"] # [doc = " or thread destruction has occurred."] pub fn get (& 'static self , i : Option < & mut Option < T > > , f : impl FnOnce () -> T) -> * const T { let key = self . key . force () ; let ptr = unsafe { get (key) as * mut Value < T > } ; if ptr . addr () > 1 { unsafe { & (* ptr) . value } } else { unsafe { Self :: try_initialize (key , ptr , i , f) } } } # [doc = " # Safety"] # [doc = " * `key` must be the result of calling `self.key.force()`"] # [doc = " * `ptr` must be the current value associated with `key`."] unsafe fn try_initialize (key : Key , ptr : * mut Value < T > , i : Option < & mut Option < T > > , f : impl FnOnce () -> T ,) -> * const T { if ptr . addr () == 1 { return ptr :: null () ; } let value = Box :: new (Value { value : i . and_then (Option :: take) . unwrap_or_else (f) , key }) ; let ptr = Box :: into_raw (value) ; let old = unsafe { let old = get (key) as * mut Value < T > ; set (key , ptr as * mut u8) ; old } ; if ! old . is_null () { drop (unsafe { Box :: from_raw (old) }) ; } unsafe { & (* ptr) . value } } }}}

macro_rules! destroy_value_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function destroy_value in module {}", module_path!());
    };
}

mkfn!{
    destroy_value_introspect!();
    unsafe extern "C" fn destroy_value < T : 'static > (ptr : * mut u8) { abort_on_dtor_unwind (| | { let ptr = unsafe { Box :: from_raw (ptr as * mut Value < T >) } ; let key = ptr . key ; unsafe { set (key , ptr :: without_provenance_mut (1)) } ; drop (ptr) ; unsafe { set (key , ptr :: null_mut ()) } ; guard :: enable () ; }) ; }
}
mkitem!{# [rustc_macro_transparency = "semitransparent"] pub (crate) macro local_pointer { () => { } , ($ vis : vis static $ name : ident ; $ ($ rest : tt) *) => { $ vis static $ name : $ crate :: sys :: thread_local :: LocalPointer = $ crate :: sys :: thread_local :: LocalPointer :: __new () ; $ crate :: sys :: thread_local :: local_pointer ! { $ ($ rest) * } } , }}
mkitem!{mkstruct!{pub (crate) struct LocalPointer { key : LazyKey , }}}
mkitem!{mkimpl!{impl LocalPointer { pub const fn __new () -> LocalPointer { LocalPointer { key : LazyKey :: new (None) } } pub fn get (& 'static self) -> * mut () { unsafe { get (self . key . force ()) as * mut () } } pub fn set (& 'static self , p : * mut ()) { unsafe { set (self . key . force () , p as * mut u8) } } }}}