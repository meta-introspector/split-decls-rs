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
mkuse!{use std :: iter ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_errors :: { Applicability , Diag , E0309 , E0310 , E0311 , E0803 , Subdiagnostic , struct_span_code_err , } ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: intravisit :: Visitor ;}
mkuse!{use rustc_hir :: { self as hir , ParamName } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: traits :: ObligationCauseCode ;}
mkuse!{use rustc_middle :: ty :: error :: TypeError ;}
mkuse!{use rustc_middle :: ty :: { self , IsSuggestable , Region , Ty , TyCtxt , TypeVisitableExt as _ , Upcast as _ , } ;}
mkuse!{use rustc_span :: { BytePos , ErrorGuaranteed , Span , Symbol , kw } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: ObligationCauseAsDiagArg ;}
mkuse!{use super :: nice_region_error :: find_anon_type ;}
mkuse!{use crate :: error_reporting :: TypeErrCtxt ;}
mkuse!{use crate :: error_reporting :: infer :: ObligationCauseExt ;}
mkuse!{use crate :: errors :: { self , FulfillReqLifetime , LfBoundNotSatisfied , OutlivesBound , OutlivesContent , RefLongerThanData , RegionOriginNote , WhereClauseSuggestions , note_and_explain , } ;}
mkuse!{use crate :: fluent_generated as fluent ;}
mkuse!{use crate :: infer :: region_constraints :: GenericKind ;}
mkuse!{use crate :: infer :: { BoundRegionConversionTime , InferCtxt , RegionResolutionError , RegionVariableOrigin , SubregionOrigin , } ;}
mkitem!{mkimpl!{impl < 'a , 'tcx > TypeErrCtxt < 'a , 'tcx > { pub fn report_region_errors (& self , generic_param_scope : LocalDefId , errors : & [RegionResolutionError < 'tcx >] ,) -> ErrorGuaranteed { assert ! (! errors . is_empty ()) ; if let Some (guaranteed) = self . infcx . tainted_by_errors () { return guaranteed ; } debug ! ("report_region_errors(): {} errors to start" , errors . len ()) ; let errors = self . process_errors (errors) ; debug ! ("report_region_errors: {} errors after preprocessing" , errors . len ()) ; let mut guar = None ; for error in errors { debug ! ("report_region_errors: error = {:?}" , error) ; let e = if let Some (guar) = self . try_report_nice_region_error (generic_param_scope , & error) { guar } else { match error . clone () { RegionResolutionError :: ConcreteFailure (origin , sub , sup) => { if sub . is_placeholder () || sup . is_placeholder () { self . report_placeholder_failure (generic_param_scope , origin , sub , sup) . emit () } else { self . report_concrete_failure (generic_param_scope , origin , sub , sup) . emit () } } RegionResolutionError :: GenericBoundFailure (origin , param_ty , sub) => self . report_generic_bound_failure (generic_param_scope , origin . span () , Some (origin) , param_ty , sub ,) , RegionResolutionError :: SubSupConflict (_ , var_origin , sub_origin , sub_r , sup_origin , sup_r , _ ,) => { if sub_r . is_placeholder () { self . report_placeholder_failure (generic_param_scope , sub_origin , sub_r , sup_r ,) . emit () } else if sup_r . is_placeholder () { self . report_placeholder_failure (generic_param_scope , sup_origin , sub_r , sup_r ,) . emit () } else { self . report_sub_sup_conflict (generic_param_scope , var_origin , sub_origin , sub_r , sup_origin , sup_r ,) } } RegionResolutionError :: UpperBoundUniverseConflict (_ , _ , _ , sup_origin , sup_r ,) => { assert ! (sup_r . is_placeholder ()) ; let sub_r = self . tcx . lifetimes . re_erased ; self . report_placeholder_failure (generic_param_scope , sup_origin , sub_r , sup_r ,) . emit () } RegionResolutionError :: CannotNormalize (clause , origin) => { let clause : ty :: Clause < 'tcx > = clause . map_bound (ty :: ClauseKind :: TypeOutlives) . upcast (self . tcx) ; self . tcx . dcx () . struct_span_err (origin . span () , format ! ("cannot normalize `{clause}`")) . emit () } } } ; guar = Some (e) } guar . unwrap () } fn process_errors (& self , errors : & [RegionResolutionError < 'tcx >] ,) -> Vec < RegionResolutionError < 'tcx > > { debug ! ("process_errors()") ; let is_bound_failure = | e : & RegionResolutionError < 'tcx > | match * e { RegionResolutionError :: GenericBoundFailure (..) => true , RegionResolutionError :: ConcreteFailure (..) | RegionResolutionError :: SubSupConflict (..) | RegionResolutionError :: UpperBoundUniverseConflict (..) | RegionResolutionError :: CannotNormalize (..) => false , } ; let mut errors = if errors . iter () . all (| e | is_bound_failure (e)) { errors . to_owned () } else { errors . iter () . filter (| & e | ! is_bound_failure (e)) . cloned () . collect () } ; errors . sort_by_key (| u | match * u { RegionResolutionError :: ConcreteFailure (ref sro , _ , _) => sro . span () , RegionResolutionError :: GenericBoundFailure (ref sro , _ , _) => sro . span () , RegionResolutionError :: SubSupConflict (_ , ref rvo , _ , _ , _ , _ , _) => rvo . span () , RegionResolutionError :: UpperBoundUniverseConflict (_ , ref rvo , _ , _ , _) => rvo . span () , RegionResolutionError :: CannotNormalize (_ , ref sro) => sro . span () , }) ; errors } pub (super) fn note_region_origin (& self , err : & mut Diag < '_ > , origin : & SubregionOrigin < 'tcx >) { match * origin { SubregionOrigin :: Subtype (ref trace) => RegionOriginNote :: WithRequirement { span : trace . cause . span , requirement : ObligationCauseAsDiagArg (trace . cause . clone ()) , expected_found : self . values_str (trace . values , & trace . cause , err . long_ty_path ()) , } . add_to_diag (err) , SubregionOrigin :: Reborrow (span) => { RegionOriginNote :: Plain { span , msg : fluent :: trait_selection_reborrow } . add_to_diag (err) } SubregionOrigin :: RelateObjectBound (span) => { RegionOriginNote :: Plain { span , msg : fluent :: trait_selection_relate_object_bound } . add_to_diag (err) ; } SubregionOrigin :: ReferenceOutlivesReferent (ty , span) => { RegionOriginNote :: WithName { span , msg : fluent :: trait_selection_reference_outlives_referent , name : & self . ty_to_string (ty) , continues : false , } . add_to_diag (err) ; } SubregionOrigin :: RelateParamBound (span , ty , opt_span) => { RegionOriginNote :: WithName { span , msg : fluent :: trait_selection_relate_param_bound , name : & self . ty_to_string (ty) , continues : opt_span . is_some () , } . add_to_diag (err) ; if let Some (span) = opt_span { RegionOriginNote :: Plain { span , msg : fluent :: trait_selection_relate_param_bound_2 , } . add_to_diag (err) ; } } SubregionOrigin :: RelateRegionParamBound (span , _) => { RegionOriginNote :: Plain { span , msg : fluent :: trait_selection_relate_region_param_bound , } . add_to_diag (err) ; } SubregionOrigin :: CompareImplItemObligation { span , .. } => { RegionOriginNote :: Plain { span , msg : fluent :: trait_selection_compare_impl_item_obligation , } . add_to_diag (err) ; } SubregionOrigin :: CheckAssociatedTypeBounds { ref parent , .. } => { self . note_region_origin (err , parent) ; } SubregionOrigin :: AscribeUserTypeProvePredicate (span) => { RegionOriginNote :: Plain { span , msg : fluent :: trait_selection_ascribe_user_type_prove_predicate , } . add_to_diag (err) ; } } } pub (super) fn report_concrete_failure (& self , generic_param_scope : LocalDefId , origin : SubregionOrigin < 'tcx > , sub : Region < 'tcx > , sup : Region < 'tcx > ,) -> Diag < 'a > { let mut err = match origin { SubregionOrigin :: Subtype (box trace) => { let terr = TypeError :: RegionsDoesNotOutlive (sup , sub) ; let mut err = self . report_and_explain_type_error (trace , self . tcx . param_env (generic_param_scope) , terr ,) ; match (sub . kind () , sup . kind ()) { (ty :: RePlaceholder (_) , ty :: RePlaceholder (_)) => { } (ty :: RePlaceholder (_) , _) => { note_and_explain_region (self . tcx , & mut err , generic_param_scope , "" , sup , " doesn't meet the lifetime requirements" , None ,) ; } (_ , ty :: RePlaceholder (_)) => { note_and_explain_region (self . tcx , & mut err , generic_param_scope , "the required lifetime does not necessarily outlive " , sub , "" , None ,) ; } _ => { note_and_explain_region (self . tcx , & mut err , generic_param_scope , "" , sup , "..." , None ,) ; note_and_explain_region (self . tcx , & mut err , generic_param_scope , "...does not necessarily outlive " , sub , "" , None ,) ; } } err } SubregionOrigin :: Reborrow (span) => { let reference_valid = note_and_explain :: RegionExplanation :: new (self . tcx , generic_param_scope , sub , None , note_and_explain :: PrefixKind :: RefValidFor , note_and_explain :: SuffixKind :: Continues ,) ; let content_valid = note_and_explain :: RegionExplanation :: new (self . tcx , generic_param_scope , sup , None , note_and_explain :: PrefixKind :: ContentValidFor , note_and_explain :: SuffixKind :: Empty ,) ; self . dcx () . create_err (OutlivesContent { span , notes : reference_valid . into_iter () . chain (content_valid) . collect () , }) } SubregionOrigin :: RelateObjectBound (span) => { let object_valid = note_and_explain :: RegionExplanation :: new (self . tcx , generic_param_scope , sub , None , note_and_explain :: PrefixKind :: TypeObjValidFor , note_and_explain :: SuffixKind :: Empty ,) ; let pointer_valid = note_and_explain :: RegionExplanation :: new (self . tcx , generic_param_scope , sup , None , note_and_explain :: PrefixKind :: SourcePointerValidFor , note_and_explain :: SuffixKind :: Empty ,) ; self . dcx () . create_err (OutlivesBound { span , notes : object_valid . into_iter () . chain (pointer_valid) . collect () , }) } SubregionOrigin :: RelateParamBound (span , ty , opt_span) => { let prefix = match sub . kind () { ty :: ReStatic => note_and_explain :: PrefixKind :: TypeSatisfy , _ => note_and_explain :: PrefixKind :: TypeOutlive , } ; let suffix = if opt_span . is_some () { note_and_explain :: SuffixKind :: ReqByBinding } else { note_and_explain :: SuffixKind :: Empty } ; let note = note_and_explain :: RegionExplanation :: new (self . tcx , generic_param_scope , sub , opt_span , prefix , suffix ,) ; self . dcx () . create_err (FulfillReqLifetime { span , ty : self . resolve_vars_if_possible (ty) , note , }) } SubregionOrigin :: RelateRegionParamBound (span , ty) => { let param_instantiated = note_and_explain :: RegionExplanation :: new (self . tcx , generic_param_scope , sup , None , note_and_explain :: PrefixKind :: LfParamInstantiatedWith , note_and_explain :: SuffixKind :: Empty ,) ; let mut alt_span = None ; if let Some (ty) = ty && sub . is_static () && let ty :: Dynamic (preds , _ , ty :: DynKind :: Dyn) = ty . kind () && let Some (def_id) = preds . principal_def_id () { for (clause , span) in self . tcx . predicates_of (def_id) . instantiate_identity (self . tcx) { if let ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (a , b)) = clause . kind () . skip_binder () && let ty :: Param (param) = a . kind () && param . name == kw :: SelfUpper && b . is_static () { alt_span = Some (span) ; } } } let param_must_outlive = note_and_explain :: RegionExplanation :: new (self . tcx , generic_param_scope , sub , alt_span , note_and_explain :: PrefixKind :: LfParamMustOutlive , note_and_explain :: SuffixKind :: Empty ,) ; self . dcx () . create_err (LfBoundNotSatisfied { span , notes : param_instantiated . into_iter () . chain (param_must_outlive) . collect () , }) } SubregionOrigin :: ReferenceOutlivesReferent (ty , span) => { let pointer_valid = note_and_explain :: RegionExplanation :: new (self . tcx , generic_param_scope , sub , None , note_and_explain :: PrefixKind :: PointerValidFor , note_and_explain :: SuffixKind :: Empty ,) ; let data_valid = note_and_explain :: RegionExplanation :: new (self . tcx , generic_param_scope , sup , None , note_and_explain :: PrefixKind :: DataValidFor , note_and_explain :: SuffixKind :: Empty ,) ; self . dcx () . create_err (RefLongerThanData { span , ty : self . resolve_vars_if_possible (ty) , notes : pointer_valid . into_iter () . chain (data_valid) . collect () , }) } SubregionOrigin :: CompareImplItemObligation { span , impl_item_def_id , trait_item_def_id , } => { let mut err = self . report_extra_impl_obligation (span , impl_item_def_id , trait_item_def_id , & format ! ("`{sup}: {sub}`") ,) ; if let Some (generics) = self . tcx . hir_get_generics (impl_item_def_id) && generics . where_clause_span . contains (span) { self . suggest_copy_trait_method_bounds (trait_item_def_id , impl_item_def_id , & mut err ,) ; } err } SubregionOrigin :: CheckAssociatedTypeBounds { impl_item_def_id , trait_item_def_id , parent , } => { let mut err = self . report_concrete_failure (generic_param_scope , * parent , sub , sup) ; if ! self . tcx . is_impl_trait_in_trait (impl_item_def_id . to_def_id ()) { let trait_item_span = self . tcx . def_span (trait_item_def_id) ; let item_name = self . tcx . item_name (impl_item_def_id . to_def_id ()) ; err . span_label (trait_item_span , format ! ("definition of `{item_name}` from trait") ,) ; } self . suggest_copy_trait_method_bounds (trait_item_def_id , impl_item_def_id , & mut err ,) ; err } SubregionOrigin :: AscribeUserTypeProvePredicate (span) => { let instantiated = note_and_explain :: RegionExplanation :: new (self . tcx , generic_param_scope , sup , None , note_and_explain :: PrefixKind :: LfInstantiatedWith , note_and_explain :: SuffixKind :: Empty ,) ; let must_outlive = note_and_explain :: RegionExplanation :: new (self . tcx , generic_param_scope , sub , None , note_and_explain :: PrefixKind :: LfMustOutlive , note_and_explain :: SuffixKind :: Empty ,) ; self . dcx () . create_err (LfBoundNotSatisfied { span , notes : instantiated . into_iter () . chain (must_outlive) . collect () , }) } } ; if sub . is_error () || sup . is_error () { err . downgrade_to_delayed_bug () ; } err } pub fn suggest_copy_trait_method_bounds (& self , trait_item_def_id : DefId , impl_item_def_id : LocalDefId , err : & mut Diag < '_ > ,) { let Some (impl_def_id) = self . tcx . trait_impl_of_assoc (impl_item_def_id . to_def_id ()) else { return ; } ; let trait_ref = self . tcx . impl_trait_ref (impl_def_id) . unwrap () ; let trait_args = trait_ref . instantiate_identity () . with_replaced_self_ty (self . tcx , Ty :: new_param (self . tcx , 0 , kw :: SelfUpper)) . args ; let trait_item_args = ty :: GenericArgs :: identity_for_item (self . tcx , impl_item_def_id) . rebase_onto (self . tcx , impl_def_id , trait_args) ; let Ok (trait_predicates) = self . tcx . explicit_predicates_of (trait_item_def_id) . instantiate_own (self . tcx , trait_item_args) . map (| (pred , _) | { if pred . is_suggestable (self . tcx , false) { Ok (pred . to_string ()) } else { Err (()) } }) . collect :: < Result < Vec < _ > , () > > () else { return ; } ; let Some (generics) = self . tcx . hir_get_generics (impl_item_def_id) else { return ; } ; let suggestion = if trait_predicates . is_empty () { WhereClauseSuggestions :: Remove { span : generics . where_clause_span } } else { let space = if generics . where_clause_span . is_empty () { " " } else { "" } ; WhereClauseSuggestions :: CopyPredicates { span : generics . where_clause_span , space , trait_predicates : trait_predicates . join (", ") , } } ; err . subdiagnostic (suggestion) ; } pub (super) fn report_placeholder_failure (& self , generic_param_scope : LocalDefId , placeholder_origin : SubregionOrigin < 'tcx > , sub : Region < 'tcx > , sup : Region < 'tcx > ,) -> Diag < 'a > { debug ! (? placeholder_origin , ? sub , ? sup , "report_placeholder_failure") ; match placeholder_origin { SubregionOrigin :: Subtype (box ref trace) if matches ! (& trace . cause . code () . peel_derives () , ObligationCauseCode :: WhereClause (..) | ObligationCauseCode :: WhereClauseInExpr (..)) => { if let ObligationCauseCode :: WhereClause (_ , span) | ObligationCauseCode :: WhereClauseInExpr (_ , span , ..) = & trace . cause . code () . peel_derives () { let span = * span ; let mut err = self . report_concrete_failure (generic_param_scope , placeholder_origin , sub , sup ,) ; if ! span . is_dummy () { err = err . with_span_note (span , "the lifetime requirement is introduced here") ; } err } else { unreachable ! ("control flow ensures we have a `BindingObligation` or `WhereClauseInExpr` here...") } } SubregionOrigin :: Subtype (box trace) => { let terr = TypeError :: RegionsPlaceholderMismatch ; return self . report_and_explain_type_error (trace , self . tcx . param_env (generic_param_scope) , terr ,) ; } _ => { return self . report_concrete_failure (generic_param_scope , placeholder_origin , sub , sup ,) ; } } } pub fn report_generic_bound_failure (& self , generic_param_scope : LocalDefId , span : Span , origin : Option < SubregionOrigin < 'tcx > > , bound_kind : GenericKind < 'tcx > , sub : Region < 'tcx > ,) -> ErrorGuaranteed { self . construct_generic_bound_failure (generic_param_scope , span , origin , bound_kind , sub) . emit () } pub fn construct_generic_bound_failure (& self , generic_param_scope : LocalDefId , span : Span , origin : Option < SubregionOrigin < 'tcx > > , bound_kind : GenericKind < 'tcx > , sub : Region < 'tcx > ,) -> Diag < 'a > { if let Some (SubregionOrigin :: CompareImplItemObligation { span , impl_item_def_id , trait_item_def_id , }) = origin { return self . report_extra_impl_obligation (span , impl_item_def_id , trait_item_def_id , & format ! ("`{bound_kind}: {sub}`") ,) ; } let labeled_user_string = match bound_kind { GenericKind :: Param (_) => format ! ("the parameter type `{bound_kind}`") , GenericKind :: Placeholder (_) => format ! ("the placeholder type `{bound_kind}`") , GenericKind :: Alias (p) => match p . kind (self . tcx) { ty :: Projection | ty :: Inherent => { format ! ("the associated type `{bound_kind}`") } ty :: Free => format ! ("the type alias `{bound_kind}`") , ty :: Opaque => format ! ("the opaque type `{bound_kind}`") , } , } ; let mut err = self . tcx . dcx () . struct_span_err (span , format ! ("{labeled_user_string} may not live long enough")) ; err . code (match sub . kind () { ty :: ReEarlyParam (_) | ty :: ReLateParam (_) if sub . is_named (self . tcx) => E0309 , ty :: ReStatic => E0310 , _ => E0311 , }) ; '_explain : { let (description , span) = match sub . kind () { ty :: ReEarlyParam (_) | ty :: ReLateParam (_) | ty :: ReStatic => { msg_span_from_named_region (self . tcx , generic_param_scope , sub , Some (span)) } _ => (format ! ("lifetime `{sub}`") , Some (span)) , } ; let prefix = format ! ("{labeled_user_string} must be valid for ") ; label_msg_span (& mut err , & prefix , description , span , "...") ; if let Some (origin) = origin { self . note_region_origin (& mut err , & origin) ; } } 'suggestion : { let msg = "consider adding an explicit lifetime bound" ; if (bound_kind , sub) . has_infer_regions () || (bound_kind , sub) . has_placeholders () || ! bound_kind . is_suggestable (self . tcx , false) { let lt_name = sub . get_name_or_anon (self . tcx) . to_string () ; err . help (format ! ("{msg} `{bound_kind}: {lt_name}`...")) ; break 'suggestion ; } let mut generic_param_scope = generic_param_scope ; while self . tcx . def_kind (generic_param_scope) == DefKind :: OpaqueTy { generic_param_scope = self . tcx . local_parent (generic_param_scope) ; } let (type_scope , type_param_sugg_span) = match bound_kind { GenericKind :: Param (param) => { let generics = self . tcx . generics_of (generic_param_scope) ; let type_param = generics . type_param (param , self . tcx) ; let def_id = type_param . def_id . expect_local () ; let scope = self . tcx . local_def_id_to_hir_id (def_id) . owner . def_id ; let hir_generics = self . tcx . hir_get_generics (scope) . unwrap () ; let sugg_span = match hir_generics . bounds_span_for_suggestions (def_id) { Some ((span , open_paren_sp)) => Some ((span , true , open_paren_sp)) , None if generics . has_self && param . index == 0 => None , None => { let span = if let Some (param) = hir_generics . params . iter () . find (| param | param . def_id == def_id) && let ParamName :: Plain (ident) = param . name { ident . span . shrink_to_hi () } else { let span = self . tcx . def_span (def_id) ; span . shrink_to_hi () } ; Some ((span , false , None)) } } ; (scope , sugg_span) } _ => (generic_param_scope , None) , } ; let suggestion_scope = { let lifetime_scope = match sub . kind () { ty :: ReStatic => hir :: def_id :: CRATE_DEF_ID , _ => match self . tcx . is_suitable_region (generic_param_scope , sub) { Some (info) => info . scope , None => generic_param_scope , } , } ; match self . tcx . is_descendant_of (type_scope . into () , lifetime_scope . into ()) { true => type_scope , false => lifetime_scope , } } ; let mut suggs = vec ! [] ; let lt_name = self . suggest_name_region (generic_param_scope , sub , & mut suggs) ; if let Some ((sp , has_lifetimes , open_paren_sp)) = type_param_sugg_span && suggestion_scope == type_scope { let suggestion = if has_lifetimes { format ! (" + {lt_name}") } else { format ! (": {lt_name}") } ; if let Some (open_paren_sp) = open_paren_sp { suggs . push ((open_paren_sp , "(" . to_string ())) ; suggs . push ((sp , format ! ("){suggestion}"))) ; } else { suggs . push ((sp , suggestion)) } } else if let GenericKind :: Alias (ref p) = bound_kind && let ty :: Projection = p . kind (self . tcx) && let DefKind :: AssocTy = self . tcx . def_kind (p . def_id) && let Some (ty :: ImplTraitInTraitData :: Trait { .. }) = self . tcx . opt_rpitit_info (p . def_id) { } else if let Some (generics) = self . tcx . hir_get_generics (suggestion_scope) { let pred = format ! ("{bound_kind}: {lt_name}") ; let suggestion = format ! ("{} {}" , generics . add_where_or_trailing_comma () , pred) ; suggs . push ((generics . tail_span_for_predicate_suggestion () , suggestion)) } else { let consider = format ! ("{msg} `{bound_kind}: {sub}`...") ; err . help (consider) ; } if ! suggs . is_empty () { err . multipart_suggestion_verbose (msg , suggs , Applicability :: MaybeIncorrect ,) ; } } err } pub fn suggest_name_region (& self , generic_param_scope : LocalDefId , lifetime : Region < 'tcx > , add_lt_suggs : & mut Vec < (Span , String) > ,) -> String { struct LifetimeReplaceVisitor < 'a > { needle : hir :: LifetimeKind , new_lt : & 'a str , add_lt_suggs : & 'a mut Vec < (Span , String) > , } impl < 'hir > hir :: intravisit :: Visitor < 'hir > for LifetimeReplaceVisitor < '_ > { fn visit_lifetime (& mut self , lt : & 'hir hir :: Lifetime) { if lt . kind == self . needle { self . add_lt_suggs . push (lt . suggestion (self . new_lt)) ; } } } let (lifetime_def_id , lifetime_scope) = match self . tcx . is_suitable_region (generic_param_scope , lifetime) { Some (info) if ! lifetime . is_named (self . tcx) => { (info . region_def_id . expect_local () , info . scope) } _ => return lifetime . get_name_or_anon (self . tcx) . to_string () , } ; let new_lt = { let generics = self . tcx . generics_of (lifetime_scope) ; let mut used_names = iter :: successors (Some (generics) , | g | g . parent . map (| p | self . tcx . generics_of (p))) . flat_map (| g | & g . own_params) . filter (| p | matches ! (p . kind , ty :: GenericParamDefKind :: Lifetime)) . map (| p | p . name) . collect :: < Vec < _ > > () ; let hir_id = self . tcx . local_def_id_to_hir_id (lifetime_scope) ; used_names . extend (self . tcx . late_bound_vars (hir_id) . into_iter () . filter_map (| p | match p { ty :: BoundVariableKind :: Region (lt) => lt . get_name (self . tcx) , _ => None , } ,)) ; (b'a' ..= b'z') . map (| c | format ! ("'{}" , c as char)) . find (| candidate | ! used_names . iter () . any (| e | e . as_str () == candidate)) . unwrap_or_else (| | "'lt" . to_string ()) } ; let mut visitor = LifetimeReplaceVisitor { needle : hir :: LifetimeKind :: Param (lifetime_def_id) , add_lt_suggs , new_lt : & new_lt , } ; match self . tcx . expect_hir_owner_node (lifetime_scope) { hir :: OwnerNode :: Item (i) => visitor . visit_item (i) , hir :: OwnerNode :: ForeignItem (i) => visitor . visit_foreign_item (i) , hir :: OwnerNode :: ImplItem (i) => visitor . visit_impl_item (i) , hir :: OwnerNode :: TraitItem (i) => visitor . visit_trait_item (i) , hir :: OwnerNode :: Crate (_) => bug ! ("OwnerNode::Crate doesn't not have generics") , hir :: OwnerNode :: Synthetic => unreachable ! () , } let ast_generics = self . tcx . hir_get_generics (lifetime_scope) . unwrap () ; let sugg = ast_generics . span_for_lifetime_suggestion () . map (| span | (span , format ! ("{new_lt}, "))) . unwrap_or_else (| | (ast_generics . span , format ! ("<{new_lt}>"))) ; add_lt_suggs . push (sugg) ; new_lt } fn report_sub_sup_conflict (& self , generic_param_scope : LocalDefId , var_origin : RegionVariableOrigin , sub_origin : SubregionOrigin < 'tcx > , sub_region : Region < 'tcx > , sup_origin : SubregionOrigin < 'tcx > , sup_region : Region < 'tcx > ,) -> ErrorGuaranteed { let mut err = self . report_inference_failure (var_origin) ; note_and_explain_region (self . tcx , & mut err , generic_param_scope , "first, the lifetime cannot outlive " , sup_region , "..." , None ,) ; debug ! ("report_sub_sup_conflict: var_origin={:?}" , var_origin) ; debug ! ("report_sub_sup_conflict: sub_region={:?}" , sub_region) ; debug ! ("report_sub_sup_conflict: sub_origin={:?}" , sub_origin) ; debug ! ("report_sub_sup_conflict: sup_region={:?}" , sup_region) ; debug ! ("report_sub_sup_conflict: sup_origin={:?}" , sup_origin) ; if let SubregionOrigin :: Subtype (ref sup_trace) = sup_origin && let SubregionOrigin :: Subtype (ref sub_trace) = sub_origin && let Some ((sup_expected , sup_found)) = self . values_str (sup_trace . values , & sup_trace . cause , err . long_ty_path ()) && let Some ((sub_expected , sub_found)) = self . values_str (sub_trace . values , & sup_trace . cause , err . long_ty_path ()) && sub_expected == sup_expected && sub_found == sup_found { note_and_explain_region (self . tcx , & mut err , generic_param_scope , "...but the lifetime must also be valid for " , sub_region , "..." , None ,) ; err . span_note (sup_trace . cause . span , format ! ("...so that the {}" , sup_trace . cause . as_requirement_str ()) ,) ; err . note_expected_found ("" , sup_expected , "" , sup_found) ; return if sub_region . is_error () | sup_region . is_error () { err . delay_as_bug () } else { err . emit () } ; } self . note_region_origin (& mut err , & sup_origin) ; note_and_explain_region (self . tcx , & mut err , generic_param_scope , "but, the lifetime must be valid for " , sub_region , "..." , None ,) ; self . note_region_origin (& mut err , & sub_origin) ; if sub_region . is_error () | sup_region . is_error () { err . delay_as_bug () } else { err . emit () } } fn report_inference_failure (& self , var_origin : RegionVariableOrigin) -> Diag < '_ > { let br_string = | br : ty :: BoundRegionKind | { let mut s = match br { ty :: BoundRegionKind :: Named (def_id) => self . tcx . item_name (def_id) . to_string () , _ => String :: new () , } ; if ! s . is_empty () { s . push (' ') ; } s } ; let var_description = match var_origin { RegionVariableOrigin :: Misc (_) => String :: new () , RegionVariableOrigin :: PatternRegion (_) => " for pattern" . to_string () , RegionVariableOrigin :: BorrowRegion (_) => " for borrow expression" . to_string () , RegionVariableOrigin :: Autoref (_) => " for autoref" . to_string () , RegionVariableOrigin :: Coercion (_) => " for automatic coercion" . to_string () , RegionVariableOrigin :: BoundRegion (_ , br , BoundRegionConversionTime :: FnCall) => { format ! (" for lifetime parameter {}in function call" , br_string (br)) } RegionVariableOrigin :: BoundRegion (_ , br , BoundRegionConversionTime :: HigherRankedType ,) => { format ! (" for lifetime parameter {}in generic type" , br_string (br)) } RegionVariableOrigin :: BoundRegion (_ , br , BoundRegionConversionTime :: AssocTypeProjection (def_id) ,) => format ! (" for lifetime parameter {}in trait containing associated type `{}`" , br_string (br) , self . tcx . associated_item (def_id) . name ()) , RegionVariableOrigin :: RegionParameterDefinition (_ , name) => { format ! (" for lifetime parameter `{name}`") } RegionVariableOrigin :: UpvarRegion (ref upvar_id , _) => { let var_name = self . tcx . hir_name (upvar_id . var_path . hir_id) ; format ! (" for capture of `{var_name}` by closure") } RegionVariableOrigin :: Nll (..) => bug ! ("NLL variable found in lexical phase") , } ; struct_span_code_err ! (self . dcx () , var_origin . span () , E0803 , "cannot infer an appropriate lifetime{} due to conflicting requirements" , var_description) } }}}

