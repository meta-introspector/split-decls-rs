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
mkmod!{ambiguity, { 
                getname!(ambiguity);
                getsrc!(ambiguity);
                getpath!(ambiguity);
                get_deps!(ambiguity);
                get_crates!(ambiguity);
                mkinclude!(ambiguity);
                 
            }}
mkmod!{call_kind, { 
                getname!(call_kind);
                getsrc!(call_kind);
                getpath!(call_kind);
                get_deps!(call_kind);
                get_crates!(call_kind);
                mkinclude!(call_kind);
                 
            }}
mkmod!{fulfillment_errors, { 
                getname!(fulfillment_errors);
                getsrc!(fulfillment_errors);
                getpath!(fulfillment_errors);
                get_deps!(fulfillment_errors);
                get_crates!(fulfillment_errors);
                mkinclude!(fulfillment_errors);
                 
            }}
mkmod!{on_unimplemented, { 
                getname!(on_unimplemented);
                getsrc!(on_unimplemented);
                getpath!(on_unimplemented);
                get_deps!(on_unimplemented);
                get_crates!(on_unimplemented);
                mkinclude!(on_unimplemented);
                 
            }}
mkmod!{on_unimplemented_condition, { 
                getname!(on_unimplemented_condition);
                getsrc!(on_unimplemented_condition);
                getpath!(on_unimplemented_condition);
                get_deps!(on_unimplemented_condition);
                get_crates!(on_unimplemented_condition);
                mkinclude!(on_unimplemented_condition);
                 
            }}
mkmod!{on_unimplemented_format, { 
                getname!(on_unimplemented_format);
                getsrc!(on_unimplemented_format);
                getpath!(on_unimplemented_format);
                get_deps!(on_unimplemented_format);
                get_crates!(on_unimplemented_format);
                mkinclude!(on_unimplemented_format);
                 
            }}
mkmod!{overflow, { 
                getname!(overflow);
                getsrc!(overflow);
                getpath!(overflow);
                get_deps!(overflow);
                get_crates!(overflow);
                mkinclude!(overflow);
                 
            }}
mkmod!{suggestions, { 
                getname!(suggestions);
                getsrc!(suggestions);
                getpath!(suggestions);
                get_deps!(suggestions);
                get_crates!(suggestions);
                mkinclude!(suggestions);
                 
            }}
