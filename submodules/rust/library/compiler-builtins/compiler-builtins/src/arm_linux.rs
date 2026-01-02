mkuse!{use std :: sync :: Mutex ;}
mkuse!{use std :: collections :: HashMap ;}
mkuse!{use std :: sync :: LazyLock ;}
mkitem!{static USE_MATRIX : LazyLock < Mutex < HashMap < String , Vec < String > > > > = LazyLock :: new (| | Mutex :: new (HashMap :: new ())) ;}

macro_rules! get_use_matrix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_use_matrix in module {}", module_path!());
    };
}

mkfn!{
    get_use_matrix_introspect!();
    pub fn get_use_matrix () -> HashMap < String , Vec < String > > { USE_MATRIX . lock () . unwrap () . clone () }
}
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { { use std :: fs :: OpenOptions ; use std :: io :: Write ; let message = format ! ($ ($ arg) *) ; if let Ok (mut file) = OpenOptions :: new () . create (true) . append (true) . open ("macro_report.txt") { let _ = writeln ! (file , "{}" , message) ; } } } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (#[$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (#[$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{#[macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{#[macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { emit_message ! ("USE|{}|{}" , module_path ! () , stringify ! ($ use_stmt)) ; $ use_stmt } ; }}
mkitem!{macro_rules ! mkstruct { ($ struct_def : item) => { $ struct_def } ; }}
mkitem!{macro_rules ! mkenum { ($ enum_def : item) => { $ enum_def } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { stringify ! ($ name) } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { "processed file" } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { "processed_path" } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { vec ! [] } ; }}
mkmod!{rustc_complete, { 
                getname!(rustc_complete);
                getsrc!(rustc_complete);
                getpath!(rustc_complete);
                get_deps!(rustc_complete);
                get_crates!(rustc_complete);
                mkinclude!(rustc_complete);
                mkmod!{emitter, { 
                getname!(emitter);
                getsrc!(emitter);
                getpath!(emitter);
                get_deps!(emitter);
                get_crates!(emitter);
                mkinclude!(emitter);
                
macro_rules! stderr_destination_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stderr_destination in module {}", module_path!());
    };
}

mkfn!{
    stderr_destination_introspect!();
    pub fn stderr_destination () { }
} 
            }}
mkmod!{registry, { 
                getname!(registry);
                getsrc!(registry);
                getpath!(registry);
                get_deps!(registry);
                get_crates!(registry);
                mkinclude!(registry);
                mkitem!{mkstruct!{pub struct Registry ;}} 
            }}
mkmod!{translation, { 
                getname!(translation);
                getsrc!(translation);
                getpath!(translation);
                get_deps!(translation);
                get_crates!(translation);
                mkinclude!(translation);
                mkitem!{mkstruct!{pub struct Translator ;}} 
            }}
mkitem!{mkstruct!{pub struct ColorConfig ;}}
mkitem!{mkstruct!{pub struct DiagCtxt ;}}
mkitem!{mkstruct!{pub struct ErrCode ;}}
mkitem!{mkstruct!{pub struct FatalError ;}}
mkitem!{mkstruct!{pub struct PResult < T > (pub T) ;}}
mkmod!{markdown, { 
                getname!(markdown);
                getsrc!(markdown);
                getpath!(markdown);
                get_deps!(markdown);
                get_crates!(markdown);
                mkinclude!(markdown);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                mkitem!{mkstruct!{pub struct CG_OPTIONS ;}}
mkitem!{mkstruct!{pub struct CrateType ;}}
mkitem!{mkstruct!{pub struct ErrorOutputType ;}}
mkitem!{mkstruct!{pub struct Input ;}}
mkitem!{mkstruct!{pub struct OptionDesc ;}}
mkitem!{mkstruct!{pub struct OutFileName ;}}
mkitem!{mkstruct!{pub struct OutputType ;}}
mkitem!{mkstruct!{pub struct Sysroot ;}}
mkitem!{mkstruct!{pub struct UnstableOptions ;}}
mkitem!{mkstruct!{pub struct Z_OPTIONS ;}}

macro_rules! nightly_options_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nightly_options in module {}", module_path!());
    };
}

mkfn!{
    nightly_options_introspect!();
    pub fn nightly_options () { }
}

macro_rules! parse_target_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_target_triple in module {}", module_path!());
    };
}

