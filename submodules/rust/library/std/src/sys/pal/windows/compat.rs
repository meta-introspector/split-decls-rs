mkuse!{use crate :: ffi :: { CStr , c_void } ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sys :: c ;}
mkitem!{# [cfg (target_vendor = "win7")] # [used] # [unsafe (link_section = ".CRT$XCT")] static INIT_TABLE_ENTRY : unsafe extern "C" fn () = init ;}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    # [doc = " Preload some imported functions."] # [doc = ""] # [doc = " Note that any functions included here will be unconditionally loaded in"] # [doc = " the final binary, regardless of whether or not they're actually used."] # [doc = ""] # [doc = " Therefore, this should be limited to `compat_fn_optional` functions which"] # [doc = " must be preloaded or any functions where lazier loading demonstrates a"] # [doc = " negative performance impact in practical situations."] # [doc = ""] # [doc = " Currently we only preload `WaitOnAddress` and `WakeByAddressSingle`."] # [cfg (target_vendor = "win7")] unsafe extern "C" fn init () { load_synch_functions () ; }
}
mkitem!{# [doc = " Helper macro for creating CStrs from literals and symbol names."] macro_rules ! ansi_str { (sym $ ident : ident) => { { crate :: sys :: compat :: const_cstr_from_bytes (concat ! (stringify ! ($ ident) , "\0") . as_bytes ()) } } ; ($ lit : literal) => { { crate :: sys :: compat :: const_cstr_from_bytes (concat ! ($ lit , "\0") . as_bytes ()) } } ; }}

macro_rules! const_cstr_from_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function const_cstr_from_bytes in module {}", module_path!());
    };
}