mkuse!{use std :: { fmt , iter } ;}
mkuse!{use rustc_data_structures :: fx :: { FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_errors :: { Applicability , Diag , E0038 , E0276 , MultiSpan , struct_span_code_err } ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: intravisit :: Visitor ;}
mkuse!{use rustc_hir :: { self as hir , AmbigArg } ;}
mkuse!{use rustc_infer :: traits :: solve :: Goal ;}
mkuse!{use rustc_infer :: traits :: { DynCompatibilityViolation , Obligation , ObligationCause , ObligationCauseCode , PredicateObligation , SelectionError , } ;}
mkuse!{use rustc_middle :: ty :: print :: { PrintTraitRefExt as _ , with_no_trimmed_paths } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , ExpnKind , Span } ;}
mkuse!{use tracing :: { info , instrument } ;}
mkuse!{pub use self :: overflow :: * ;}
mkuse!{use crate :: error_reporting :: TypeErrCtxt ;}
mkuse!{use crate :: traits :: { FulfillmentError , FulfillmentErrorCode } ;}
mkitem!{mkenum!{#[derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] pub enum CandidateSimilarity { Exact { ignoring_lifetimes : bool } , Fuzzy { ignoring_lifetimes : bool } , }}}
mkitem!{mkstruct!{#[derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct ImplCandidate < 'tcx > { pub trait_ref : ty :: TraitRef < 'tcx > , pub similarity : CandidateSimilarity , impl_def_id : DefId , }}}
mkitem!{mkenum!{enum GetSafeTransmuteErrorAndReason { Silent , Default , Error { err_msg : String , safe_transmute_explanation : Option < String > } , }}}
mkitem!{mkstruct!{#[doc = " Crude way of getting back an `Expr` from a `Span`."] pub struct FindExprBySpan < 'hir > { pub span : Span , pub result : Option < & 'hir hir :: Expr < 'hir > > , pub ty_result : Option < & 'hir hir :: Ty < 'hir > > , pub include_closures : bool , pub tcx : TyCtxt < 'hir > , }}}
mkitem!{mkimpl!{impl < 'hir > FindExprBySpan < 'hir > { pub fn new (span : Span , tcx : TyCtxt < 'hir >) -> Self { Self { span , result : None , ty_result : None , tcx , include_closures : false } } }}}
mkitem!{mkimpl!{impl < 'v > Visitor < 'v > for FindExprBySpan < 'v > { type NestedFilter = rustc_middle :: hir :: nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_expr (& mut self , ex : & 'v hir :: Expr < 'v >) { if self . span == ex . span { self . result = Some (ex) ; } else { if let hir :: ExprKind :: Closure (..) = ex . kind && self . include_closures && let closure_header_sp = self . span . with_hi (ex . span . hi ()) && closure_header_sp == ex . span { self . result = Some (ex) ; } hir :: intravisit :: walk_expr (self , ex) ; } } fn visit_ty (& mut self , ty : & 'v hir :: Ty < 'v , AmbigArg >) { if self . span == ty . span { self . ty_result = Some (ty . as_unambig_ty ()) ; } else { hir :: intravisit :: walk_ty (self , ty) ; } } }}}
mkitem!{mkenum!{#[doc = " Summarizes information"] #[derive (Clone)] pub enum ArgKind { #[doc = " An argument of non-tuple type. Parameters are (name, ty)"] Arg (String , String) , #[doc = " An argument of tuple type. For a \"found\" argument, the span is"] #[doc = " the location in the source of the pattern. For an \"expected\""] #[doc = " argument, it will be None. The vector is a list of (name, ty)"] #[doc = " strings for the components of the tuple."] Tuple (Option < Span > , Vec < (String , String) >) , }}}
mkitem!{mkimpl!{impl ArgKind { fn empty () -> ArgKind { ArgKind :: Arg ("_" . to_owned () , "_" . to_owned ()) } #[doc = " Creates an `ArgKind` from the expected type of an"] #[doc = " argument. It has no name (`_`) and an optional source span."] pub fn from_expected_ty (t : Ty < '_ > , span : Option < Span >) -> ArgKind { match t . kind () { ty :: Tuple (tys) => ArgKind :: Tuple (span , tys . iter () . map (| ty | ("_" . to_owned () , ty . to_string ())) . collect :: < Vec < _ > > () ,) , _ => ArgKind :: Arg ("_" . to_owned () , t . to_string ()) , } } }}}
mkitem!{mkenum!{#[derive (Copy , Clone)] pub enum DefIdOrName { DefId (DefId) , Name (& 'static str) , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > TypeErrCtxt < 'a , 'tcx > { pub fn report_fulfillment_errors (& self , mut errors : Vec < FulfillmentError < 'tcx > > ,) -> ErrorGuaranteed { #[derive (Debug)] struct ErrorDescriptor < 'tcx > { goal : Goal < 'tcx , ty :: Predicate < 'tcx > > , index : Option < usize > , } let mut error_map : FxIndexMap < _ , Vec < _ > > = self . reported_trait_errors . borrow () . iter () . map (| (& span , goals) | { (span , goals . 0 . iter () . map (| & goal | ErrorDescriptor { goal , index : None }) . collect ()) }) . collect () ; errors . sort_by_key (| e | { let maybe_sizedness_did = match e . obligation . predicate . kind () . skip_binder () { ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (pred)) => Some (pred . def_id ()) , ty :: PredicateKind :: Clause (ty :: ClauseKind :: HostEffect (pred)) => Some (pred . def_id ()) , _ => None , } ; match e . obligation . predicate . kind () . skip_binder () { _ if maybe_sizedness_did == self . tcx . lang_items () . sized_trait () => 1 , _ if maybe_sizedness_did == self . tcx . lang_items () . meta_sized_trait () => 2 , _ if maybe_sizedness_did == self . tcx . lang_items () . pointee_sized_trait () => 3 , ty :: PredicateKind :: Coerce (_) => 4 , ty :: PredicateKind :: Clause (ty :: ClauseKind :: WellFormed (_)) => 5 , _ => 0 , } }) ; for (index , error) in errors . iter () . enumerate () { let mut span = error . obligation . cause . span ; let expn_data = span . ctxt () . outer_expn_data () ; if let ExpnKind :: Desugaring (_) = expn_data . kind { span = expn_data . call_site ; } error_map . entry (span) . or_default () . push (ErrorDescriptor { goal : error . obligation . as_goal () , index : Some (index) }) ; } let mut is_suppressed = vec ! [false ; errors . len ()] ; for (_ , error_set) in error_map . iter () { for error in error_set { if let Some (index) = error . index { for error2 in error_set { if error2 . index . is_some_and (| index2 | is_suppressed [index2]) { continue ; } if self . error_implies (error2 . goal , error . goal) && ! (error2 . index >= error . index && self . error_implies (error . goal , error2 . goal)) { info ! ("skipping {:?} (implied by {:?})" , error , error2) ; is_suppressed [index] = true ; break ; } } } } } let mut reported = None ; for from_expansion in [false , true] { for (error , suppressed) in iter :: zip (& errors , & is_suppressed) { if ! suppressed && error . obligation . cause . span . from_expansion () == from_expansion { let guar = self . report_fulfillment_error (error) ; self . infcx . set_tainted_by_errors (guar) ; reported = Some (guar) ; let mut span = error . obligation . cause . span ; let expn_data = span . ctxt () . outer_expn_data () ; if let ExpnKind :: Desugaring (_) = expn_data . kind { span = expn_data . call_site ; } self . reported_trait_errors . borrow_mut () . entry (span) . or_insert_with (| | (vec ! [] , guar)) . 0 . push (error . obligation . as_goal ()) ; } } } reported . unwrap_or_else (| | self . dcx () . delayed_bug ("failed to report fulfillment errors")) } #[instrument (skip (self) , level = "debug")] fn report_fulfillment_error (& self , error : & FulfillmentError < 'tcx >) -> ErrorGuaranteed { let mut error = FulfillmentError { obligation : error . obligation . clone () , code : error . code . clone () , root_obligation : error . root_obligation . clone () , } ; if matches ! (error . code , FulfillmentErrorCode :: Select (crate :: traits :: SelectionError :: Unimplemented) | FulfillmentErrorCode :: Project (_)) && self . apply_do_not_recommend (& mut error . obligation) { error . code = FulfillmentErrorCode :: Select (SelectionError :: Unimplemented) ; } match error . code { FulfillmentErrorCode :: Select (ref selection_error) => self . report_selection_error (error . obligation . clone () , & error . root_obligation , selection_error ,) , FulfillmentErrorCode :: Project (ref e) => { self . report_projection_error (& error . obligation , e) } FulfillmentErrorCode :: Ambiguity { overflow : None } => { self . maybe_report_ambiguity (& error . obligation) } FulfillmentErrorCode :: Ambiguity { overflow : Some (suggest_increasing_limit) } => { self . report_overflow_no_abort (error . obligation . clone () , suggest_increasing_limit) } FulfillmentErrorCode :: Subtype (ref expected_found , ref err) => self . report_mismatched_types (& error . obligation . cause , error . obligation . param_env , expected_found . expected , expected_found . found , * err ,) . emit () , FulfillmentErrorCode :: ConstEquate (ref expected_found , ref err) => { let mut diag = self . report_mismatched_consts (& error . obligation . cause , error . obligation . param_env , expected_found . expected , expected_found . found , * err ,) ; let code = error . obligation . cause . code () . peel_derives () . peel_match_impls () ; if let ObligationCauseCode :: WhereClause (..) | ObligationCauseCode :: WhereClauseInExpr (..) = code { self . note_obligation_cause_code (error . obligation . cause . body_id , & mut diag , error . obligation . predicate , error . obligation . param_env , code , & mut vec ! [] , & mut Default :: default () ,) ; } diag . emit () } FulfillmentErrorCode :: Cycle (ref cycle) => self . report_overflow_obligation_cycle (cycle) , } } }}}

macro_rules! to_pretty_impl_header_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_pretty_impl_header in module {}", module_path!());
    };
}

mkfn!{
    to_pretty_impl_header_introspect!();
    #[doc = " Recovers the \"impl X for Y\" signature from `impl_def_id` and returns it as a"] #[doc = " string."] pub (crate) fn to_pretty_impl_header (tcx : TyCtxt < '_ > , impl_def_id : DefId) -> Option < String > { use std :: fmt :: Write ; let trait_ref = tcx . impl_trait_ref (impl_def_id) ? . instantiate_identity () ; let mut w = "impl" . to_owned () ; #[derive (Debug , Default)] struct SizednessFound { sized : bool , meta_sized : bool , } let mut types_with_sizedness_bounds = FxIndexMap :: < _ , SizednessFound > :: default () ; let args = ty :: GenericArgs :: identity_for_item (tcx , impl_def_id) ; let arg_names = args . iter () . map (| k | k . to_string ()) . filter (| k | k != "'_") . collect :: < Vec < _ > > () ; if ! arg_names . is_empty () { w . push ('<') ; w . push_str (& arg_names . join (", ")) ; w . push ('>') ; for ty in args . types () { types_with_sizedness_bounds . insert (ty , SizednessFound :: default ()) ; } } write ! (w , " {}{} for {}" , tcx . impl_polarity (impl_def_id) . as_str () , trait_ref . print_only_trait_path () , tcx . type_of (impl_def_id) . instantiate_identity ()) . unwrap () ; let predicates = tcx . predicates_of (impl_def_id) . predicates ; let mut pretty_predicates = Vec :: with_capacity (predicates . len ()) ; let sized_trait = tcx . lang_items () . sized_trait () ; let meta_sized_trait = tcx . lang_items () . meta_sized_trait () ; for (p , _) in predicates { if let Some (trait_clause) = p . as_trait_clause () { let self_ty = trait_clause . self_ty () . skip_binder () ; let sizedness_of = types_with_sizedness_bounds . entry (self_ty) . or_default () ; if Some (trait_clause . def_id ()) == sized_trait { sizedness_of . sized = true ; continue ; } else if Some (trait_clause . def_id ()) == meta_sized_trait { sizedness_of . meta_sized = true ; continue ; } } pretty_predicates . push (p . to_string ()) ; } for (ty , sizedness) in types_with_sizedness_bounds { if ! tcx . features () . sized_hierarchy () { if sizedness . sized { } else { pretty_predicates . push (format ! ("{ty}: ?Sized")) ; } } else { if sizedness . sized { pretty_predicates . push (format ! ("{ty}: Sized")) ; } else if sizedness . meta_sized { pretty_predicates . push (format ! ("{ty}: MetaSized")) ; } else { pretty_predicates . push (format ! ("{ty}: PointeeSized")) ; } } } if ! pretty_predicates . is_empty () { write ! (w , "\n  where {}" , pretty_predicates . join (", ")) . unwrap () ; } w . push (';') ; Some (w) }
}
mkitem!{mkimpl!{impl < 'a , 'tcx > TypeErrCtxt < 'a , 'tcx > { pub fn report_extra_impl_obligation (& self , error_span : Span , impl_item_def_id : LocalDefId , trait_item_def_id : DefId , requirement : & dyn fmt :: Display ,) -> Diag < 'a > { let mut err = struct_span_code_err ! (self . dcx () , error_span , E0276 , "impl has stricter requirements than trait") ; if ! self . tcx . is_impl_trait_in_trait (trait_item_def_id) { if let Some (span) = self . tcx . hir_span_if_local (trait_item_def_id) { let item_name = self . tcx . item_name (impl_item_def_id . to_def_id ()) ; err . span_label (span , format ! ("definition of `{item_name}` from trait")) ; } } err . span_label (error_span , format ! ("impl has extra requirement {requirement}")) ; err } }}}

macro_rules! report_dyn_incompatibility_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_dyn_incompatibility in module {}", module_path!());
    };
}

mkfn!{
    report_dyn_incompatibility_introspect!();
    pub fn report_dyn_incompatibility < 'tcx > (tcx : TyCtxt < 'tcx > , span : Span , hir_id : Option < hir :: HirId > , trait_def_id : DefId , violations : & [DynCompatibilityViolation] ,) -> Diag < 'tcx > { let trait_str = tcx . def_path_str (trait_def_id) ; let trait_span = tcx . hir_get_if_local (trait_def_id) . and_then (| node | match node { hir :: Node :: Item (item) => match item . kind { hir :: ItemKind :: Trait (_ , _ , _ , ident , ..) | hir :: ItemKind :: TraitAlias (ident , _ , _) => { Some (ident . span) } _ => unreachable ! () , } , _ => None , }) ; let mut err = struct_span_code_err ! (tcx . dcx () , span , E0038 , "the {} `{}` is not dyn compatible" , tcx . def_descr (trait_def_id) , trait_str) ; err . span_label (span , format ! ("`{trait_str}` is not dyn compatible")) ; attempt_dyn_to_impl_suggestion (tcx , hir_id , & mut err) ; let mut reported_violations = FxIndexSet :: default () ; let mut multi_span = vec ! [] ; let mut messages = vec ! [] ; for violation in violations { if let DynCompatibilityViolation :: SizedSelf (sp) = & violation && ! sp . is_empty () { reported_violations . insert (DynCompatibilityViolation :: SizedSelf (vec ! [] . into ())) ; } if reported_violations . insert (violation . clone ()) { let spans = violation . spans () ; let msg = if trait_span . is_none () || spans . is_empty () { format ! ("the trait is not dyn compatible because {}" , violation . error_msg ()) } else { format ! ("...because {}" , violation . error_msg ()) } ; if spans . is_empty () { err . note (msg) ; } else { for span in spans { multi_span . push (span) ; messages . push (msg . clone ()) ; } } } } let has_multi_span = ! multi_span . is_empty () ; let mut note_span = MultiSpan :: from_spans (multi_span . clone ()) ; if let (Some (trait_span) , true) = (trait_span , has_multi_span) { note_span . push_span_label (trait_span , "this trait is not dyn compatible...") ; } for (span , msg) in iter :: zip (multi_span , messages) { note_span . push_span_label (span , msg) ; } err . span_note (note_span , "for a trait to be dyn compatible it needs to allow building a vtable\n\
        for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>" ,) ; if trait_span . is_some () { let mut potential_solutions : Vec < _ > = reported_violations . into_iter () . map (| violation | violation . solution ()) . collect () ; potential_solutions . sort () ; potential_solutions . dedup () ; for solution in potential_solutions { solution . add_to (& mut err) ; } } attempt_dyn_to_enum_suggestion (tcx , trait_def_id , & * trait_str , & mut err) ; err }
}

macro_rules! attempt_dyn_to_enum_suggestion_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function attempt_dyn_to_enum_suggestion in module {}", module_path!());
    };
}

mkfn!{
    attempt_dyn_to_enum_suggestion_introspect!();
    #[doc = " Attempt to suggest converting the `dyn Trait` argument to an enumeration"] #[doc = " over the types that implement `Trait`."] fn attempt_dyn_to_enum_suggestion (tcx : TyCtxt < '_ > , trait_def_id : DefId , trait_str : & str , err : & mut Diag < '_ > ,) { let impls_of = tcx . trait_impls_of (trait_def_id) ; if ! impls_of . blanket_impls () . is_empty () { return ; } let concrete_impls : Option < Vec < Ty < '_ > > > = impls_of . non_blanket_impls () . values () . flatten () . map (| impl_id | { let Some (impl_type) = tcx . type_of (* impl_id) . no_bound_vars () else { return None } ; match impl_type . kind () { ty :: Str | ty :: Slice (_) | ty :: Dynamic (_ , _ , ty :: DynKind :: Dyn) => { return None ; } _ => { } } Some (impl_type) }) . collect () ; let Some (concrete_impls) = concrete_impls else { return } ; const MAX_IMPLS_TO_SUGGEST_CONVERTING_TO_ENUM : usize = 9 ; if concrete_impls . is_empty () || concrete_impls . len () > MAX_IMPLS_TO_SUGGEST_CONVERTING_TO_ENUM { return ; } let externally_visible = if let Some (def_id) = trait_def_id . as_local () { tcx . resolutions (()) . effective_visibilities . is_exported (def_id) } else { false } ; if let [only_impl] = & concrete_impls [..] { let within = if externally_visible { " within this crate" } else { "" } ; err . help (with_no_trimmed_paths ! (format ! ("only type `{only_impl}` implements `{trait_str}`{within}; \
            consider using it directly instead."))) ; } else { let types = concrete_impls . iter () . map (| t | with_no_trimmed_paths ! (format ! ("  {}" , t))) . collect :: < Vec < String > > () . join ("\n") ; err . help (format ! ("the following types implement `{trait_str}`:\n\
             {types}\n\
             consider defining an enum where each variant holds one of these types,\n\
             implementing `{trait_str}` for this new enum and using it instead" ,)) ; } if externally_visible { err . note (format ! ("`{trait_str}` may be implemented in other crates; if you want to support your users \
             passing their own types here, you can't refer to a specific type" ,)) ; } }
}

macro_rules! attempt_dyn_to_impl_suggestion_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function attempt_dyn_to_impl_suggestion in module {}", module_path!());
    };
}

mkfn!{
    attempt_dyn_to_impl_suggestion_introspect!();
    #[doc = " Attempt to suggest that a `dyn Trait` argument or return type be converted"] #[doc = " to use `impl Trait`."] fn attempt_dyn_to_impl_suggestion (tcx : TyCtxt < '_ > , hir_id : Option < hir :: HirId > , err : & mut Diag < '_ >) { let Some (hir_id) = hir_id else { return } ; let hir :: Node :: Ty (ty) = tcx . hir_node (hir_id) else { return } ; let hir :: TyKind :: TraitObject ([trait_ref , ..] , ..) = ty . kind else { return } ; let Some ((_id , first_non_type_parent_node)) = tcx . hir_parent_iter (hir_id) . find (| (_id , node) | ! matches ! (node , hir :: Node :: Ty (_))) else { return ; } ; if first_non_type_parent_node . fn_sig () . is_none () { return ; } err . span_suggestion_verbose (ty . span . until (trait_ref . span) , "consider using an opaque type instead" , "impl " , Applicability :: MaybeIncorrect ,) ; }
}