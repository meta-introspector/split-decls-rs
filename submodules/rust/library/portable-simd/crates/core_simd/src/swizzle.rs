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
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (# [$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (# [$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{# [macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{# [macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { compile_error ! (concat ! ("USE|" , module_path ! () , "|" , stringify ! ($ use_stmt))) ; } ; }}
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
mkuse!{use crate :: simd :: { LaneCount , Mask , MaskElement , Simd , SimdElement , SupportedLaneCount } ;}
mkitem!{# [doc = " Constructs a new SIMD vector by copying elements from selected elements in other vectors."] # [doc = ""] # [doc = " When swizzling one vector, elements are selected like [`Swizzle::swizzle`]."] # [doc = ""] # [doc = " When swizzling two vectors, elements are selected like [`Swizzle::concat_swizzle`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " With a single SIMD vector, the const array specifies element indices in that vector:"] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # use core::simd::{u32x2, u32x4, simd_swizzle};"] # [doc = " let v = u32x4::from_array([10, 11, 12, 13]);"] # [doc = ""] # [doc = " // Keeping the same size"] # [doc = " let r: u32x4 = simd_swizzle!(v, [3, 0, 1, 2]);"] # [doc = " assert_eq!(r.to_array(), [13, 10, 11, 12]);"] # [doc = ""] # [doc = " // Changing the number of elements"] # [doc = " let r: u32x2 = simd_swizzle!(v, [3, 1]);"] # [doc = " assert_eq!(r.to_array(), [13, 11]);"] # [doc = " ```"] # [doc = ""] # [doc = " With two input SIMD vectors, the const array specifies element indices in the concatenation of"] # [doc = " those vectors:"] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::{u32x2, u32x4, simd_swizzle};"] # [doc = " let a = u32x4::from_array([0, 1, 2, 3]);"] # [doc = " let b = u32x4::from_array([4, 5, 6, 7]);"] # [doc = ""] # [doc = " // Keeping the same size"] # [doc = " let r: u32x4 = simd_swizzle!(a, b, [0, 1, 6, 7]);"] # [doc = " assert_eq!(r.to_array(), [0, 1, 6, 7]);"] # [doc = ""] # [doc = " // Changing the number of elements"] # [doc = " let r: u32x2 = simd_swizzle!(a, b, [0, 4]);"] # [doc = " assert_eq!(r.to_array(), [0, 4]);"] # [doc = " ```"] # [allow (unused_macros)] pub macro simd_swizzle { ($ vector : expr , $ index : expr $ (,) ?) => { { use $ crate :: simd :: Swizzle ; struct Impl ; impl Swizzle < { $ index . len () } > for Impl { const INDEX : [usize ; { $ index . len () }] = $ index ; } Impl :: swizzle ($ vector) } } , ($ first : expr , $ second : expr , $ index : expr $ (,) ?) => { { use $ crate :: simd :: Swizzle ; struct Impl ; impl Swizzle < { $ index . len () } > for Impl { const INDEX : [usize ; { $ index . len () }] = $ index ; } Impl :: concat_swizzle ($ first , $ second) } } }}
mkitem!{mktrait!{# [doc = " Creates a vector from the elements of another vector."] pub trait Swizzle < const N : usize > { # [doc = " Map from the elements of the input vector to the output vector."] const INDEX : [usize ; N] ; # [doc = " Creates a new vector from the elements of `vector`."] # [doc = ""] # [doc = " Lane `i` of the output is `vector[Self::INDEX[i]]`."] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] fn swizzle < T , const M : usize > (vector : Simd < T , M >) -> Simd < T , N > where T : SimdElement , LaneCount < N > : SupportedLaneCount , LaneCount < M > : SupportedLaneCount , { unsafe { core :: intrinsics :: simd :: simd_shuffle (vector , vector , const { let mut output = [0 ; N] ; let mut i = 0 ; while i < N { let index = Self :: INDEX [i] ; assert ! (index as u32 as usize == index) ; assert ! (index < M , "source element index exceeds input vector length") ; output [i] = index as u32 ; i += 1 ; } # [repr (simd)] struct SimdShuffleIdx < const LEN : usize > ([u32 ; LEN]) ; SimdShuffleIdx (output) } ,) } } # [doc = " Creates a new vector from the elements of `first` and `second`."] # [doc = ""] # [doc = " Lane `i` of the output is `concat[Self::INDEX[i]]`, where `concat` is the concatenation of"] # [doc = " `first` and `second`."] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] fn concat_swizzle < T , const M : usize > (first : Simd < T , M > , second : Simd < T , M >) -> Simd < T , N > where T : SimdElement , LaneCount < N > : SupportedLaneCount , LaneCount < M > : SupportedLaneCount , { unsafe { core :: intrinsics :: simd :: simd_shuffle (first , second , const { let mut output = [0 ; N] ; let mut i = 0 ; while i < N { let index = Self :: INDEX [i] ; assert ! (index as u32 as usize == index) ; assert ! (index < 2 * M , "source element index exceeds input vector length") ; output [i] = index as u32 ; i += 1 ; } # [repr (simd)] struct SimdShuffleIdx < const LEN : usize > ([u32 ; LEN]) ; SimdShuffleIdx (output) } ,) } } # [doc = " Creates a new mask from the elements of `mask`."] # [doc = ""] # [doc = " Element `i` of the output is `mask[Self::INDEX[i]]`."] # [inline] # [must_use = "method returns a new mask and does not mutate the original inputs"] fn swizzle_mask < T , const M : usize > (mask : Mask < T , M >) -> Mask < T , N > where T : MaskElement , LaneCount < N > : SupportedLaneCount , LaneCount < M > : SupportedLaneCount , { unsafe { Mask :: from_int_unchecked (Self :: swizzle (mask . to_int ())) } } # [doc = " Creates a new mask from the elements of `first` and `second`."] # [doc = ""] # [doc = " Element `i` of the output is `concat[Self::INDEX[i]]`, where `concat` is the concatenation of"] # [doc = " `first` and `second`."] # [inline] # [must_use = "method returns a new mask and does not mutate the original inputs"] fn concat_swizzle_mask < T , const M : usize > (first : Mask < T , M > , second : Mask < T , M >) -> Mask < T , N > where T : MaskElement , LaneCount < N > : SupportedLaneCount , LaneCount < M > : SupportedLaneCount , { unsafe { Mask :: from_int_unchecked (Self :: concat_swizzle (first . to_int () , second . to_int ())) } } }}}
mkitem!{mkimpl!{impl < T , const N : usize > Simd < T , N > where T : SimdElement , LaneCount < N > : SupportedLaneCount , { # [doc = " Reverse the order of the elements in the vector."] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn reverse (self) -> Self { struct Reverse ; impl < const N : usize > Swizzle < N > for Reverse { const INDEX : [usize ; N] = const { let mut index = [0 ; N] ; let mut i = 0 ; while i < N { index [i] = N - i - 1 ; i += 1 ; } index } ; } Reverse :: swizzle (self) } # [doc = " Rotates the vector such that the first `OFFSET` elements of the slice move to the end"] # [doc = " while the last `self.len() - OFFSET` elements move to the front. After calling `rotate_elements_left`,"] # [doc = " the element previously at index `OFFSET` will become the first element in the slice."] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd::Simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd::Simd;"] # [doc = " let a = Simd::from_array([0, 1, 2, 3]);"] # [doc = " let x = a.rotate_elements_left::<3>();"] # [doc = " assert_eq!(x.to_array(), [3, 0, 1, 2]);"] # [doc = ""] # [doc = " let y = a.rotate_elements_left::<7>();"] # [doc = " assert_eq!(y.to_array(), [3, 0, 1, 2]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn rotate_elements_left < const OFFSET : usize > (self) -> Self { struct Rotate < const OFFSET : usize > ; impl < const OFFSET : usize , const N : usize > Swizzle < N > for Rotate < OFFSET > { const INDEX : [usize ; N] = const { let offset = OFFSET % N ; let mut index = [0 ; N] ; let mut i = 0 ; while i < N { index [i] = (i + offset) % N ; i += 1 ; } index } ; } Rotate :: < OFFSET > :: swizzle (self) } # [doc = " Rotates the vector such that the first `self.len() - OFFSET` elements of the vector move to"] # [doc = " the end while the last `OFFSET` elements move to the front. After calling `rotate_elements_right`,"] # [doc = " the element previously at index `self.len() - OFFSET` will become the first element in the slice."] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd::Simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd::Simd;"] # [doc = " let a = Simd::from_array([0, 1, 2, 3]);"] # [doc = " let x = a.rotate_elements_right::<3>();"] # [doc = " assert_eq!(x.to_array(), [1, 2, 3, 0]);"] # [doc = ""] # [doc = " let y = a.rotate_elements_right::<7>();"] # [doc = " assert_eq!(y.to_array(), [1, 2, 3, 0]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn rotate_elements_right < const OFFSET : usize > (self) -> Self { struct Rotate < const OFFSET : usize > ; impl < const OFFSET : usize , const N : usize > Swizzle < N > for Rotate < OFFSET > { const INDEX : [usize ; N] = const { let offset = N - OFFSET % N ; let mut index = [0 ; N] ; let mut i = 0 ; while i < N { index [i] = (i + offset) % N ; i += 1 ; } index } ; } Rotate :: < OFFSET > :: swizzle (self) } # [doc = " Shifts the vector elements to the left by `OFFSET`, filling in with"] # [doc = " `padding` from the right."] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd::Simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd::Simd;"] # [doc = " let a = Simd::from_array([0, 1, 2, 3]);"] # [doc = " let x = a.shift_elements_left::<3>(255);"] # [doc = " assert_eq!(x.to_array(), [3, 255, 255, 255]);"] # [doc = ""] # [doc = " let y = a.shift_elements_left::<7>(255);"] # [doc = " assert_eq!(y.to_array(), [255, 255, 255, 255]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn shift_elements_left < const OFFSET : usize > (self , padding : T) -> Self { struct Shift < const OFFSET : usize > ; impl < const OFFSET : usize , const N : usize > Swizzle < N > for Shift < OFFSET > { const INDEX : [usize ; N] = const { let mut index = [N ; N] ; let mut i = 0 ; while i + OFFSET < N { index [i] = i + OFFSET ; i += 1 ; } index } ; } Shift :: < OFFSET > :: concat_swizzle (self , Simd :: splat (padding)) } # [doc = " Shifts the vector elements to the right by `OFFSET`, filling in with"] # [doc = " `padding` from the left."] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd::Simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd::Simd;"] # [doc = " let a = Simd::from_array([0, 1, 2, 3]);"] # [doc = " let x = a.shift_elements_right::<3>(255);"] # [doc = " assert_eq!(x.to_array(), [255, 255, 255, 0]);"] # [doc = ""] # [doc = " let y = a.shift_elements_right::<7>(255);"] # [doc = " assert_eq!(y.to_array(), [255, 255, 255, 255]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn shift_elements_right < const OFFSET : usize > (self , padding : T) -> Self { struct Shift < const OFFSET : usize > ; impl < const OFFSET : usize , const N : usize > Swizzle < N > for Shift < OFFSET > { const INDEX : [usize ; N] = const { let mut index = [N ; N] ; let mut i = OFFSET ; while i < N { index [i] = i - OFFSET ; i += 1 ; } index } ; } Shift :: < OFFSET > :: concat_swizzle (self , Simd :: splat (padding)) } # [doc = " Interleave two vectors."] # [doc = ""] # [doc = " The resulting vectors contain elements taken alternatively from `self` and `other`, first"] # [doc = " filling the first result, and then the second."] # [doc = ""] # [doc = " The reverse of this operation is [`Simd::deinterleave`]."] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # use core::simd::Simd;"] # [doc = " let a = Simd::from_array([0, 1, 2, 3]);"] # [doc = " let b = Simd::from_array([4, 5, 6, 7]);"] # [doc = " let (x, y) = a.interleave(b);"] # [doc = " assert_eq!(x.to_array(), [0, 4, 1, 5]);"] # [doc = " assert_eq!(y.to_array(), [2, 6, 3, 7]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn interleave (self , other : Self) -> (Self , Self) { const fn interleave < const N : usize > (high : bool) -> [usize ; N] { let mut idx = [0 ; N] ; let mut i = 0 ; while i < N { let dst_index = if high { i + N } else { i } ; let src_index = dst_index / 2 + (dst_index % 2) * N ; idx [i] = src_index ; i += 1 ; } idx } struct Lo ; struct Hi ; impl < const N : usize > Swizzle < N > for Lo { const INDEX : [usize ; N] = interleave :: < N > (false) ; } impl < const N : usize > Swizzle < N > for Hi { const INDEX : [usize ; N] = interleave :: < N > (true) ; } (Lo :: concat_swizzle (self , other) , Hi :: concat_swizzle (self , other) ,) } # [doc = " Deinterleave two vectors."] # [doc = ""] # [doc = " The first result takes every other element of `self` and then `other`, starting with"] # [doc = " the first element."] # [doc = ""] # [doc = " The second result takes every other element of `self` and then `other`, starting with"] # [doc = " the second element."] # [doc = ""] # [doc = " The reverse of this operation is [`Simd::interleave`]."] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::Simd;"] # [doc = " let a = Simd::from_array([0, 4, 1, 5]);"] # [doc = " let b = Simd::from_array([2, 6, 3, 7]);"] # [doc = " let (x, y) = a.deinterleave(b);"] # [doc = " assert_eq!(x.to_array(), [0, 1, 2, 3]);"] # [doc = " assert_eq!(y.to_array(), [4, 5, 6, 7]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn deinterleave (self , other : Self) -> (Self , Self) { const fn deinterleave < const N : usize > (second : bool) -> [usize ; N] { let mut idx = [0 ; N] ; let mut i = 0 ; while i < N { idx [i] = i * 2 + second as usize ; i += 1 ; } idx } struct Even ; struct Odd ; impl < const N : usize > Swizzle < N > for Even { const INDEX : [usize ; N] = deinterleave :: < N > (false) ; } impl < const N : usize > Swizzle < N > for Odd { const INDEX : [usize ; N] = deinterleave :: < N > (true) ; } (Even :: concat_swizzle (self , other) , Odd :: concat_swizzle (self , other) ,) } # [doc = " Resize a vector."] # [doc = ""] # [doc = " If `M` > `N`, extends the length of a vector, setting the new elements to `value`."] # [doc = " If `M` < `N`, truncates the vector to the first `M` elements."] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::u32x4;"] # [doc = " let x = u32x4::from_array([0, 1, 2, 3]);"] # [doc = " assert_eq!(x.resize::<8>(9).to_array(), [0, 1, 2, 3, 9, 9, 9, 9]);"] # [doc = " assert_eq!(x.resize::<2>(9).to_array(), [0, 1]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn resize < const M : usize > (self , value : T) -> Simd < T , M > where LaneCount < M > : SupportedLaneCount , { struct Resize < const N : usize > ; impl < const N : usize , const M : usize > Swizzle < M > for Resize < N > { const INDEX : [usize ; M] = const { let mut index = [0 ; M] ; let mut i = 0 ; while i < M { index [i] = if i < N { i } else { N } ; i += 1 ; } index } ; } Resize :: < N > :: concat_swizzle (self , Simd :: splat (value)) } # [doc = " Extract a vector from another vector."] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::u32x4;"] # [doc = " let x = u32x4::from_array([0, 1, 2, 3]);"] # [doc = " assert_eq!(x.extract::<1, 2>().to_array(), [1, 2]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn extract < const START : usize , const LEN : usize > (self) -> Simd < T , LEN > where LaneCount < LEN > : SupportedLaneCount , { struct Extract < const N : usize , const START : usize > ; impl < const N : usize , const START : usize , const LEN : usize > Swizzle < LEN > for Extract < N , START > { const INDEX : [usize ; LEN] = const { assert ! (START + LEN <= N , "index out of bounds") ; let mut index = [0 ; LEN] ; let mut i = 0 ; while i < LEN { index [i] = START + i ; i += 1 ; } index } ; } Extract :: < N , START > :: swizzle (self) } }}}
mkitem!{mkimpl!{impl < T , const N : usize > Mask < T , N > where T : MaskElement , LaneCount < N > : SupportedLaneCount , { # [doc = " Reverse the order of the elements in the mask."] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn reverse (self) -> Self { unsafe { Self :: from_int_unchecked (self . to_int () . reverse ()) } } # [doc = " Rotates the mask such that the first `OFFSET` elements of the slice move to the end"] # [doc = " while the last `self.len() - OFFSET` elements move to the front. After calling `rotate_elements_left`,"] # [doc = " the element previously at index `OFFSET` will become the first element in the slice."] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn rotate_elements_left < const OFFSET : usize > (self) -> Self { unsafe { Self :: from_int_unchecked (self . to_int () . rotate_elements_left :: < OFFSET > ()) } } # [doc = " Rotates the mask such that the first `self.len() - OFFSET` elements of the mask move to"] # [doc = " the end while the last `OFFSET` elements move to the front. After calling `rotate_elements_right`,"] # [doc = " the element previously at index `self.len() - OFFSET` will become the first element in the slice."] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn rotate_elements_right < const OFFSET : usize > (self) -> Self { unsafe { Self :: from_int_unchecked (self . to_int () . rotate_elements_right :: < OFFSET > ()) } } # [doc = " Shifts the mask elements to the left by `OFFSET`, filling in with"] # [doc = " `padding` from the right."] # [inline] # [must_use = "method returns a new mask and does not mutate the original inputs"] pub fn shift_elements_left < const OFFSET : usize > (self , padding : bool) -> Self { unsafe { Self :: from_int_unchecked (self . to_int () . shift_elements_left :: < OFFSET > (if padding { T :: TRUE } else { T :: FALSE })) } } # [doc = " Shifts the mask elements to the right by `OFFSET`, filling in with"] # [doc = " `padding` from the left."] # [inline] # [must_use = "method returns a new mask and does not mutate the original inputs"] pub fn shift_elements_right < const OFFSET : usize > (self , padding : bool) -> Self { unsafe { Self :: from_int_unchecked (self . to_int () . shift_elements_right :: < OFFSET > (if padding { T :: TRUE } else { T :: FALSE })) } } # [doc = " Interleave two masks."] # [doc = ""] # [doc = " The resulting masks contain elements taken alternatively from `self` and `other`, first"] # [doc = " filling the first result, and then the second."] # [doc = ""] # [doc = " The reverse of this operation is [`Mask::deinterleave`]."] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::mask32x4;"] # [doc = " let a = mask32x4::from_array([false, true, false, true]);"] # [doc = " let b = mask32x4::from_array([false, false, true, true]);"] # [doc = " let (x, y) = a.interleave(b);"] # [doc = " assert_eq!(x.to_array(), [false, false, true, false]);"] # [doc = " assert_eq!(y.to_array(), [false, true, true, true]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn interleave (self , other : Self) -> (Self , Self) { let (lo , hi) = self . to_int () . interleave (other . to_int ()) ; unsafe { (Self :: from_int_unchecked (lo) , Self :: from_int_unchecked (hi)) } } # [doc = " Deinterleave two masks."] # [doc = ""] # [doc = " The first result takes every other element of `self` and then `other`, starting with"] # [doc = " the first element."] # [doc = ""] # [doc = " The second result takes every other element of `self` and then `other`, starting with"] # [doc = " the second element."] # [doc = ""] # [doc = " The reverse of this operation is [`Mask::interleave`]."] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::mask32x4;"] # [doc = " let a = mask32x4::from_array([false, true, false, true]);"] # [doc = " let b = mask32x4::from_array([false, false, true, true]);"] # [doc = " let (x, y) = a.deinterleave(b);"] # [doc = " assert_eq!(x.to_array(), [false, false, false, true]);"] # [doc = " assert_eq!(y.to_array(), [true, true, false, true]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn deinterleave (self , other : Self) -> (Self , Self) { let (even , odd) = self . to_int () . deinterleave (other . to_int ()) ; unsafe { (Self :: from_int_unchecked (even) , Self :: from_int_unchecked (odd) ,) } } # [doc = " Resize a mask."] # [doc = ""] # [doc = " If `M` > `N`, extends the length of a mask, setting the new elements to `value`."] # [doc = " If `M` < `N`, truncates the mask to the first `M` elements."] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::mask32x4;"] # [doc = " let x = mask32x4::from_array([false, true, true, false]);"] # [doc = " assert_eq!(x.resize::<8>(true).to_array(), [false, true, true, false, true, true, true, true]);"] # [doc = " assert_eq!(x.resize::<2>(true).to_array(), [false, true]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn resize < const M : usize > (self , value : bool) -> Mask < T , M > where LaneCount < M > : SupportedLaneCount , { unsafe { Mask :: < T , M > :: from_int_unchecked (self . to_int () . resize :: < M > (if value { T :: TRUE } else { T :: FALSE })) } } # [doc = " Extract a vector from another vector."] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::mask32x4;"] # [doc = " let x = mask32x4::from_array([false, true, true, false]);"] # [doc = " assert_eq!(x.extract::<1, 2>().to_array(), [true, true]);"] # [doc = " ```"] # [inline] # [must_use = "method returns a new vector and does not mutate the original inputs"] pub fn extract < const START : usize , const LEN : usize > (self) -> Mask < T , LEN > where LaneCount < LEN > : SupportedLaneCount , { unsafe { Mask :: < T , LEN > :: from_int_unchecked (self . to_int () . extract :: < START , LEN > ()) } } }}}