macro_rules! note_and_explain_region_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function note_and_explain_region in module {}", module_path!());
    };
}

mkfn!{
    note_and_explain_region_introspect!();
    pub (super) fn note_and_explain_region < 'tcx > (tcx : TyCtxt < 'tcx > , err : & mut Diag < '_ > , generic_param_scope : LocalDefId , prefix : & str , region : ty :: Region < 'tcx > , suffix : & str , alt_span : Option < Span > ,) { let (description , span) = match region . kind () { ty :: ReEarlyParam (_) | ty :: ReLateParam (_) | ty :: RePlaceholder (_) | ty :: ReStatic => { msg_span_from_named_region (tcx , generic_param_scope , region , alt_span) } ty :: ReError (_) => return , ty :: ReVar (_) => (format ! ("lifetime `{region}`") , alt_span) , ty :: ReBound (..) | ty :: ReErased => { bug ! ("unexpected region for note_and_explain_region: {:?}" , region) ; } } ; emit_msg_span (err , prefix , description , span , suffix) ; }
}

macro_rules! explain_free_region_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function explain_free_region in module {}", module_path!());
    };
}

mkfn!{
    explain_free_region_introspect!();
    fn explain_free_region < 'tcx > (tcx : TyCtxt < 'tcx > , err : & mut Diag < '_ > , generic_param_scope : LocalDefId , prefix : & str , region : ty :: Region < 'tcx > , suffix : & str ,) { let (description , span) = msg_span_from_named_region (tcx , generic_param_scope , region , None) ; label_msg_span (err , prefix , description , span , suffix) ; }
}