mkfn!{
    parse_target_triple_introspect!();
    pub fn parse_target_triple () { }
} 
            }}
mkmod!{getopts, { 
                getname!(getopts);
                getsrc!(getopts);
                getpath!(getopts);
                get_deps!(getopts);
                get_crates!(getopts);
                mkinclude!(getopts);
                mkitem!{mkstruct!{pub struct Matches ;}} 
            }}
mkmod!{lint, { 
                getname!(lint);
                getsrc!(lint);
                getpath!(lint);
                get_deps!(lint);
                get_crates!(lint);
                mkinclude!(lint);
                mkitem!{mkstruct!{pub struct Lint ;}}
mkitem!{mkstruct!{pub struct LintId ;}} 
            }}
mkmod!{output, { 
                getname!(output);
                getsrc!(output);
                getpath!(output);
                get_deps!(output);
                get_crates!(output);
                mkinclude!(output);
                mkitem!{mkstruct!{pub struct CRATE_TYPES ;}}

macro_rules! collect_crate_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_crate_types in module {}", module_path!());
    };
}

mkfn!{
    collect_crate_types_introspect!();
    pub fn collect_crate_types () { }
}

macro_rules! invalid_output_for_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invalid_output_for_target in module {}", module_path!());
    };
}

mkfn!{
    invalid_output_for_target_introspect!();
    pub fn invalid_output_for_target () { }
} 
            }}
mkitem!{mkstruct!{pub struct EarlyDiagCtxt ;}}
mkitem!{mkstruct!{pub struct Session ;}}
mkitem!{mkstruct!{pub struct FileName ;}}
mkmod!{def_id, { 
                getname!(def_id);
                getsrc!(def_id);
                getpath!(def_id);
                get_deps!(def_id);
                get_crates!(def_id);
                mkinclude!(def_id);
                mkitem!{mkstruct!{pub struct LOCAL_CRATE ;}} 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                mkitem!{mkstruct!{pub struct TyCtxt < T > (pub T) ;}} 
            }} 
            }}
mkmod!{session_diagnostics, { 
                getname!(session_diagnostics);
                getsrc!(session_diagnostics);
                getpath!(session_diagnostics);
                get_deps!(session_diagnostics);
                get_crates!(session_diagnostics);
                mkinclude!(session_diagnostics);
                mkitem!{mkstruct!{pub struct CantEmitMIR ;}}
mkitem!{mkstruct!{pub struct RLinkEmptyVersionNumber ;}}
mkitem!{mkstruct!{pub struct RLinkEncodingVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkRustcVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkWrongFileType ;}}
mkitem!{mkstruct!{pub struct RlinkCorruptFile ;}}
mkitem!{mkstruct!{pub struct RlinkNotAFile ;}}
mkitem!{mkstruct!{pub struct RlinkUnableToRead ;}}
mkitem!{mkstruct!{pub struct UnstableFeatureUsage ;}} 
            }}
mkitem!{macro_rules ! do_not_use_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use print") } ; }}
mkitem!{macro_rules ! do_not_use_safe_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use safe_print") } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { pub fn get_module_name () -> &'static str { stringify ! ($ name) } } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { pub fn get_source_info () -> &'static str { concat ! ("Module: " , stringify ! ($ name)) } } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { pub fn get_module_path () -> &'static str { module_path ! () } } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { pub fn get_dependencies () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! get_crates { ($ name : ident) => { pub fn get_required_crates () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! forall_crates { ($ ($ crate_name : ident) ,*) => { $ (extern crate $ crate_name ;) * } ; }}
mkitem!{macro_rules ! emit_extern { ($ crate_name : ident) => { extern crate $ crate_name ; } ; }}
mkitem!{macro_rules ! get_externs { ($ crate_name : ident) => { stringify ! ($ crate_name) } ; }}
mkuse!{use core :: sync :: atomic :: { AtomicU32 , Ordering } ;}
mkuse!{use core :: { arch , mem } ;}

macro_rules! __kuser_cmpxchg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __kuser_cmpxchg in module {}", module_path!());
    };
}

