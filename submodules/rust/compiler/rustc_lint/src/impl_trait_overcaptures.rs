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
mkuse!{use std :: assert_matches :: debug_assert_matches ;}
mkuse!{use std :: cell :: LazyCell ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_data_structures :: unord :: UnordSet ;}
mkuse!{use rustc_errors :: { LintDiagnostic , Subdiagnostic } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_infer :: infer :: outlives :: env :: OutlivesEnvironment ;}
mkuse!{use rustc_macros :: LintDiagnostic ;}
mkuse!{use rustc_middle :: middle :: resolve_bound_vars :: ResolvedArg ;}
mkuse!{use rustc_middle :: ty :: relate :: { Relate , RelateResult , TypeRelation , structurally_relate_consts , structurally_relate_tys , } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeSuperVisitable , TypeVisitable , TypeVisitableExt , TypeVisitor , } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_session :: lint :: FutureIncompatibilityReason ;}
mkuse!{use rustc_session :: { declare_lint , declare_lint_pass } ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: { Span , Symbol } ;}
mkuse!{use rustc_trait_selection :: errors :: { AddPreciseCapturingForOvercapture , impl_trait_overcapture_suggestion , } ;}
mkuse!{use rustc_trait_selection :: regions :: OutlivesEnvironmentBuildExt ;}
mkuse!{use rustc_trait_selection :: traits :: ObligationCtxt ;}
mkuse!{use crate :: { LateContext , LateLintPass , fluent_generated as fluent } ;}
mkitem!{declare_lint ! { #[doc = " The `impl_trait_overcaptures` lint warns against cases where lifetime"] #[doc = " capture behavior will differ in edition 2024."] #[doc = ""] #[doc = " In the 2024 edition, `impl Trait`s will capture all lifetimes in scope,"] #[doc = " rather than just the lifetimes that are mentioned in the bounds of the type."] #[doc = " Often these sets are equal, but if not, it means that the `impl Trait` may"] #[doc = " cause erroneous borrow-checker errors."] #[doc = ""] #[doc = " ### Example"] #[doc = ""] #[doc = " ```rust,compile_fail,edition2021"] #[doc = " # #![deny(impl_trait_overcaptures)]"] #[doc = " # use std::fmt::Display;"] #[doc = " let mut x = vec![];"] #[doc = " x.push(1);"] #[doc = ""] #[doc = " fn test(x: &Vec<i32>) -> impl Display {"] #[doc = "     x[0]"] #[doc = " }"] #[doc = ""] #[doc = " let element = test(&x);"] #[doc = " x.push(2);"] #[doc = " println!(\"{element}\");"] #[doc = " ```"] #[doc = ""] #[doc = " {{produces}}"] #[doc = ""] #[doc = " ### Explanation"] #[doc = ""] #[doc = " In edition < 2024, the returned `impl Display` doesn't capture the"] #[doc = " lifetime from the `&Vec<i32>`, so the vector can be mutably borrowed"] #[doc = " while the `impl Display` is live."] #[doc = ""] #[doc = " To fix this, we can explicitly state that the `impl Display` doesn't"] #[doc = " capture any lifetimes, using `impl Display + use<>`."] pub IMPL_TRAIT_OVERCAPTURES , Allow , "`impl Trait` will capture more lifetimes than possibly intended in edition 2024" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionSemanticsChange (Edition :: Edition2024) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2024/rpit-lifetime-capture.html>" , } ; }}
mkitem!{declare_lint ! { #[doc = " The `impl_trait_redundant_captures` lint warns against cases where use of the"] #[doc = " precise capturing `use<...>` syntax is not needed."] #[doc = ""] #[doc = " In the 2024 edition, `impl Trait`s will capture all lifetimes in scope."] #[doc = " If precise-capturing `use<...>` syntax is used, and the set of parameters"] #[doc = " that are captures are *equal* to the set of parameters in scope, then"] #[doc = " the syntax is redundant, and can be removed."] #[doc = ""] #[doc = " ### Example"] #[doc = ""] #[doc = " ```rust,edition2024,compile_fail"] #[doc = " # #![deny(impl_trait_redundant_captures)]"] #[doc = " fn test<'a>(x: &'a i32) -> impl Sized + use<'a> { x }"] #[doc = " ```"] #[doc = ""] #[doc = " {{produces}}"] #[doc = ""] #[doc = " ### Explanation"] #[doc = ""] #[doc = " To fix this, remove the `use<'a>`, since the lifetime is already captured"] #[doc = " since it is in scope."] pub IMPL_TRAIT_REDUNDANT_CAPTURES , Allow , "redundant precise-capturing `use<...>` syntax on an `impl Trait`" , }}
mkitem!{declare_lint_pass ! (#[doc = " Lint for opaque types that will begin capturing in-scope but unmentioned lifetimes"] #[doc = " in edition 2024."] ImplTraitOvercaptures => [IMPL_TRAIT_OVERCAPTURES , IMPL_TRAIT_REDUNDANT_CAPTURES]) ;}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for ImplTraitOvercaptures { fn check_item (& mut self , cx : & LateContext < 'tcx > , it : & 'tcx hir :: Item < 'tcx >) { match & it . kind { hir :: ItemKind :: Fn { .. } => check_fn (cx . tcx , it . owner_id . def_id) , _ => { } } } fn check_impl_item (& mut self , cx : & LateContext < 'tcx > , it : & 'tcx hir :: ImplItem < 'tcx >) { match & it . kind { hir :: ImplItemKind :: Fn (_ , _) => check_fn (cx . tcx , it . owner_id . def_id) , _ => { } } } fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , it : & 'tcx hir :: TraitItem < 'tcx >) { match & it . kind { hir :: TraitItemKind :: Fn (_ , _) => check_fn (cx . tcx , it . owner_id . def_id) , _ => { } } } }}}
mkitem!{mkenum!{#[derive (PartialEq , Eq , Hash , Debug , Copy , Clone)] enum ParamKind { Early (Symbol , u32) , Free (DefId) , Late , }}}

macro_rules! check_fn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_fn in module {}", module_path!());
    };
}

mkfn!{
    check_fn_introspect!();
    fn check_fn (tcx : TyCtxt < '_ > , parent_def_id : LocalDefId) { let sig = tcx . fn_sig (parent_def_id) . instantiate_identity () ; let mut in_scope_parameters = FxIndexMap :: default () ; let mut current_def_id = Some (parent_def_id . to_def_id ()) ; while let Some (def_id) = current_def_id { let generics = tcx . generics_of (def_id) ; for param in & generics . own_params { in_scope_parameters . insert (param . def_id , ParamKind :: Early (param . name , param . index)) ; } current_def_id = generics . parent ; } for bound_var in sig . bound_vars () { let ty :: BoundVariableKind :: Region (ty :: BoundRegionKind :: Named (def_id)) = bound_var else { span_bug ! (tcx . def_span (parent_def_id) , "unexpected non-lifetime binder on fn sig") ; } ; in_scope_parameters . insert (def_id , ParamKind :: Free (def_id)) ; } let sig = tcx . liberate_late_bound_regions (parent_def_id . to_def_id () , sig) ; sig . visit_with (& mut VisitOpaqueTypes { tcx , parent_def_id , in_scope_parameters , seen : Default :: default () , variances : LazyCell :: new (| | { let mut functional_variances = FunctionalVariances { tcx , variances : FxHashMap :: default () , ambient_variance : ty :: Covariant , generics : tcx . generics_of (parent_def_id) , } ; functional_variances . relate (sig , sig) . unwrap () ; functional_variances . variances }) , outlives_env : LazyCell :: new (| | { let typing_env = ty :: TypingEnv :: non_body_analysis (tcx , parent_def_id) ; let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let ocx = ObligationCtxt :: new (& infcx) ; let assumed_wf_tys = ocx . assumed_wf_types (param_env , parent_def_id) . unwrap_or_default () ; OutlivesEnvironment :: new (& infcx , parent_def_id , param_env , assumed_wf_tys) }) , }) ; }
}
mkitem!{mkstruct!{struct VisitOpaqueTypes < 'tcx , VarFn , OutlivesFn > { tcx : TyCtxt < 'tcx > , parent_def_id : LocalDefId , in_scope_parameters : FxIndexMap < DefId , ParamKind > , variances : LazyCell < FxHashMap < DefId , ty :: Variance > , VarFn > , outlives_env : LazyCell < OutlivesEnvironment < 'tcx > , OutlivesFn > , seen : FxIndexSet < LocalDefId > , }}}
mkitem!{mkimpl!{impl < 'tcx , VarFn , OutlivesFn > TypeVisitor < TyCtxt < 'tcx > > for VisitOpaqueTypes < 'tcx , VarFn , OutlivesFn > where VarFn : FnOnce () -> FxHashMap < DefId , ty :: Variance > , OutlivesFn : FnOnce () -> OutlivesEnvironment < 'tcx > , { fn visit_binder < T : TypeVisitable < TyCtxt < 'tcx > > > (& mut self , t : & ty :: Binder < 'tcx , T >) { let mut added = vec ! [] ; for arg in t . bound_vars () { let arg : ty :: BoundVariableKind = arg ; match arg { ty :: BoundVariableKind :: Region (ty :: BoundRegionKind :: Named (def_id)) | ty :: BoundVariableKind :: Ty (ty :: BoundTyKind :: Param (def_id)) => { added . push (def_id) ; let unique = self . in_scope_parameters . insert (def_id , ParamKind :: Late) ; assert_eq ! (unique , None) ; } _ => { self . tcx . dcx () . span_delayed_bug (self . tcx . def_span (self . parent_def_id) , format ! ("unsupported bound variable kind: {arg:?}") ,) ; } } } t . super_visit_with (self) ; for arg in added . into_iter () . rev () { self . in_scope_parameters . shift_remove (& arg) ; } } fn visit_ty (& mut self , t : Ty < 'tcx >) { if ! t . has_aliases () { return ; } if let ty :: Alias (ty :: Projection , opaque_ty) = * t . kind () && self . tcx . is_impl_trait_in_trait (opaque_ty . def_id) { self . tcx . type_of (opaque_ty . def_id) . instantiate (self . tcx , opaque_ty . args) . visit_with (self) } else if let ty :: Alias (ty :: Opaque , opaque_ty) = * t . kind () && let Some (opaque_def_id) = opaque_ty . def_id . as_local () && self . seen . insert (opaque_def_id) && let opaque = self . tcx . hir_node_by_def_id (opaque_def_id) . expect_opaque_ty () && let hir :: OpaqueTyOrigin :: FnReturn { parent , .. } | hir :: OpaqueTyOrigin :: AsyncFn { parent , .. } = opaque . origin && parent == self . parent_def_id { let opaque_span = self . tcx . def_span (opaque_def_id) ; let new_capture_rules = opaque_span . at_least_rust_2024 () ; if ! new_capture_rules && ! opaque . bounds . iter () . any (| bound | matches ! (bound , hir :: GenericBound :: Use (..))) { let mut captured = FxIndexSet :: default () ; let mut captured_regions = FxIndexSet :: default () ; let variances = self . tcx . variances_of (opaque_def_id) ; let mut current_def_id = Some (opaque_def_id . to_def_id ()) ; while let Some (def_id) = current_def_id { let generics = self . tcx . generics_of (def_id) ; for param in & generics . own_params { if variances [param . index as usize] != ty :: Invariant { continue ; } let arg = opaque_ty . args [param . index as usize] ; captured . insert (extract_def_id_from_arg (self . tcx , generics , arg)) ; captured_regions . extend (arg . as_region ()) ; } current_def_id = generics . parent ; } let mut uncaptured_args : FxIndexSet < _ > = self . in_scope_parameters . iter () . filter (| & (def_id , _) | ! captured . contains (def_id)) . collect () ; uncaptured_args . retain (| & (def_id , kind) | { let Some (ty :: Bivariant | ty :: Contravariant) = self . variances . get (def_id) else { return true ; } ; debug_assert_matches ! (self . tcx . def_kind (def_id) , DefKind :: LifetimeParam) ; let uncaptured = match * kind { ParamKind :: Early (name , index) => ty :: Region :: new_early_param (self . tcx , ty :: EarlyParamRegion { name , index } ,) , ParamKind :: Free (def_id) => ty :: Region :: new_late_param (self . tcx , self . parent_def_id . to_def_id () , ty :: LateParamRegionKind :: Named (def_id) ,) , ParamKind :: Late => return true , } ; ! captured_regions . iter () . any (| r | { self . outlives_env . free_region_map () . sub_free_regions (self . tcx , * r , uncaptured) }) }) ; if ! uncaptured_args . is_empty () { let suggestion = impl_trait_overcapture_suggestion (self . tcx , opaque_def_id , self . parent_def_id , captured ,) ; let uncaptured_spans : Vec < _ > = uncaptured_args . into_iter () . map (| (def_id , _) | self . tcx . def_span (def_id)) . collect () ; self . tcx . emit_node_span_lint (IMPL_TRAIT_OVERCAPTURES , self . tcx . local_def_id_to_hir_id (opaque_def_id) , opaque_span , ImplTraitOvercapturesLint { self_ty : t , num_captured : uncaptured_spans . len () , uncaptured_spans , suggestion , } ,) ; } } if new_capture_rules && let Some ((captured_args , capturing_span)) = opaque . bounds . iter () . find_map (| bound | match * bound { hir :: GenericBound :: Use (a , s) => Some ((a , s)) , _ => None , }) { let mut explicitly_captured = UnordSet :: default () ; for arg in captured_args { match self . tcx . named_bound_var (arg . hir_id ()) { Some (ResolvedArg :: EarlyBound (def_id) | ResolvedArg :: LateBound (_ , _ , def_id) ,) => { if self . tcx . def_kind (self . tcx . local_parent (def_id)) == DefKind :: OpaqueTy { let def_id = self . tcx . map_opaque_lifetime_to_parent_lifetime (def_id) . opt_param_def_id (self . tcx , self . parent_def_id . to_def_id ()) . expect ("variable should have been duplicated from parent") ; explicitly_captured . insert (def_id) ; } else { explicitly_captured . insert (def_id . to_def_id ()) ; } } _ => { self . tcx . dcx () . span_delayed_bug (self . tcx . hir_span (arg . hir_id ()) , "no valid for captured arg" ,) ; } } } if self . in_scope_parameters . iter () . all (| (def_id , _) | explicitly_captured . contains (def_id)) { self . tcx . emit_node_span_lint (IMPL_TRAIT_REDUNDANT_CAPTURES , self . tcx . local_def_id_to_hir_id (opaque_def_id) , opaque_span , ImplTraitRedundantCapturesLint { capturing_span } ,) ; } } for clause in self . tcx . item_bounds (opaque_ty . def_id) . iter_instantiated (self . tcx , opaque_ty . args) { clause . visit_with (self) } } t . super_visit_with (self) ; } }}}
mkitem!{mkstruct!{struct ImplTraitOvercapturesLint < 'tcx > { uncaptured_spans : Vec < Span > , self_ty : Ty < 'tcx > , num_captured : usize , suggestion : Option < AddPreciseCapturingForOvercapture > , }}}
mkitem!{mkimpl!{impl < 'a > LintDiagnostic < 'a , () > for ImplTraitOvercapturesLint < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut rustc_errors :: Diag < 'a , () >) { diag . primary_message (fluent :: lint_impl_trait_overcaptures) ; diag . arg ("self_ty" , self . self_ty . to_string ()) . arg ("num_captured" , self . num_captured) . span_note (self . uncaptured_spans , fluent :: lint_note) . note (fluent :: lint_note2) ; if let Some (suggestion) = self . suggestion { suggestion . add_to_diag (diag) ; } } }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (lint_impl_trait_redundant_captures)] struct ImplTraitRedundantCapturesLint { #[suggestion (lint_suggestion , code = "" , applicability = "machine-applicable")] capturing_span : Span , }}}

macro_rules! extract_def_id_from_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extract_def_id_from_arg in module {}", module_path!());
    };
}

mkfn!{
    extract_def_id_from_arg_introspect!();
    fn extract_def_id_from_arg < 'tcx > (tcx : TyCtxt < 'tcx > , generics : & 'tcx ty :: Generics , arg : ty :: GenericArg < 'tcx > ,) -> DefId { match arg . kind () { ty :: GenericArgKind :: Lifetime (re) => match re . kind () { ty :: ReEarlyParam (ebr) => generics . region_param (ebr , tcx) . def_id , ty :: ReBound (_ , ty :: BoundRegion { kind : ty :: BoundRegionKind :: Named (def_id) , .. }) | ty :: ReLateParam (ty :: LateParamRegion { scope : _ , kind : ty :: LateParamRegionKind :: Named (def_id) , }) => def_id , _ => unreachable ! () , } , ty :: GenericArgKind :: Type (ty) => { let ty :: Param (param_ty) = * ty . kind () else { bug ! () ; } ; generics . type_param (param_ty , tcx) . def_id } ty :: GenericArgKind :: Const (ct) => { let ty :: ConstKind :: Param (param_ct) = ct . kind () else { bug ! () ; } ; generics . const_param (param_ct , tcx) . def_id } } }
}
mkitem!{mkstruct!{#[doc = " Computes the variances of regions that appear in the type, but considering"] #[doc = " late-bound regions too, which don't have their variance computed usually."] #[doc = ""] #[doc = " Like generalization, this is a unary operation implemented on top of the binary"] #[doc = " relation infrastructure, mostly because it's much easier to have the relation"] #[doc = " track the variance for you, rather than having to do it yourself."] struct FunctionalVariances < 'tcx > { tcx : TyCtxt < 'tcx > , variances : FxHashMap < DefId , ty :: Variance > , ambient_variance : ty :: Variance , generics : & 'tcx ty :: Generics , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeRelation < TyCtxt < 'tcx > > for FunctionalVariances < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn relate_with_variance < T : Relate < TyCtxt < 'tcx > > > (& mut self , variance : ty :: Variance , _ : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , a : T , b : T ,) -> RelateResult < 'tcx , T > { let old_variance = self . ambient_variance ; self . ambient_variance = self . ambient_variance . xform (variance) ; self . relate (a , b) . unwrap () ; self . ambient_variance = old_variance ; Ok (a) } fn tys (& mut self , a : Ty < 'tcx > , b : Ty < 'tcx >) -> RelateResult < 'tcx , Ty < 'tcx > > { structurally_relate_tys (self , a , b) . unwrap () ; Ok (a) } fn regions (& mut self , a : ty :: Region < 'tcx > , _ : ty :: Region < 'tcx > ,) -> RelateResult < 'tcx , ty :: Region < 'tcx > > { let def_id = match a . kind () { ty :: ReEarlyParam (ebr) => self . generics . region_param (ebr , self . tcx) . def_id , ty :: ReBound (_ , ty :: BoundRegion { kind : ty :: BoundRegionKind :: Named (def_id) , .. }) | ty :: ReLateParam (ty :: LateParamRegion { scope : _ , kind : ty :: LateParamRegionKind :: Named (def_id) , }) => def_id , _ => { return Ok (a) ; } } ; if let Some (variance) = self . variances . get_mut (& def_id) { * variance = unify (* variance , self . ambient_variance) ; } else { self . variances . insert (def_id , self . ambient_variance) ; } Ok (a) } fn consts (& mut self , a : ty :: Const < 'tcx > , b : ty :: Const < 'tcx > ,) -> RelateResult < 'tcx , ty :: Const < 'tcx > > { structurally_relate_consts (self , a , b) . unwrap () ; Ok (a) } fn binders < T > (& mut self , a : ty :: Binder < 'tcx , T > , b : ty :: Binder < 'tcx , T > ,) -> RelateResult < 'tcx , ty :: Binder < 'tcx , T > > where T : Relate < TyCtxt < 'tcx > > , { self . relate (a . skip_binder () , b . skip_binder ()) . unwrap () ; Ok (a) } }}}

macro_rules! unify_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unify in module {}", module_path!());
    };
}

mkfn!{
    unify_introspect!();
    #[doc = " What is the variance that satisfies the two variances?"] fn unify (a : ty :: Variance , b : ty :: Variance) -> ty :: Variance { match (a , b) { (ty :: Bivariant , other) | (other , ty :: Bivariant) => other , (ty :: Invariant , _) | (_ , ty :: Invariant) => ty :: Invariant , (ty :: Contravariant , ty :: Covariant) | (ty :: Covariant , ty :: Contravariant) => ty :: Invariant , (ty :: Contravariant , ty :: Contravariant) => ty :: Contravariant , (ty :: Covariant , ty :: Covariant) => ty :: Covariant , } }
}