mkfn!{
    const_cstr_from_bytes_introspect!();
    # [doc = " Creates a C string wrapper from a byte slice, in a constant context."] # [doc = ""] # [doc = " This is a utility function used by the [`ansi_str`] macro."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the slice is not null terminated or contains nulls, except as the last item"] pub (crate) const fn const_cstr_from_bytes (bytes : & 'static [u8]) -> & 'static CStr { if ! matches ! (bytes . last () , Some (& 0)) { panic ! ("A CStr must be null terminated") ; } let mut i = 0 ; while i < bytes . len () - 1 { if bytes [i] == 0 { panic ! ("A CStr must not have interior nulls") } i += 1 ; } unsafe { crate :: ffi :: CStr :: from_bytes_with_nul_unchecked (bytes) } }
}
mkitem!{mkstruct!{# [doc = " Represents a loaded module."] # [doc = ""] # [doc = " Note that the modules std depends on must not be unloaded."] # [doc = " Therefore a `Module` is always valid for the lifetime of std."] # [derive (Copy , Clone)] pub (in crate :: sys) struct Module (NonNull < c_void >) ;}}
mkitem!{mkimpl!{impl Module { # [doc = " Try to get a handle to a loaded module."] # [doc = ""] # [doc = " # SAFETY"] # [doc = ""] # [doc = " This should only be use for modules that exist for the lifetime of std"] # [doc = " (e.g. kernel32 and ntdll)."] pub unsafe fn new (name : & CStr) -> Option < Self > { unsafe { let module = c :: GetModuleHandleA (name . as_ptr () . cast :: < u8 > ()) ; NonNull :: new (module) . map (Self) } } pub fn proc_address (self , name : & CStr) -> Option < NonNull < c_void > > { unsafe { let proc = c :: GetProcAddress (self . 0 . as_ptr () , name . as_ptr () . cast :: < u8 > ()) ; proc . map (| p | NonNull :: new_unchecked (p as * mut c_void)) } } }}}
mkitem!{# [doc = " Load a function or use a fallback implementation if that fails."] macro_rules ! compat_fn_with_fallback { (pub static $ module : ident : & CStr = $ name : expr ; $ ($ (# [$ meta : meta]) * $ vis : vis fn $ symbol : ident ($ ($ argname : ident : $ argtype : ty) ,*) -> $ rettype : ty $ fallback_body : block) *) => (pub static $ module : & CStr = $ name ; $ ($ (# [$ meta]) * pub mod $ symbol { # [allow (unused_imports)] use super ::*; use crate :: mem ; use crate :: ffi :: CStr ; use crate :: sync :: atomic :: { Atomic , AtomicPtr , Ordering } ; use crate :: sys :: compat :: Module ; type F = unsafe extern "system" fn ($ ($ argtype) ,*) -> $ rettype ; # [doc = " `PTR` contains a function pointer to one of three functions."] # [doc = " It starts with the `load` function."] # [doc = " When that is called it attempts to load the requested symbol."] # [doc = " If it succeeds, `PTR` is set to the address of that symbol."] # [doc = " If it fails, then `PTR` is set to `fallback`."] static PTR : Atomic <* mut c_void > = AtomicPtr :: new (load as * mut _) ; unsafe extern "system" fn load ($ ($ argname : $ argtype) ,*) -> $ rettype { unsafe { let func = load_from_module (Module :: new ($ module)) ; func ($ ($ argname) ,*) } } fn load_from_module (module : Option < Module >) -> F { unsafe { static SYMBOL_NAME : & CStr = ansi_str ! (sym $ symbol) ; if let Some (f) = module . and_then (| m | m . proc_address (SYMBOL_NAME)) { PTR . store (f . as_ptr () , Ordering :: Relaxed) ; mem :: transmute (f) } else { PTR . store (fallback as * mut _ , Ordering :: Relaxed) ; fallback } } } # [allow (unused_variables)] unsafe extern "system" fn fallback ($ ($ argname : $ argtype) ,*) -> $ rettype { $ fallback_body } # [inline (always)] pub unsafe fn call ($ ($ argname : $ argtype) ,*) -> $ rettype { unsafe { let func : F = mem :: transmute (PTR . load (Ordering :: Relaxed)) ; func ($ ($ argname) ,*) } } } # [allow (unused)] $ (# [$ meta]) * $ vis use $ symbol :: call as $ symbol ;) *) }}
mkitem!{# [doc = " Optionally loaded functions."] # [doc = ""] # [doc = " Relies on the functions being pre-loaded elsewhere."] # [cfg (target_vendor = "win7")] macro_rules ! compat_fn_optional { ($ ($ (# [$ meta : meta]) * $ vis : vis fn $ symbol : ident ($ ($ argname : ident : $ argtype : ty) ,*) $ (-> $ rettype : ty) ?;) +) => ($ (pub mod $ symbol { # [allow (unused_imports)] use super ::*; use crate :: ffi :: c_void ; use crate :: mem ; use crate :: ptr :: { self , NonNull } ; use crate :: sync :: atomic :: { Atomic , AtomicPtr , Ordering } ; pub (in crate :: sys) static PTR : Atomic <* mut c_void > = AtomicPtr :: new (ptr :: null_mut ()) ; type F = unsafe extern "system" fn ($ ($ argtype) ,*) $ (-> $ rettype) ?; # [inline (always)] pub fn option () -> Option < F > { NonNull :: new (PTR . load (Ordering :: Relaxed)) . map (| f | unsafe { mem :: transmute (f) }) } } # [inline] pub unsafe extern "system" fn $ symbol ($ ($ argname : $ argtype) ,*) $ (-> $ rettype) ? { unsafe { $ symbol :: option () . unwrap () ($ ($ argname) ,*) } }) +) }}

macro_rules! load_synch_functions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function load_synch_functions in module {}", module_path!());
    };
}

mkfn!{
    load_synch_functions_introspect!();
    # [doc = " Load all needed functions from \"api-ms-win-core-synch-l1-2-0\"."] # [cfg (target_vendor = "win7")] pub (super) fn load_synch_functions () { fn try_load () -> Option < () > { use crate :: sync :: atomic :: Ordering ; const MODULE_NAME : & CStr = c"api-ms-win-core-synch-l1-2-0" ; const WAIT_ON_ADDRESS : & CStr = c"WaitOnAddress" ; const WAKE_BY_ADDRESS_SINGLE : & CStr = c"WakeByAddressSingle" ; let library = unsafe { Module :: new (MODULE_NAME) } ? ; let wait_on_address = library . proc_address (WAIT_ON_ADDRESS) ? ; let wake_by_address_single = library . proc_address (WAKE_BY_ADDRESS_SINGLE) ? ; c :: WaitOnAddress :: PTR . store (wait_on_address . as_ptr () , Ordering :: Relaxed) ; c :: WakeByAddressSingle :: PTR . store (wake_by_address_single . as_ptr () , Ordering :: Relaxed) ; Some (()) } try_load () ; }
}