mkfn!{
    __kuser_cmpxchg_introspect!();
    unsafe fn __kuser_cmpxchg (oldval : u32 , newval : u32 , ptr : * mut u32) -> bool { let f = unsafe { mem :: transmute :: < _ , extern "C" fn (u32 , u32 , * mut u32) -> u32 > (0xffff0fc0usize as * const ()) } ; f (oldval , newval , ptr) == 0 }
}

macro_rules! __kuser_memory_barrier_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __kuser_memory_barrier in module {}", module_path!());
    };
}

mkfn!{
    __kuser_memory_barrier_introspect!();
    unsafe fn __kuser_memory_barrier () { let f = unsafe { mem :: transmute :: < _ , extern "C" fn () > (0xffff0fa0usize as * const ()) } ; f () ; }
}

macro_rules! align_ptr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function align_ptr in module {}", module_path!());
    };
}

mkfn!{
    align_ptr_introspect!();
    fn align_ptr < T > (ptr : * mut T) -> * mut u32 { let ptr_mask = 3 & (4 - mem :: size_of :: < T > ()) ; (ptr as usize & ! ptr_mask) as * mut u32 }
}

macro_rules! get_shift_mask_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_shift_mask in module {}", module_path!());
    };
}

mkfn!{
    get_shift_mask_introspect!();
    fn get_shift_mask < T > (ptr : * mut T) -> (u32 , u32) { let mask = match mem :: size_of :: < T > () { 1 => 0xff , 2 => 0xffff , 4 => 0xffffffff , _ => unreachable ! () , } ; let endian_adjust = if cfg ! (target_endian = "little") { 0 } else { 4 - mem :: size_of :: < T > () as u32 } ; let ptr_mask = 3 & (4 - mem :: size_of :: < T > ()) ; let shift = ((ptr as usize & ptr_mask) as u32 ^ endian_adjust) * 8 ; (shift , mask) }
}

macro_rules! extract_aligned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extract_aligned in module {}", module_path!());
    };
}

mkfn!{
    extract_aligned_introspect!();
    fn extract_aligned (aligned : u32 , shift : u32 , mask : u32) -> u32 { (aligned >> shift) & mask }
}

macro_rules! insert_aligned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_aligned in module {}", module_path!());
    };
}

mkfn!{
    insert_aligned_introspect!();
    fn insert_aligned (aligned : u32 , val : u32 , shift : u32 , mask : u32) -> u32 { (aligned & ! (mask << shift)) | ((val & mask) << shift) }
}

macro_rules! atomic_load_aligned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function atomic_load_aligned in module {}", module_path!());
    };
}

mkfn!{
    atomic_load_aligned_introspect!();
    #[doc = " Performs a relaxed atomic load of 4 bytes at `ptr`. Some of the bytes are allowed to be out of"] #[doc = " bounds as long as `size_of::<T>()` bytes are in bounds."] #[doc = ""] #[doc = " # Safety"] #[doc = ""] #[doc = " - `ptr` must be 4-aligned."] #[doc = " - `size_of::<T>()` must be at most 4."] #[doc = " - if `size_of::<T>() == 1`, `ptr` or `ptr` offset by 1, 2 or 3 bytes must be valid for a relaxed"] #[doc = "   atomic read of 1 byte."] #[doc = " - if `size_of::<T>() == 2`, `ptr` or `ptr` offset by 2 bytes must be valid for a relaxed atomic"] #[doc = "   read of 2 bytes."] #[doc = " - if `size_of::<T>() == 4`, `ptr` must be valid for a relaxed atomic read of 4 bytes."] unsafe fn atomic_load_aligned < T > (ptr : * mut u32) -> u32 { const { assert ! (size_of ::< T > () <= 4) } ; if size_of :: < T > () == 4 { unsafe { AtomicU32 :: from_ptr (ptr) . load (Ordering :: Relaxed) } } else { unsafe { let res : u32 ; arch :: asm ! ("ldr {res}, [{ptr}]" , ptr = in (reg) ptr , res = lateout (reg) res , options (nostack , preserves_flags , readonly)) ; res } } }
}