macro_rules! msg_span_from_named_region_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function msg_span_from_named_region in module {}", module_path!());
    };
}

mkfn!{
    msg_span_from_named_region_introspect!();
    fn msg_span_from_named_region < 'tcx > (tcx : TyCtxt < 'tcx > , generic_param_scope : LocalDefId , region : ty :: Region < 'tcx > , alt_span : Option < Span > ,) -> (String , Option < Span >) { match region . kind () { ty :: ReEarlyParam (br) => { let param_def_id = tcx . generics_of (generic_param_scope) . region_param (br , tcx) . def_id ; let span = tcx . def_span (param_def_id) ; let text = if br . is_named () { format ! ("the lifetime `{}` as defined here" , br . name) } else { "the anonymous lifetime as defined here" . to_string () } ; (text , Some (span)) } ty :: ReLateParam (ref fr) => { if ! fr . kind . is_named (tcx) && let Some ((ty , _)) = find_anon_type (tcx , generic_param_scope , region) { ("the anonymous lifetime defined here" . to_string () , Some (ty . span)) } else { match fr . kind { ty :: LateParamRegionKind :: Named (param_def_id) => { let name = tcx . item_name (param_def_id) ; let span = tcx . def_span (param_def_id) ; let text = if name == kw :: UnderscoreLifetime { "the anonymous lifetime as defined here" . to_string () } else { format ! ("the lifetime `{name}` as defined here") } ; (text , Some (span)) } ty :: LateParamRegionKind :: Anon (_) => ("the anonymous lifetime as defined here" . to_string () , Some (tcx . def_span (generic_param_scope)) ,) , _ => (format ! ("the lifetime `{region}` as defined here") , Some (tcx . def_span (generic_param_scope)) ,) , } } } ty :: ReStatic => ("the static lifetime" . to_owned () , alt_span) , ty :: RePlaceholder (ty :: PlaceholderRegion { bound : ty :: BoundRegion { kind : ty :: BoundRegionKind :: Named (def_id) , .. } , .. }) => (format ! ("the lifetime `{}` as defined here" , tcx . item_name (def_id)) , Some (tcx . def_span (def_id)) ,) , ty :: RePlaceholder (ty :: PlaceholderRegion { bound : ty :: BoundRegion { kind : ty :: BoundRegionKind :: Anon , .. } , .. }) => ("an anonymous lifetime" . to_owned () , None) , _ => bug ! ("{:?}" , region) , } }
}

