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
mkuse!{use super :: sealed :: Sealed ;}
mkuse!{use crate :: simd :: { LaneCount , Mask , Simd , SimdCast , SimdElement , SupportedLaneCount , cmp :: { SimdPartialEq , SimdPartialOrd } , } ;}
mkitem!{mktrait!{# [doc = " Operations on SIMD vectors of floats."] pub trait SimdFloat : Copy + Sealed { # [doc = " Mask type used for manipulating this SIMD vector type."] type Mask ; # [doc = " Scalar type contained by this SIMD vector type."] type Scalar ; # [doc = " Bit representation of this SIMD vector type."] type Bits ; # [doc = " A SIMD vector with a different element type."] type Cast < T : SimdElement > ; # [doc = " Performs elementwise conversion of this vector's elements to another SIMD-valid type."] # [doc = ""] # [doc = " This follows the semantics of Rust's `as` conversion for floats (truncating or saturating"] # [doc = " at the limits) for each element."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::prelude::*;"] # [doc = " let floats: Simd<f32, 4> = Simd::from_array([1.9, -4.5, f32::INFINITY, f32::NAN]);"] # [doc = " let ints = floats.cast::<i32>();"] # [doc = " assert_eq!(ints, Simd::from_array([1, -4, i32::MAX, 0]));"] # [doc = ""] # [doc = " // Formally equivalent, but `Simd::cast` can optimize better."] # [doc = " assert_eq!(ints, Simd::from_array(floats.to_array().map(|x| x as i32)));"] # [doc = ""] # [doc = " // The float conversion does not round-trip."] # [doc = " let floats_again = ints.cast();"] # [doc = " assert_ne!(floats, floats_again);"] # [doc = " assert_eq!(floats_again, Simd::from_array([1.0, -4.0, 2147483647.0, 0.0]));"] # [doc = " ```"] # [must_use] fn cast < T : SimdCast > (self) -> Self :: Cast < T > ; # [doc = " Rounds toward zero and converts to the same-width integer type, assuming that"] # [doc = " the value is finite and fits in that type."] # [doc = ""] # [doc = " # Safety"] # [doc = " The value must:"] # [doc = ""] # [doc = " * Not be NaN"] # [doc = " * Not be infinite"] # [doc = " * Be representable in the return type, after truncating off its fractional part"] # [doc = ""] # [doc = " If these requirements are infeasible or costly, consider using the safe function [cast],"] # [doc = " which saturates on conversion."] # [doc = ""] # [doc = " [cast]: Simd::cast"] unsafe fn to_int_unchecked < I : SimdCast > (self) -> Self :: Cast < I > where Self :: Scalar : core :: convert :: FloatToInt < I > ; # [doc = " Raw transmutation to an unsigned integer vector type with the"] # [doc = " same size and number of elements."] # [must_use = "method returns a new vector and does not mutate the original value"] fn to_bits (self) -> Self :: Bits ; # [doc = " Raw transmutation from an unsigned integer vector type with the"] # [doc = " same size and number of elements."] # [must_use = "method returns a new vector and does not mutate the original value"] fn from_bits (bits : Self :: Bits) -> Self ; # [doc = " Produces a vector where every element has the absolute value of the"] # [doc = " equivalently-indexed element in `self`."] # [must_use = "method returns a new vector and does not mutate the original value"] fn abs (self) -> Self ; # [doc = " Takes the reciprocal (inverse) of each element, `1/x`."] # [must_use = "method returns a new vector and does not mutate the original value"] fn recip (self) -> Self ; # [doc = " Converts each element from radians to degrees."] # [must_use = "method returns a new vector and does not mutate the original value"] fn to_degrees (self) -> Self ; # [doc = " Converts each element from degrees to radians."] # [must_use = "method returns a new vector and does not mutate the original value"] fn to_radians (self) -> Self ; # [doc = " Returns true for each element if it has a positive sign, including"] # [doc = " `+0.0`, `NaN`s with positive sign bit and positive infinity."] # [must_use = "method returns a new mask and does not mutate the original value"] fn is_sign_positive (self) -> Self :: Mask ; # [doc = " Returns true for each element if it has a negative sign, including"] # [doc = " `-0.0`, `NaN`s with negative sign bit and negative infinity."] # [must_use = "method returns a new mask and does not mutate the original value"] fn is_sign_negative (self) -> Self :: Mask ; # [doc = " Returns true for each element if its value is `NaN`."] # [must_use = "method returns a new mask and does not mutate the original value"] fn is_nan (self) -> Self :: Mask ; # [doc = " Returns true for each element if its value is positive infinity or negative infinity."] # [must_use = "method returns a new mask and does not mutate the original value"] fn is_infinite (self) -> Self :: Mask ; # [doc = " Returns true for each element if its value is neither infinite nor `NaN`."] # [must_use = "method returns a new mask and does not mutate the original value"] fn is_finite (self) -> Self :: Mask ; # [doc = " Returns true for each element if its value is subnormal."] # [must_use = "method returns a new mask and does not mutate the original value"] fn is_subnormal (self) -> Self :: Mask ; # [doc = " Returns true for each element if its value is neither zero, infinite,"] # [doc = " subnormal, nor `NaN`."] # [must_use = "method returns a new mask and does not mutate the original value"] fn is_normal (self) -> Self :: Mask ; # [doc = " Replaces each element with a number that represents its sign."] # [doc = ""] # [doc = " * `1.0` if the number is positive, `+0.0`, or `INFINITY`"] # [doc = " * `-1.0` if the number is negative, `-0.0`, or `NEG_INFINITY`"] # [doc = " * `NAN` if the number is `NAN`"] # [must_use = "method returns a new vector and does not mutate the original value"] fn signum (self) -> Self ; # [doc = " Returns each element with the magnitude of `self` and the sign of `sign`."] # [doc = ""] # [doc = " For any element containing a `NAN`, a `NAN` with the sign of `sign` is returned."] # [must_use = "method returns a new vector and does not mutate the original value"] fn copysign (self , sign : Self) -> Self ; # [doc = " Returns the minimum of each element."] # [doc = ""] # [doc = " If one of the values is `NAN`, then the other value is returned."] # [must_use = "method returns a new vector and does not mutate the original value"] fn simd_min (self , other : Self) -> Self ; # [doc = " Returns the maximum of each element."] # [doc = ""] # [doc = " If one of the values is `NAN`, then the other value is returned."] # [must_use = "method returns a new vector and does not mutate the original value"] fn simd_max (self , other : Self) -> Self ; # [doc = " Restrict each element to a certain interval unless it is NaN."] # [doc = ""] # [doc = " For each element in `self`, returns the corresponding element in `max` if the element is"] # [doc = " greater than `max`, and the corresponding element in `min` if the element is less"] # [doc = " than `min`.  Otherwise returns the element in `self`."] # [must_use = "method returns a new vector and does not mutate the original value"] fn simd_clamp (self , min : Self , max : Self) -> Self ; # [doc = " Returns the sum of the elements of the vector."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::prelude::*;"] # [doc = " let v = f32x2::from_array([1., 2.]);"] # [doc = " assert_eq!(v.reduce_sum(), 3.);"] # [doc = " ```"] fn reduce_sum (self) -> Self :: Scalar ; # [doc = " Reducing multiply.  Returns the product of the elements of the vector."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::prelude::*;"] # [doc = " let v = f32x2::from_array([3., 4.]);"] # [doc = " assert_eq!(v.reduce_product(), 12.);"] # [doc = " ```"] fn reduce_product (self) -> Self :: Scalar ; # [doc = " Returns the maximum element in the vector."] # [doc = ""] # [doc = " Returns values based on equality, so a vector containing both `0.` and `-0.` may"] # [doc = " return either."] # [doc = ""] # [doc = " This function will not return `NaN` unless all elements are `NaN`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::prelude::*;"] # [doc = " let v = f32x2::from_array([1., 2.]);"] # [doc = " assert_eq!(v.reduce_max(), 2.);"] # [doc = ""] # [doc = " // NaN values are skipped..."] # [doc = " let v = f32x2::from_array([1., f32::NAN]);"] # [doc = " assert_eq!(v.reduce_max(), 1.);"] # [doc = ""] # [doc = " // ...unless all values are NaN"] # [doc = " let v = f32x2::from_array([f32::NAN, f32::NAN]);"] # [doc = " assert!(v.reduce_max().is_nan());"] # [doc = " ```"] fn reduce_max (self) -> Self :: Scalar ; # [doc = " Returns the minimum element in the vector."] # [doc = ""] # [doc = " Returns values based on equality, so a vector containing both `0.` and `-0.` may"] # [doc = " return either."] # [doc = ""] # [doc = " This function will not return `NaN` unless all elements are `NaN`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(portable_simd)]"] # [doc = " # #[cfg(feature = \"as_crate\")] use core_simd::simd;"] # [doc = " # #[cfg(not(feature = \"as_crate\"))] use core::simd;"] # [doc = " # use simd::prelude::*;"] # [doc = " let v = f32x2::from_array([3., 7.]);"] # [doc = " assert_eq!(v.reduce_min(), 3.);"] # [doc = ""] # [doc = " // NaN values are skipped..."] # [doc = " let v = f32x2::from_array([1., f32::NAN]);"] # [doc = " assert_eq!(v.reduce_min(), 1.);"] # [doc = ""] # [doc = " // ...unless all values are NaN"] # [doc = " let v = f32x2::from_array([f32::NAN, f32::NAN]);"] # [doc = " assert!(v.reduce_min().is_nan());"] # [doc = " ```"] fn reduce_min (self) -> Self :: Scalar ; }}}
mkitem!{macro_rules ! impl_trait { { $ ($ ty : ty { bits : $ bits_ty : ty , mask : $ mask_ty : ty }) ,* } => { $ (impl < const N : usize > Sealed for Simd <$ ty , N > where LaneCount < N >: SupportedLaneCount , { } impl < const N : usize > SimdFloat for Simd <$ ty , N > where LaneCount < N >: SupportedLaneCount , { type Mask = Mask <<$ mask_ty as SimdElement >:: Mask , N >; type Scalar = $ ty ; type Bits = Simd <$ bits_ty , N >; type Cast < T : SimdElement > = Simd < T , N >; # [cfg (not (target_arch = "aarch64"))] # [inline] fn cast < T : SimdCast > (self) -> Self :: Cast < T > { unsafe { core :: intrinsics :: simd :: simd_as (self) } } # [cfg (target_arch = "aarch64")] # [inline] fn cast < T : SimdCast > (self) -> Self :: Cast < T > { const { assert ! (N <= 64) } ; if N <= 2 || N == 4 || N == 8 || N == 16 || N == 32 || N == 64 { unsafe { core :: intrinsics :: simd :: simd_as (self) } } else if N < 4 { let x = self . resize ::< 4 > (Default :: default ()) . cast () ; x . resize ::< N > (x [0]) } else if N < 8 { let x = self . resize ::< 8 > (Default :: default ()) . cast () ; x . resize ::< N > (x [0]) } else if N < 16 { let x = self . resize ::< 16 > (Default :: default ()) . cast () ; x . resize ::< N > (x [0]) } else if N < 32 { let x = self . resize ::< 32 > (Default :: default ()) . cast () ; x . resize ::< N > (x [0]) } else { let x = self . resize ::< 64 > (Default :: default ()) . cast () ; x . resize ::< N > (x [0]) } } # [inline] # [cfg_attr (miri , track_caller)] unsafe fn to_int_unchecked < I : SimdCast > (self) -> Self :: Cast < I > where Self :: Scalar : core :: convert :: FloatToInt < I >, { unsafe { core :: intrinsics :: simd :: simd_cast (self) } } # [inline] fn to_bits (self) -> Simd <$ bits_ty , N > { assert_eq ! (size_of ::< Self > () , size_of ::< Self :: Bits > ()) ; unsafe { core :: mem :: transmute_copy (& self) } } # [inline] fn from_bits (bits : Simd <$ bits_ty , N >) -> Self { assert_eq ! (size_of ::< Self > () , size_of ::< Self :: Bits > ()) ; unsafe { core :: mem :: transmute_copy (& bits) } } # [inline] fn abs (self) -> Self { unsafe { core :: intrinsics :: simd :: simd_fabs (self) } } # [inline] fn recip (self) -> Self { Self :: splat (1.0) / self } # [inline] fn to_degrees (self) -> Self { self * Self :: splat (Self :: Scalar :: to_degrees (1.)) } # [inline] fn to_radians (self) -> Self { self * Self :: splat (Self :: Scalar :: to_radians (1.)) } # [inline] fn is_sign_positive (self) -> Self :: Mask { ! self . is_sign_negative () } # [inline] fn is_sign_negative (self) -> Self :: Mask { let sign_bits = self . to_bits () & Simd :: splat ((! 0 >> 1) + 1) ; sign_bits . simd_gt (Simd :: splat (0)) } # [inline] fn is_nan (self) -> Self :: Mask { self . simd_ne (self) } # [inline] fn is_infinite (self) -> Self :: Mask { self . abs () . simd_eq (Self :: splat (Self :: Scalar :: INFINITY)) } # [inline] fn is_finite (self) -> Self :: Mask { self . abs () . simd_lt (Self :: splat (Self :: Scalar :: INFINITY)) } # [inline] fn is_subnormal (self) -> Self :: Mask { let not_zero = self . abs () . to_bits () . simd_ne (Self :: splat (0.0) . to_bits ()) ; not_zero & (self . to_bits () & Self :: splat (Self :: Scalar :: INFINITY) . to_bits ()) . simd_eq (Simd :: splat (0)) } # [inline] fn is_normal (self) -> Self :: Mask { ! (self . abs () . simd_eq (Self :: splat (0.0)) | self . is_nan () | self . is_subnormal () | self . is_infinite ()) } # [inline] fn signum (self) -> Self { self . is_nan () . select (Self :: splat (Self :: Scalar :: NAN) , Self :: splat (1.0) . copysign (self)) } # [inline] fn copysign (self , sign : Self) -> Self { let sign_bit = sign . to_bits () & Self :: splat (- 0.) . to_bits () ; let magnitude = self . to_bits () & ! Self :: splat (- 0.) . to_bits () ; Self :: from_bits (sign_bit | magnitude) } # [inline] fn simd_min (self , other : Self) -> Self { unsafe { core :: intrinsics :: simd :: simd_fmin (self , other) } } # [inline] fn simd_max (self , other : Self) -> Self { unsafe { core :: intrinsics :: simd :: simd_fmax (self , other) } } # [inline] fn simd_clamp (self , min : Self , max : Self) -> Self { assert ! (min . simd_le (max) . all () , "each element in `min` must be less than or equal to the corresponding element in `max`" ,) ; let mut x = self ; x = x . simd_lt (min) . select (min , x) ; x = x . simd_gt (max) . select (max , x) ; x } # [inline] fn reduce_sum (self) -> Self :: Scalar { if cfg ! (all (target_arch = "x86" , not (target_feature = "sse2"))) { self . as_array () . iter () . sum () } else { unsafe { core :: intrinsics :: simd :: simd_reduce_add_ordered (self , - 0.) } } } # [inline] fn reduce_product (self) -> Self :: Scalar { if cfg ! (all (target_arch = "x86" , not (target_feature = "sse2"))) { self . as_array () . iter () . product () } else { unsafe { core :: intrinsics :: simd :: simd_reduce_mul_ordered (self , 1.) } } } # [inline] fn reduce_max (self) -> Self :: Scalar { unsafe { core :: intrinsics :: simd :: simd_reduce_max (self) } } # [inline] fn reduce_min (self) -> Self :: Scalar { unsafe { core :: intrinsics :: simd :: simd_reduce_min (self) } } }) * } }}
mkitem!{impl_trait ! { f32 { bits : u32 , mask : i32 } , f64 { bits : u64 , mask : i64 } }}