macro_rules! atomic_rmw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function atomic_rmw in module {}", module_path!());
    };
}

mkfn!{
    atomic_rmw_introspect!();
    unsafe fn atomic_rmw < T , F : Fn (u32) -> u32 , G : Fn (u32 , u32) -> u32 > (ptr : * mut T , f : F , g : G) -> u32 { let aligned_ptr = align_ptr (ptr) ; let (shift , mask) = get_shift_mask (ptr) ; loop { let curval_aligned = unsafe { atomic_load_aligned :: < T > (aligned_ptr) } ; let curval = extract_aligned (curval_aligned , shift , mask) ; let newval = f (curval) ; let newval_aligned = insert_aligned (curval_aligned , newval , shift , mask) ; if unsafe { __kuser_cmpxchg (curval_aligned , newval_aligned , aligned_ptr) } { return g (curval , newval) ; } } }
}

macro_rules! atomic_cmpxchg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function atomic_cmpxchg in module {}", module_path!());
    };
}

mkfn!{
    atomic_cmpxchg_introspect!();
    unsafe fn atomic_cmpxchg < T > (ptr : * mut T , oldval : u32 , newval : u32) -> u32 { let aligned_ptr = align_ptr (ptr) ; let (shift , mask) = get_shift_mask (ptr) ; loop { let curval_aligned = unsafe { atomic_load_aligned :: < T > (aligned_ptr) } ; let curval = extract_aligned (curval_aligned , shift , mask) ; if curval != oldval { return curval ; } let newval_aligned = insert_aligned (curval_aligned , newval , shift , mask) ; if unsafe { __kuser_cmpxchg (curval_aligned , newval_aligned , aligned_ptr) } { return oldval ; } } }
}
mkitem!{macro_rules ! atomic_rmw { ($ name : ident , $ ty : ty , $ op : expr , $ fetch : expr) => { intrinsics ! { pub unsafe extern "C" fn $ name (ptr : * mut $ ty , val : $ ty) -> $ ty { unsafe { atomic_rmw (ptr , | x | $ op (x as $ ty , val) as u32 , | old , new | $ fetch (old , new)) as $ ty } } } } ; (@ old $ name : ident , $ ty : ty , $ op : expr) => { atomic_rmw ! ($ name , $ ty , $ op , | old , _ | old) ; } ; (@ new $ name : ident , $ ty : ty , $ op : expr) => { atomic_rmw ! ($ name , $ ty , $ op , | _ , new | new) ; } ; }}
mkitem!{macro_rules ! atomic_cmpxchg { ($ name : ident , $ ty : ty) => { intrinsics ! { pub unsafe extern "C" fn $ name (ptr : * mut $ ty , oldval : $ ty , newval : $ ty) -> $ ty { unsafe { atomic_cmpxchg (ptr , oldval as u32 , newval as u32) as $ ty } } } } ; }}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_add_1 , u8 , | a : u8 , b : u8 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_add_2 , u16 , | a : u16 , b : u16 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_add_4 , u32 , | a : u32 , b : u32 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_add_and_fetch_1 , u8 , | a : u8 , b : u8 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_add_and_fetch_2 , u16 , | a : u16 , b : u16 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_add_and_fetch_4 , u32 , | a : u32 , b : u32 | a . wrapping_add (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_sub_1 , u8 , | a : u8 , b : u8 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_sub_2 , u16 , | a : u16 , b : u16 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_sub_4 , u32 , | a : u32 , b : u32 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_sub_and_fetch_1 , u8 , | a : u8 , b : u8 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_sub_and_fetch_2 , u16 , | a : u16 , b : u16 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_sub_and_fetch_4 , u32 , | a : u32 , b : u32 | a . wrapping_sub (b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_and_1 , u8 , | a : u8 , b : u8 | a & b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_and_2 , u16 , | a : u16 , b : u16 | a & b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_and_4 , u32 , | a : u32 , b : u32 | a & b) ;}
mkitem!{atomic_rmw ! (@ new __sync_and_and_fetch_1 , u8 , | a : u8 , b : u8 | a & b) ;}
mkitem!{atomic_rmw ! (@ new __sync_and_and_fetch_2 , u16 , | a : u16 , b : u16 | a & b) ;}
mkitem!{atomic_rmw ! (@ new __sync_and_and_fetch_4 , u32 , | a : u32 , b : u32 | a & b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_or_1 , u8 , | a : u8 , b : u8 | a | b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_or_2 , u16 , | a : u16 , b : u16 | a | b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_or_4 , u32 , | a : u32 , b : u32 | a | b) ;}
mkitem!{atomic_rmw ! (@ new __sync_or_and_fetch_1 , u8 , | a : u8 , b : u8 | a | b) ;}
mkitem!{atomic_rmw ! (@ new __sync_or_and_fetch_2 , u16 , | a : u16 , b : u16 | a | b) ;}
mkitem!{atomic_rmw ! (@ new __sync_or_and_fetch_4 , u32 , | a : u32 , b : u32 | a | b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_xor_1 , u8 , | a : u8 , b : u8 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_xor_2 , u16 , | a : u16 , b : u16 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_xor_4 , u32 , | a : u32 , b : u32 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ new __sync_xor_and_fetch_1 , u8 , | a : u8 , b : u8 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ new __sync_xor_and_fetch_2 , u16 , | a : u16 , b : u16 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ new __sync_xor_and_fetch_4 , u32 , | a : u32 , b : u32 | a ^ b) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_nand_1 , u8 , | a : u8 , b : u8 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_nand_2 , u16 , | a : u16 , b : u16 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_nand_4 , u32 , | a : u32 , b : u32 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_nand_and_fetch_1 , u8 , | a : u8 , b : u8 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_nand_and_fetch_2 , u16 , | a : u16 , b : u16 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ new __sync_nand_and_fetch_4 , u32 , | a : u32 , b : u32 | ! (a & b)) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_max_1 , i8 , | a : i8 , b : i8 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_max_2 , i16 , | a : i16 , b : i16 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_max_4 , i32 , | a : i32 , b : i32 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umax_1 , u8 , | a : u8 , b : u8 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umax_2 , u16 , | a : u16 , b : u16 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umax_4 , u32 , | a : u32 , b : u32 | if a > b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_min_1 , i8 , | a : i8 , b : i8 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_min_2 , i16 , | a : i16 , b : i16 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_min_4 , i32 , | a : i32 , b : i32 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umin_1 , u8 , | a : u8 , b : u8 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umin_2 , u16 , | a : u16 , b : u16 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_fetch_and_umin_4 , u32 , | a : u32 , b : u32 | if a < b { a } else { b }) ;}
mkitem!{atomic_rmw ! (@ old __sync_lock_test_and_set_1 , u8 , | _ : u8 , b : u8 | b) ;}
mkitem!{atomic_rmw ! (@ old __sync_lock_test_and_set_2 , u16 , | _ : u16 , b : u16 | b) ;}
mkitem!{atomic_rmw ! (@ old __sync_lock_test_and_set_4 , u32 , | _ : u32 , b : u32 | b) ;}
mkitem!{atomic_cmpxchg ! (__sync_val_compare_and_swap_1 , u8) ;}
mkitem!{atomic_cmpxchg ! (__sync_val_compare_and_swap_2 , u16) ;}
mkitem!{atomic_cmpxchg ! (__sync_val_compare_and_swap_4 , u32) ;}
mkitem!{intrinsics ! { pub unsafe extern "C" fn __sync_synchronize () { unsafe { __kuser_memory_barrier () } ; } }}