macro_rules! emit_msg_span_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_msg_span in module {}", module_path!());
    };
}

mkfn!{
    emit_msg_span_introspect!();
    fn emit_msg_span (err : & mut Diag < '_ > , prefix : & str , description : String , span : Option < Span > , suffix : & str ,) { let message = format ! ("{prefix}{description}{suffix}") ; if let Some (span) = span { err . span_note (span , message) ; } else { err . note (message) ; } }
}

macro_rules! label_msg_span_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function label_msg_span in module {}", module_path!());
    };
}

mkfn!{
    label_msg_span_introspect!();
    fn label_msg_span (err : & mut Diag < '_ > , prefix : & str , description : String , span : Option < Span > , suffix : & str ,) { let message = format ! ("{prefix}{description}{suffix}") ; if let Some (span) = span { err . span_label (span , message) ; } else { err . note (message) ; } }
}

macro_rules! unexpected_hidden_region_diagnostic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unexpected_hidden_region_diagnostic in module {}", module_path!());
    };
}

mkfn!{
    unexpected_hidden_region_diagnostic_introspect!();
    #[instrument (level = "trace" , skip (infcx))] pub fn unexpected_hidden_region_diagnostic < 'a , 'tcx > (infcx : & 'a InferCtxt < 'tcx > , generic_param_scope : LocalDefId , span : Span , hidden_ty : Ty < 'tcx > , hidden_region : ty :: Region < 'tcx > , opaque_ty_key : ty :: OpaqueTypeKey < 'tcx > ,) -> Diag < 'a > { let tcx = infcx . tcx ; let mut err = infcx . dcx () . create_err (errors :: OpaqueCapturesLifetime { span , opaque_ty : Ty :: new_opaque (tcx , opaque_ty_key . def_id . to_def_id () , opaque_ty_key . args) , opaque_ty_span : tcx . def_span (opaque_ty_key . def_id) , }) ; match hidden_region . kind () { ty :: ReEarlyParam (_) | ty :: ReLateParam (_) | ty :: ReStatic => { explain_free_region (tcx , & mut err , generic_param_scope , & format ! ("hidden type `{hidden_ty}` captures ") , hidden_region , "" ,) ; if let Some (_) = tcx . is_suitable_region (generic_param_scope , hidden_region) { suggest_precise_capturing (tcx , opaque_ty_key . def_id , hidden_region , & mut err) ; } } ty :: RePlaceholder (_) => { explain_free_region (tcx , & mut err , generic_param_scope , & format ! ("hidden type `{}` captures " , hidden_ty) , hidden_region , "" ,) ; } ty :: ReError (_) => { err . downgrade_to_delayed_bug () ; } _ => { note_and_explain_region (tcx , & mut err , generic_param_scope , & format ! ("hidden type `{hidden_ty}` captures ") , hidden_region , "" , None ,) ; } } err }
}

