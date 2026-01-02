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
mkuse!{use rustc_data_structures :: fx :: { FxHashSet , FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: struct_span_code_err ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_lint_defs :: builtin :: UNUSED_ASSOCIATED_TYPE_BOUNDS ;}
mkuse!{use rustc_middle :: ty :: elaborate :: ClauseWithSupertraitSpan ;}
mkuse!{use rustc_middle :: ty :: { self , BottomUpFolder , DynKind , ExistentialPredicateStableCmpExt as _ , Ty , TyCtxt , TypeFoldable , TypeVisitableExt , Upcast , } ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , Span } ;}
mkuse!{use rustc_trait_selection :: error_reporting :: traits :: report_dyn_incompatibility ;}
mkuse!{use rustc_trait_selection :: traits ;}
mkuse!{use smallvec :: { SmallVec , smallvec } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: HirTyLowerer ;}
mkuse!{use crate :: errors :: SelfInTypeAlias ;}
mkuse!{use crate :: hir_ty_lowering :: { GenericArgCountMismatch , PredicateFilter , RegionInferReason } ;}
mkitem!{mkimpl!{impl < 'tcx > dyn HirTyLowerer < 'tcx > + '_ { # [doc = " Lower a trait object type from the HIR to our internal notion of a type."] # [instrument (level = "debug" , skip_all , ret)] pub (super) fn lower_trait_object_ty (& self , span : Span , hir_id : hir :: HirId , hir_bounds : & [hir :: PolyTraitRef < 'tcx >] , lifetime : & hir :: Lifetime , representation : DynKind ,) -> Ty < 'tcx > { let tcx = self . tcx () ; let dummy_self = tcx . types . trait_object_dummy_self ; let mut user_written_bounds = Vec :: new () ; let mut potential_assoc_types = Vec :: new () ; for poly_trait_ref in hir_bounds . iter () { let result = self . lower_poly_trait_ref (poly_trait_ref , dummy_self , & mut user_written_bounds , PredicateFilter :: SelfOnly ,) ; if let Err (GenericArgCountMismatch { invalid_args , .. }) = result . correct { potential_assoc_types . extend (invalid_args) ; } } let ast_bounds : Vec < _ > = hir_bounds . iter () . map (| & trait_ref | hir :: GenericBound :: Trait (trait_ref)) . collect () ; self . add_default_traits (& mut user_written_bounds , dummy_self , & ast_bounds , None , span) ; let (elaborated_trait_bounds , elaborated_projection_bounds) = traits :: expand_trait_aliases (tcx , user_written_bounds . iter () . copied ()) ; let (regular_traits , mut auto_traits) : (Vec < _ > , Vec < _ >) = elaborated_trait_bounds . into_iter () . partition (| (trait_ref , _) | ! tcx . trait_is_auto (trait_ref . def_id ())) ; if regular_traits . is_empty () && auto_traits . is_empty () { let guar = self . report_trait_object_with_no_traits (span , user_written_bounds . iter () . copied ()) ; return Ty :: new_error (tcx , guar) ; } if regular_traits . len () > 1 { let guar = self . report_trait_object_addition_traits (& regular_traits) ; return Ty :: new_error (tcx , guar) ; } if let Err (guar) = regular_traits . error_reported () { return Ty :: new_error (tcx , guar) ; } for (clause , span) in user_written_bounds { if let Some (trait_pred) = clause . as_trait_clause () { let violations = self . dyn_compatibility_violations (trait_pred . def_id ()) ; if ! violations . is_empty () { let reported = report_dyn_incompatibility (tcx , span , Some (hir_id) , trait_pred . def_id () , & violations ,) . emit () ; return Ty :: new_error (tcx , reported) ; } } } let mut projection_bounds = FxIndexMap :: default () ; for (proj , proj_span) in elaborated_projection_bounds { let proj = proj . map_bound (| mut b | { if let Some (term_ty) = & b . term . as_type () { let references_self = term_ty . walk () . any (| arg | arg == dummy_self . into ()) ; if references_self { let guar = self . dcx () . emit_err (SelfInTypeAlias { span }) ; b . term = replace_dummy_self_with_error (tcx , b . term , guar) ; } } b }) ; let key = (proj . skip_binder () . projection_term . def_id , tcx . anonymize_bound_vars (proj . map_bound (| proj | proj . projection_term . trait_ref (tcx)) ,) ,) ; if let Some ((old_proj , old_proj_span)) = projection_bounds . insert (key , (proj , proj_span)) && tcx . anonymize_bound_vars (proj) != tcx . anonymize_bound_vars (old_proj) { let item = tcx . item_name (proj . item_def_id ()) ; self . dcx () . struct_span_err (span , format ! ("conflicting associated type bounds for `{item}` when \
                            expanding trait alias") ,) . with_span_label (old_proj_span , format ! ("`{item}` is specified to be `{}` here" , old_proj . term ()) ,) . with_span_label (proj_span , format ! ("`{item}` is specified to be `{}` here" , proj . term ()) ,) . emit () ; } } let principal_trait = regular_traits . into_iter () . next () ; let mut ordered_associated_types = vec ! [] ; if let Some ((principal_trait , ref spans)) = principal_trait { let principal_trait = principal_trait . map_bound (| trait_pred | { assert_eq ! (trait_pred . polarity , ty :: PredicatePolarity :: Positive) ; trait_pred . trait_ref }) ; for ClauseWithSupertraitSpan { clause , supertrait_span } in traits :: elaborate (tcx , [ClauseWithSupertraitSpan :: new (ty :: TraitRef :: identity (tcx , principal_trait . def_id ()) . upcast (tcx) , * spans . last () . unwrap () ,)] ,) . filter_only_self () { let clause = clause . instantiate_supertrait (tcx , principal_trait) ; debug ! ("observing object predicate `{clause:?}`") ; let bound_predicate = clause . kind () ; match bound_predicate . skip_binder () { ty :: ClauseKind :: Trait (pred) => { let trait_ref = tcx . anonymize_bound_vars (bound_predicate . rebind (pred . trait_ref)) ; ordered_associated_types . extend (tcx . associated_items (pred . trait_ref . def_id) . in_definition_order () . filter (| item | item . is_type ()) . filter (| item | ! item . is_impl_trait_in_trait ()) . map (| item | (item . def_id , trait_ref)) ,) ; } ty :: ClauseKind :: Projection (pred) => { let pred = bound_predicate . rebind (pred) ; let references_self = match pred . skip_binder () . term . kind () { ty :: TermKind :: Ty (ty) => ty . walk () . any (| arg | arg == dummy_self . into ()) , ty :: TermKind :: Const (_) => false , } ; if ! references_self { let key = (pred . skip_binder () . projection_term . def_id , tcx . anonymize_bound_vars (pred . map_bound (| proj | proj . projection_term . trait_ref (tcx)) ,) ,) ; if ! projection_bounds . contains_key (& key) { projection_bounds . insert (key , (pred , supertrait_span)) ; } } self . check_elaborated_projection_mentions_input_lifetimes (pred , * spans . first () . unwrap () , supertrait_span ,) ; } _ => () , } } } for & (projection_bound , span) in projection_bounds . values () { let def_id = projection_bound . item_def_id () ; if tcx . generics_require_sized_self (def_id) { tcx . emit_node_span_lint (UNUSED_ASSOCIATED_TYPE_BOUNDS , hir_id , span , crate :: errors :: UnusedAssociatedTypeBounds { span } ,) ; } } let mut missing_assoc_types = FxIndexSet :: default () ; let projection_bounds : Vec < _ > = ordered_associated_types . into_iter () . filter_map (| key | { if let Some (assoc) = projection_bounds . get (& key) { Some (* assoc) } else { if ! tcx . generics_require_sized_self (key . 0) { missing_assoc_types . insert (key) ; } None } }) . collect () ; if let Err (guar) = self . check_for_required_assoc_tys (principal_trait . as_ref () . map_or (smallvec ! [] , | (_ , spans) | spans . clone ()) , missing_assoc_types , potential_assoc_types , hir_bounds ,) { return Ty :: new_error (tcx , guar) ; } let mut duplicates = FxHashSet :: default () ; auto_traits . retain (| (trait_pred , _) | duplicates . insert (trait_pred . def_id ())) ; debug ! (? principal_trait) ; debug ! (? auto_traits) ; let principal_trait_ref = principal_trait . map (| (trait_pred , spans) | { trait_pred . map_bound (| trait_pred | { let trait_ref = trait_pred . trait_ref ; assert_eq ! (trait_pred . polarity , ty :: PredicatePolarity :: Positive) ; assert_eq ! (trait_ref . self_ty () , dummy_self) ; let span = * spans . first () . unwrap () ; let mut missing_type_params = vec ! [] ; let generics = tcx . generics_of (trait_ref . def_id) ; let args : Vec < _ > = trait_ref . args . iter () . enumerate () . skip (1) . map (| (index , arg) | { if arg . walk () . any (| arg | arg == dummy_self . into ()) { let param = & generics . own_params [index] ; missing_type_params . push (param . name) ; Ty :: new_misc_error (tcx) . into () } else { arg } }) . collect () ; let empty_generic_args = hir_bounds . iter () . any (| hir_bound | { hir_bound . trait_ref . path . res == Res :: Def (DefKind :: Trait , trait_ref . def_id) && hir_bound . span . contains (span) }) ; self . report_missing_type_params (missing_type_params , trait_ref . def_id , span , empty_generic_args ,) ; ty :: ExistentialPredicate :: Trait (ty :: ExistentialTraitRef :: new (tcx , trait_ref . def_id , args ,)) }) }) ; let existential_projections = projection_bounds . into_iter () . map (| (bound , _) | { bound . map_bound (| mut b | { assert_eq ! (b . projection_term . self_ty () , dummy_self) ; let references_self = b . projection_term . args . iter () . skip (1) . any (| arg | { if arg . walk () . any (| arg | arg == dummy_self . into ()) { return true ; } false }) ; if references_self { let guar = tcx . dcx () . span_delayed_bug (span , "trait object projection bounds reference `Self`") ; b . projection_term = replace_dummy_self_with_error (tcx , b . projection_term , guar) ; } ty :: ExistentialPredicate :: Projection (ty :: ExistentialProjection :: erase_self_ty (tcx , b ,)) }) }) ; let mut auto_trait_predicates : Vec < _ > = auto_traits . into_iter () . map (| (trait_pred , _) | { assert_eq ! (trait_pred . polarity () , ty :: PredicatePolarity :: Positive) ; assert_eq ! (trait_pred . self_ty () . skip_binder () , dummy_self) ; ty :: Binder :: dummy (ty :: ExistentialPredicate :: AutoTrait (trait_pred . def_id ())) }) . collect () ; auto_trait_predicates . dedup () ; let mut v = principal_trait_ref . into_iter () . chain (existential_projections) . chain (auto_trait_predicates) . collect :: < SmallVec < [_ ; 8] > > () ; v . sort_by (| a , b | a . skip_binder () . stable_cmp (tcx , & b . skip_binder ())) ; let existential_predicates = tcx . mk_poly_existential_predicates (& v) ; let region_bound = if ! lifetime . is_elided () { self . lower_lifetime (lifetime , RegionInferReason :: ExplicitObjectLifetime) } else { self . compute_object_lifetime_bound (span , existential_predicates) . unwrap_or_else (| | { if tcx . named_bound_var (lifetime . hir_id) . is_some () { self . lower_lifetime (lifetime , RegionInferReason :: ExplicitObjectLifetime) } else { let reason = if let hir :: LifetimeKind :: ImplicitObjectLifetimeDefault = lifetime . kind { if let hir :: Node :: Ty (hir :: Ty { kind : hir :: TyKind :: Ref (parent_lifetime , _) , .. }) = tcx . parent_hir_node (hir_id) && tcx . named_bound_var (parent_lifetime . hir_id) . is_none () { RegionInferReason :: ExplicitObjectLifetime } else { RegionInferReason :: ObjectLifetimeDefault } } else { RegionInferReason :: ExplicitObjectLifetime } ; self . re_infer (span , reason) } }) } ; debug ! (? region_bound) ; Ty :: new_dynamic (tcx , existential_predicates , region_bound , representation) } # [doc = " Check that elaborating the principal of a trait ref doesn't lead to projections"] # [doc = " that are unconstrained. This can happen because an otherwise unconstrained"] # [doc = " *type variable* can be substituted with a type that has late-bound regions. See"] # [doc = " `elaborated-predicates-unconstrained-late-bound.rs` for a test."] fn check_elaborated_projection_mentions_input_lifetimes (& self , pred : ty :: PolyProjectionPredicate < 'tcx > , span : Span , supertrait_span : Span ,) { let tcx = self . tcx () ; let late_bound_in_projection_term = tcx . collect_constrained_late_bound_regions (pred . map_bound (| pred | pred . projection_term)) ; let late_bound_in_term = tcx . collect_referenced_late_bound_regions (pred . map_bound (| pred | pred . term)) ; debug ! (? late_bound_in_projection_term) ; debug ! (? late_bound_in_term) ; self . validate_late_bound_regions (late_bound_in_projection_term , late_bound_in_term , | br_name | { let item_name = tcx . item_name (pred . item_def_id ()) ; struct_span_code_err ! (self . dcx () , span , E0582 , "binding for associated type `{}` references {}, \
                             which does not appear in the trait input types" , item_name , br_name) . with_span_label (supertrait_span , "due to this supertrait") } ,) ; } }}}

macro_rules! replace_dummy_self_with_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function replace_dummy_self_with_error in module {}", module_path!());
    };
}

mkfn!{
    replace_dummy_self_with_error_introspect!();
    fn replace_dummy_self_with_error < 'tcx , T : TypeFoldable < TyCtxt < 'tcx > > > (tcx : TyCtxt < 'tcx > , t : T , guar : ErrorGuaranteed ,) -> T { t . fold_with (& mut BottomUpFolder { tcx , ty_op : | ty | { if ty == tcx . types . trait_object_dummy_self { Ty :: new_error (tcx , guar) } else { ty } } , lt_op : | lt | lt , ct_op : | ct | ct , }) }
}