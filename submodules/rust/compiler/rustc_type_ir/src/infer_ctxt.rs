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
mkuse!{use derive_where :: derive_where ;}
mkuse!{# [cfg (feature = "nightly")] use rustc_macros :: { Decodable_NoContext , Encodable_NoContext , HashStable_NoContext } ;}
mkuse!{use crate :: fold :: TypeFoldable ;}
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: relate :: RelateResult ;}
mkuse!{use crate :: relate :: combine :: PredicateEmittingRelation ;}
mkuse!{use crate :: { self as ty , Interner } ;}
mkitem!{mkenum!{# [doc = " The current typing mode of an inference context. We unfortunately have some"] # [doc = " slightly different typing rules depending on the current context. See the"] # [doc = " doc comment for each variant for how and why they are used."] # [doc = ""] # [doc = " In most cases you can get the correct typing mode automatically via:"] # [doc = " - `mir::Body::typing_mode`"] # [doc = " - `rustc_lint::LateContext::typing_mode`"] # [doc = ""] # [doc = " If neither of these functions are available, feel free to reach out to"] # [doc = " t-types for help."] # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub enum TypingMode < I : Interner > { # [doc = " When checking whether impls overlap, we check whether any obligations"] # [doc = " are guaranteed to never hold when unifying the impls. This requires us"] # [doc = " to be complete: we must never fail to prove something which may actually"] # [doc = " hold."] # [doc = ""] # [doc = " In this typing mode we bail with ambiguity in case its not knowable"] # [doc = " whether a trait goal may hold, e.g. because the trait may get implemented"] # [doc = " in a downstream or sibling crate."] # [doc = ""] # [doc = " We also have to be careful when generalizing aliases inside of higher-ranked"] # [doc = " types to not unnecessarily constrain any inference variables."] Coherence , # [doc = " Analysis includes type inference, checking that items are well-formed, and"] # [doc = " pretty much everything else which may emit proper type errors to the user."] # [doc = ""] # [doc = " We only normalize opaque types which may get defined by the current body,"] # [doc = " which are stored in `defining_opaque_types`."] # [doc = ""] # [doc = " We also refuse to project any associated type that is marked `default`."] # [doc = " Non-`default` (\"final\") types are always projected. This is necessary in"] # [doc = " general for soundness of specialization. However, we *could* allow projections"] # [doc = " in fully-monomorphic cases. We choose not to, because we prefer for `default type`"] # [doc = " to force the type definition to be treated abstractly by any consumers of the"] # [doc = " impl. Concretely, that means that the following example will"] # [doc = " fail to compile:"] # [doc = ""] # [doc = " ```compile_fail,E0308"] # [doc = " #![feature(specialization)]"] # [doc = " trait Assoc {"] # [doc = "     type Output;"] # [doc = " }"] # [doc = ""] # [doc = " impl<T> Assoc for T {"] # [doc = "     default type Output = bool;"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let x: <() as Assoc>::Output = true;"] # [doc = " }"] # [doc = " ```"] Analysis { defining_opaque_types_and_generators : I :: LocalDefIds } , # [doc = " The behavior during MIR borrowck is identical to `TypingMode::Analysis`"] # [doc = " except that the initial value for opaque types is the type computed during"] # [doc = " HIR typeck with unique unconstrained region inference variables."] # [doc = ""] # [doc = " This is currently only used with by the new solver as it results in new"] # [doc = " non-universal defining uses of opaque types, which is a breaking change."] # [doc = " See tests/ui/impl-trait/non-defining-use/as-projection-term.rs."] Borrowck { defining_opaque_types : I :: LocalDefIds } , # [doc = " Any analysis after borrowck for a given body should be able to use all the"] # [doc = " hidden types defined by borrowck, without being able to define any new ones."] # [doc = ""] # [doc = " This is currently only used by the new solver, but should be implemented in"] # [doc = " the old solver as well."] PostBorrowckAnalysis { defined_opaque_types : I :: LocalDefIds } , # [doc = " After analysis, mostly during codegen and MIR optimizations, we're able to"] # [doc = " reveal all opaque types. As the concrete type should *never* be observable"] # [doc = " directly by the user, this should not be used by checks which may expose"] # [doc = " such details to the user."] # [doc = ""] # [doc = " There are some exceptions to this as for example `layout_of` and const-evaluation"] # [doc = " always run in `PostAnalysis` mode, even when used during analysis. This exposes"] # [doc = " some information about the underlying type to users, but not the type itself."] PostAnalysis , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for TypingMode < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > TypingMode < I > { # [doc = " Analysis outside of a body does not define any opaque types."] pub fn non_body_analysis () -> TypingMode < I > { TypingMode :: Analysis { defining_opaque_types_and_generators : Default :: default () } } pub fn typeck_for_body (cx : I , body_def_id : I :: LocalDefId) -> TypingMode < I > { TypingMode :: Analysis { defining_opaque_types_and_generators : cx . opaque_types_and_coroutines_defined_by (body_def_id) , } } # [doc = " While typechecking a body, we need to be able to define the opaque"] # [doc = " types defined by that body."] # [doc = ""] # [doc = " FIXME: This will be removed because it's generally not correct to define"] # [doc = " opaques outside of HIR typeck."] pub fn analysis_in_body (cx : I , body_def_id : I :: LocalDefId) -> TypingMode < I > { TypingMode :: Analysis { defining_opaque_types_and_generators : cx . opaque_types_defined_by (body_def_id) , } } pub fn borrowck (cx : I , body_def_id : I :: LocalDefId) -> TypingMode < I > { let defining_opaque_types = cx . opaque_types_defined_by (body_def_id) ; if defining_opaque_types . is_empty () { TypingMode :: non_body_analysis () } else { TypingMode :: Borrowck { defining_opaque_types } } } pub fn post_borrowck_analysis (cx : I , body_def_id : I :: LocalDefId) -> TypingMode < I > { let defined_opaque_types = cx . opaque_types_defined_by (body_def_id) ; if defined_opaque_types . is_empty () { TypingMode :: non_body_analysis () } else { TypingMode :: PostBorrowckAnalysis { defined_opaque_types } } } }}}
mkitem!{mktrait!{# [cfg_attr (feature = "nightly" , rustc_diagnostic_item = "type_ir_infer_ctxt_like")] pub trait InferCtxtLike : Sized { type Interner : Interner ; fn cx (& self) -> Self :: Interner ; # [doc = " Whether the new trait solver is enabled. This only exists because rustc"] # [doc = " shares code between the new and old trait solvers; for all other users,"] # [doc = " this should always be true. If this is unknowingly false and you try to"] # [doc = " use the new trait solver, things will break badly."] fn next_trait_solver (& self) -> bool { true } fn typing_mode (& self) -> TypingMode < Self :: Interner > ; fn universe (& self) -> ty :: UniverseIndex ; fn create_next_universe (& self) -> ty :: UniverseIndex ; fn universe_of_ty (& self , ty : ty :: TyVid) -> Option < ty :: UniverseIndex > ; fn universe_of_lt (& self , lt : ty :: RegionVid) -> Option < ty :: UniverseIndex > ; fn universe_of_ct (& self , ct : ty :: ConstVid) -> Option < ty :: UniverseIndex > ; fn root_ty_var (& self , var : ty :: TyVid) -> ty :: TyVid ; fn sub_unification_table_root_var (& self , var : ty :: TyVid) -> ty :: TyVid ; fn root_const_var (& self , var : ty :: ConstVid) -> ty :: ConstVid ; fn opportunistic_resolve_ty_var (& self , vid : ty :: TyVid) -> < Self :: Interner as Interner > :: Ty ; fn opportunistic_resolve_int_var (& self , vid : ty :: IntVid) -> < Self :: Interner as Interner > :: Ty ; fn opportunistic_resolve_float_var (& self , vid : ty :: FloatVid ,) -> < Self :: Interner as Interner > :: Ty ; fn opportunistic_resolve_ct_var (& self , vid : ty :: ConstVid ,) -> < Self :: Interner as Interner > :: Const ; fn opportunistic_resolve_lt_var (& self , vid : ty :: RegionVid ,) -> < Self :: Interner as Interner > :: Region ; fn is_changed_arg (& self , arg : < Self :: Interner as Interner > :: GenericArg) -> bool ; fn next_region_infer (& self) -> < Self :: Interner as Interner > :: Region ; fn next_ty_infer (& self) -> < Self :: Interner as Interner > :: Ty ; fn next_const_infer (& self) -> < Self :: Interner as Interner > :: Const ; fn fresh_args_for_item (& self , def_id : < Self :: Interner as Interner > :: DefId ,) -> < Self :: Interner as Interner > :: GenericArgs ; fn instantiate_binder_with_infer < T : TypeFoldable < Self :: Interner > + Copy > (& self , value : ty :: Binder < Self :: Interner , T > ,) -> T ; fn enter_forall < T : TypeFoldable < Self :: Interner > , U > (& self , value : ty :: Binder < Self :: Interner , T > , f : impl FnOnce (T) -> U ,) -> U ; fn equate_ty_vids_raw (& self , a : ty :: TyVid , b : ty :: TyVid) ; fn sub_unify_ty_vids_raw (& self , a : ty :: TyVid , b : ty :: TyVid) ; fn equate_int_vids_raw (& self , a : ty :: IntVid , b : ty :: IntVid) ; fn equate_float_vids_raw (& self , a : ty :: FloatVid , b : ty :: FloatVid) ; fn equate_const_vids_raw (& self , a : ty :: ConstVid , b : ty :: ConstVid) ; fn instantiate_ty_var_raw < R : PredicateEmittingRelation < Self > > (& self , relation : & mut R , target_is_expected : bool , target_vid : ty :: TyVid , instantiation_variance : ty :: Variance , source_ty : < Self :: Interner as Interner > :: Ty ,) -> RelateResult < Self :: Interner , () > ; fn instantiate_int_var_raw (& self , vid : ty :: IntVid , value : ty :: IntVarValue) ; fn instantiate_float_var_raw (& self , vid : ty :: FloatVid , value : ty :: FloatVarValue) ; fn instantiate_const_var_raw < R : PredicateEmittingRelation < Self > > (& self , relation : & mut R , target_is_expected : bool , target_vid : ty :: ConstVid , source_ct : < Self :: Interner as Interner > :: Const ,) -> RelateResult < Self :: Interner , () > ; fn set_tainted_by_errors (& self , e : < Self :: Interner as Interner > :: ErrorGuaranteed) ; fn shallow_resolve (& self , ty : < Self :: Interner as Interner > :: Ty ,) -> < Self :: Interner as Interner > :: Ty ; fn shallow_resolve_const (& self , ty : < Self :: Interner as Interner > :: Const ,) -> < Self :: Interner as Interner > :: Const ; fn resolve_vars_if_possible < T > (& self , value : T) -> T where T : TypeFoldable < Self :: Interner > ; fn probe < T > (& self , probe : impl FnOnce () -> T) -> T ; fn sub_regions (& self , sub : < Self :: Interner as Interner > :: Region , sup : < Self :: Interner as Interner > :: Region , span : < Self :: Interner as Interner > :: Span ,) ; fn equate_regions (& self , a : < Self :: Interner as Interner > :: Region , b : < Self :: Interner as Interner > :: Region , span : < Self :: Interner as Interner > :: Span ,) ; fn register_ty_outlives (& self , ty : < Self :: Interner as Interner > :: Ty , r : < Self :: Interner as Interner > :: Region , span : < Self :: Interner as Interner > :: Span ,) ; type OpaqueTypeStorageEntries : OpaqueTypeStorageEntries ; fn opaque_types_storage_num_entries (& self) -> Self :: OpaqueTypeStorageEntries ; fn clone_opaque_types_lookup_table (& self ,) -> Vec < (ty :: OpaqueTypeKey < Self :: Interner > , < Self :: Interner as Interner > :: Ty) > ; fn clone_duplicate_opaque_types (& self ,) -> Vec < (ty :: OpaqueTypeKey < Self :: Interner > , < Self :: Interner as Interner > :: Ty) > ; fn clone_opaque_types_added_since (& self , prev_entries : Self :: OpaqueTypeStorageEntries ,) -> Vec < (ty :: OpaqueTypeKey < Self :: Interner > , < Self :: Interner as Interner > :: Ty) > ; fn register_hidden_type_in_storage (& self , opaque_type_key : ty :: OpaqueTypeKey < Self :: Interner > , hidden_ty : < Self :: Interner as Interner > :: Ty , span : < Self :: Interner as Interner > :: Span ,) -> Option < < Self :: Interner as Interner > :: Ty > ; fn add_duplicate_opaque_type (& self , opaque_type_key : ty :: OpaqueTypeKey < Self :: Interner > , hidden_ty : < Self :: Interner as Interner > :: Ty , span : < Self :: Interner as Interner > :: Span ,) ; fn reset_opaque_types (& self) ; }}}

macro_rules! may_use_unstable_feature_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function may_use_unstable_feature in module {}", module_path!());
    };
}

mkfn!{
    may_use_unstable_feature_introspect!();
    pub fn may_use_unstable_feature < 'a , I : Interner , Infcx > (infcx : & 'a Infcx , param_env : I :: ParamEnv , symbol : I :: Symbol ,) -> bool where Infcx : InferCtxtLike < Interner = I > , { for pred in param_env . caller_bounds () . iter () { if let ty :: ClauseKind :: UnstableFeature (sym) = pred . kind () . skip_binder () { if sym == symbol { return true ; } } } (infcx . typing_mode () == TypingMode :: PostAnalysis) || infcx . cx () . features () . feature_bound_holds_in_crate (symbol) }
}