macro_rules! suggest_precise_capturing_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function suggest_precise_capturing in module {}", module_path!());
    };
}

mkfn!{
    suggest_precise_capturing_introspect!();
    fn suggest_precise_capturing < 'tcx > (tcx : TyCtxt < 'tcx > , opaque_def_id : LocalDefId , captured_lifetime : ty :: Region < 'tcx > , diag : & mut Diag < '_ > ,) { let hir :: OpaqueTy { bounds , origin , .. } = tcx . hir_node_by_def_id (opaque_def_id) . expect_opaque_ty () ; let hir :: OpaqueTyOrigin :: FnReturn { parent : fn_def_id , .. } = * origin else { return ; } ; let new_lifetime = Symbol :: intern (& captured_lifetime . to_string ()) ; if let Some ((args , span)) = bounds . iter () . find_map (| bound | match bound { hir :: GenericBound :: Use (args , span) => Some ((args , span)) , _ => None , }) { let last_lifetime_span = args . iter () . rev () . find_map (| arg | match arg { hir :: PreciseCapturingArg :: Lifetime (lt) => Some (lt . ident . span) , _ => None , }) ; let first_param_span = args . iter () . find_map (| arg | match arg { hir :: PreciseCapturingArg :: Param (p) => Some (p . ident . span) , _ => None , }) ; let (span , pre , post) = if let Some (last_lifetime_span) = last_lifetime_span { (last_lifetime_span . shrink_to_hi () , ", " , "") } else if let Some (first_param_span) = first_param_span { (first_param_span . shrink_to_lo () , "" , ", ") } else { (span . with_hi (span . hi () - BytePos (1)) . shrink_to_hi () , "" , "") } ; diag . subdiagnostic (errors :: AddPreciseCapturing :: Existing { span , new_lifetime , pre , post }) ; } else { let mut captured_lifetimes = FxIndexSet :: default () ; let mut captured_non_lifetimes = FxIndexSet :: default () ; let variances = tcx . variances_of (opaque_def_id) ; let mut generics = tcx . generics_of (opaque_def_id) ; let mut synthetics = vec ! [] ; loop { for param in & generics . own_params { if variances [param . index as usize] == ty :: Bivariant { continue ; } match param . kind { ty :: GenericParamDefKind :: Lifetime => { captured_lifetimes . insert (param . name) ; } ty :: GenericParamDefKind :: Type { synthetic : true , .. } => { synthetics . push ((tcx . def_span (param . def_id) , param . name)) ; } ty :: GenericParamDefKind :: Type { .. } | ty :: GenericParamDefKind :: Const { .. } => { captured_non_lifetimes . insert (param . name) ; } } } if let Some (parent) = generics . parent { generics = tcx . generics_of (parent) ; } else { break ; } } if ! captured_lifetimes . insert (new_lifetime) { return ; } if synthetics . is_empty () { let concatenated_bounds = captured_lifetimes . into_iter () . chain (captured_non_lifetimes) . map (| sym | sym . to_string ()) . collect :: < Vec < _ > > () . join (", ") ; diag . subdiagnostic (errors :: AddPreciseCapturing :: New { span : tcx . def_span (opaque_def_id) . shrink_to_hi () , new_lifetime , concatenated_bounds , }) ; } else { let mut next_fresh_param = | | { ["T" , "U" , "V" , "W" , "X" , "Y" , "A" , "B" , "C"] . into_iter () . map (Symbol :: intern) . chain ((0 ..) . map (| i | Symbol :: intern (& format ! ("T{i}")))) . find (| s | captured_non_lifetimes . insert (* s)) . unwrap () } ; let mut new_params = String :: new () ; let mut suggs = vec ! [] ; let mut apit_spans = vec ! [] ; for (i , (span , name)) in synthetics . into_iter () . enumerate () { apit_spans . push (span) ; let fresh_param = next_fresh_param () ; suggs . push ((span , fresh_param . to_string ())) ; if i > 0 { new_params += ", " ; } let name_as_bounds = name . as_str () . trim_start_matches ("impl") . trim_start () ; new_params += fresh_param . as_str () ; new_params += ": " ; new_params += name_as_bounds ; } let Some (generics) = tcx . hir_get_generics (fn_def_id) else { return ; } ; suggs . push (if let Some (params_span) = generics . span_for_param_suggestion () { (params_span , format ! (", {new_params}")) } else { (generics . span , format ! ("<{new_params}>")) }) ; let concatenated_bounds = captured_lifetimes . into_iter () . chain (captured_non_lifetimes) . map (| sym | sym . to_string ()) . collect :: < Vec < _ > > () . join (", ") ; suggs . push ((tcx . def_span (opaque_def_id) . shrink_to_hi () , format ! (" + use<{concatenated_bounds}>") ,)) ; diag . subdiagnostic (errors :: AddPreciseCapturingAndParams { suggs , new_lifetime , apit_spans , }) ; } } }
}