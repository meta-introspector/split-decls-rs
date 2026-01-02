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
mkuse!{use rustc_errors :: { Applicability , Diag } ;}
mkuse!{use rustc_hir :: def :: { CtorOf , DefKind , Res } ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_hir :: { self as hir , ExprKind , HirId , PatKind } ;}
mkuse!{use rustc_hir_pretty :: ty_to_string ;}
mkuse!{use rustc_middle :: ty :: { self , Ty } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_trait_selection :: traits :: { MatchExpressionArmCause , ObligationCause , ObligationCauseCode , } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: coercion :: { AsCoercionSite , CoerceMany } ;}
mkuse!{use crate :: { Diverges , Expectation , FnCtxt , GatherLocalsVisitor , Needs } ;}
mkitem!{mkimpl!{impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { #[instrument (skip (self) , level = "debug" , ret)] pub (crate) fn check_expr_match (& self , expr : & 'tcx hir :: Expr < 'tcx > , scrut : & 'tcx hir :: Expr < 'tcx > , arms : & 'tcx [hir :: Arm < 'tcx >] , orig_expected : Expectation < 'tcx > , match_src : hir :: MatchSource ,) -> Ty < 'tcx > { let tcx = self . tcx ; let acrb = arms_contain_ref_bindings (arms) ; let scrutinee_ty = self . demand_scrutinee_type (scrut , acrb , arms . is_empty ()) ; debug ! (? scrutinee_ty) ; if arms . is_empty () { self . diverges . set (self . diverges . get () | Diverges :: always (expr . span)) ; return tcx . types . never ; } self . warn_arms_when_scrutinee_diverges (arms) ; let scrut_diverges = self . diverges . replace (Diverges :: Maybe) ; let scrut_span = scrut . span . find_ancestor_inside (expr . span) . unwrap_or (scrut . span) ; for arm in arms { GatherLocalsVisitor :: gather_from_arm (self , arm) ; self . check_pat_top (arm . pat , scrutinee_ty , Some (scrut_span) , Some (scrut) , None) ; } let mut all_arms_diverge = Diverges :: WarnedAlways ; let expected = orig_expected . try_structurally_resolve_and_adjust_for_branches (self , expr . span) ; debug ! (? expected) ; let mut coercion = { let coerce_first = match expected { Expectation :: ExpectHasType (ety) if ety != tcx . types . unit => ety , _ => self . next_ty_var (expr . span) , } ; CoerceMany :: with_coercion_sites (coerce_first , arms) } ; let mut prior_non_diverging_arms = vec ! [] ; let mut prior_arm = None ; for arm in arms { self . diverges . set (Diverges :: Maybe) ; if let Some (e) = & arm . guard { self . check_expr_has_type_or_error (e , tcx . types . bool , | _ | { }) ; } let arm_ty = self . check_expr_with_expectation (arm . body , expected) ; all_arms_diverge &= self . diverges . get () ; let tail_defines_return_position_impl_trait = self . return_position_impl_trait_from_match_expectation (orig_expected) ; let (arm_block_id , arm_span) = if let hir :: ExprKind :: Block (blk , _) = arm . body . kind { (Some (blk . hir_id) , self . find_block_span (blk)) } else { (None , arm . body . span) } ; let code = match prior_arm { None => ObligationCauseCode :: BlockTailExpression (arm . body . hir_id , match_src) , Some ((prior_arm_block_id , prior_arm_ty , prior_arm_span)) => { ObligationCauseCode :: MatchExpressionArm (Box :: new (MatchExpressionArmCause { arm_block_id , arm_span , arm_ty , prior_arm_block_id , prior_arm_ty , prior_arm_span , scrut_span : scrut . span , expr_span : expr . span , source : match_src , prior_non_diverging_arms : prior_non_diverging_arms . clone () , tail_defines_return_position_impl_trait , })) } } ; let cause = self . cause (arm_span , code) ; coercion . coerce_inner (self , & cause , Some (arm . body) , arm_ty , | err | { self . explain_never_type_coerced_to_unit (err , arm , arm_ty , prior_arm , expr) ; } , false ,) ; if ! arm_ty . is_never () { prior_arm = Some ((arm_block_id , arm_ty , arm_span)) ; prior_non_diverging_arms . push (arm_span) ; if prior_non_diverging_arms . len () > 5 { prior_non_diverging_arms . remove (0) ; } } } if let (Diverges :: Always { .. } , hir :: MatchSource :: Normal) = (all_arms_diverge , match_src) { all_arms_diverge = Diverges :: Always { span : expr . span , custom_note : Some ("any code following this `match` expression is unreachable, as all arms diverge" ,) , } ; } self . diverges . set (scrut_diverges | all_arms_diverge) ; coercion . complete (self) } fn explain_never_type_coerced_to_unit (& self , err : & mut Diag < '_ > , arm : & hir :: Arm < 'tcx > , arm_ty : Ty < 'tcx > , prior_arm : Option < (Option < hir :: HirId > , Ty < 'tcx > , Span) > , expr : & hir :: Expr < 'tcx > ,) { if let hir :: ExprKind :: Block (block , _) = arm . body . kind && let Some (expr) = block . expr && let arm_tail_ty = self . node_ty (expr . hir_id) && arm_tail_ty . is_never () && ! arm_ty . is_never () { err . span_label (expr . span , format ! ("this expression is of type `!`, but it is coerced to `{arm_ty}` due to its \
                     surrounding expression" ,) ,) ; self . suggest_mismatched_types_on_tail (err , expr , arm_ty , prior_arm . map_or (arm_tail_ty , | (_ , ty , _) | ty) , expr . hir_id ,) ; } self . suggest_removing_semicolon_for_coerce (err , expr , arm_ty , prior_arm) } fn suggest_removing_semicolon_for_coerce (& self , diag : & mut Diag < '_ > , expr : & hir :: Expr < 'tcx > , arm_ty : Ty < 'tcx > , prior_arm : Option < (Option < hir :: HirId > , Ty < 'tcx > , Span) > ,) { let Some (body) = self . tcx . hir_maybe_body_owned_by (self . body_id) else { return ; } ; let hir :: ExprKind :: Block (block , _) = body . value . kind else { return ; } ; let Some (hir :: Stmt { kind : hir :: StmtKind :: Semi (last_expr) , span : semi_span , .. }) = block . innermost_block () . stmts . last () else { return ; } ; if last_expr . hir_id != expr . hir_id { return ; } let Some (ret) = self . tcx . hir_node_by_def_id (self . body_id) . fn_decl () . map (| decl | decl . output . span ()) else { return ; } ; let can_coerce_to_return_ty = match self . ret_coercion . as_ref () { Some (ret_coercion) => { let ret_ty = ret_coercion . borrow () . expected_ty () ; let ret_ty = self . infcx . shallow_resolve (ret_ty) ; self . may_coerce (arm_ty , ret_ty) && prior_arm . is_none_or (| (_ , ty , _) | self . may_coerce (ty , ret_ty)) && ! matches ! (ret_ty . kind () , ty :: Alias (ty :: Opaque , ..)) } _ => false , } ; if ! can_coerce_to_return_ty { return ; } let semi = expr . span . shrink_to_hi () . with_hi (semi_span . hi ()) ; let sugg = crate :: errors :: RemoveSemiForCoerce { expr : expr . span , ret , semi } ; diag . subdiagnostic (sugg) ; } #[doc = " When the previously checked expression (the scrutinee) diverges,"] #[doc = " warn the user about the match arms being unreachable."] fn warn_arms_when_scrutinee_diverges (& self , arms : & 'tcx [hir :: Arm < 'tcx >]) { for arm in arms { self . warn_if_unreachable (arm . body . hir_id , arm . body . span , "arm") ; } } #[doc = " Handle the fallback arm of a desugared if(-let) like a missing else."] #[doc = ""] #[doc = " Returns `true` if there was an error forcing the coercion to the `()` type."] pub (super) fn if_fallback_coercion < T > (& self , if_span : Span , cond_expr : & 'tcx hir :: Expr < 'tcx > , then_expr : & 'tcx hir :: Expr < 'tcx > , coercion : & mut CoerceMany < 'tcx , '_ , T > ,) -> bool where T : AsCoercionSite , { let hir_id = self . tcx . parent_hir_id (self . tcx . parent_hir_id (then_expr . hir_id)) ; let ret_reason = self . maybe_get_coercion_reason (hir_id , if_span) ; let cause = self . cause (if_span , ObligationCauseCode :: IfExpressionWithNoElse) ; let mut error = false ; coercion . coerce_forced_unit (self , & cause , | err | self . explain_if_expr (err , ret_reason , if_span , cond_expr , then_expr , & mut error) , false ,) ; error } #[doc = " Explain why `if` expressions without `else` evaluate to `()` and detect likely irrefutable"] #[doc = " `if let PAT = EXPR {}` expressions that could be turned into `let PAT = EXPR;`."] fn explain_if_expr (& self , err : & mut Diag < '_ > , ret_reason : Option < (Span , String) > , if_span : Span , cond_expr : & 'tcx hir :: Expr < 'tcx > , then_expr : & 'tcx hir :: Expr < 'tcx > , error : & mut bool ,) { if let Some ((if_span , msg)) = ret_reason { err . span_label (if_span , msg) ; } else if let ExprKind :: Block (block , _) = then_expr . kind && let Some (expr) = block . expr { err . span_label (expr . span , "found here") ; } err . note ("`if` expressions without `else` evaluate to `()`") ; err . help ("consider adding an `else` block that evaluates to the expected type") ; * error = true ; if let ExprKind :: Let (hir :: LetExpr { span , pat , init , .. }) = cond_expr . kind && let ExprKind :: Block (block , _) = then_expr . kind && let PatKind :: TupleStruct (qpath , ..) | PatKind :: Struct (qpath , ..) = pat . kind && let hir :: QPath :: Resolved (_ , path) = qpath { match path . res { Res :: Def (DefKind :: Ctor (CtorOf :: Struct , _) , _) => { } Res :: Def (DefKind :: Ctor (CtorOf :: Variant , _) , def_id) if self . tcx . adt_def (self . tcx . parent (self . tcx . parent (def_id))) . variants () . len () == 1 => { } _ => return , } let mut sugg = vec ! [(if_span . until (* span) , String :: new ()) ,] ; match (block . stmts , block . expr) { ([first , ..] , Some (expr)) => { let padding = self . tcx . sess . source_map () . indentation_before (first . span) . unwrap_or_else (| | String :: new ()) ; sugg . extend ([(init . span . between (first . span) , format ! (";\n{padding}")) , (expr . span . shrink_to_hi () . with_hi (block . span . hi ()) , String :: new ()) ,]) ; } ([] , Some (expr)) => { let padding = self . tcx . sess . source_map () . indentation_before (expr . span) . unwrap_or_else (| | String :: new ()) ; sugg . extend ([(init . span . between (expr . span) , format ! (";\n{padding}")) , (expr . span . shrink_to_hi () . with_hi (block . span . hi ()) , String :: new ()) ,]) ; } (_ , None) => return , } err . multipart_suggestion ("consider using an irrefutable `let` binding instead" , sugg , Applicability :: MaybeIncorrect ,) ; } } pub (crate) fn maybe_get_coercion_reason (& self , hir_id : hir :: HirId , sp : Span ,) -> Option < (Span , String) > { let node = self . tcx . hir_node (hir_id) ; if let hir :: Node :: Block (block) = node { let parent = self . tcx . parent_hir_node (self . tcx . parent_hir_id (block . hir_id)) ; if let (Some (expr) , hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Fn { .. } , .. })) = (& block . expr , parent) { if expr . span == sp { return self . get_fn_decl (hir_id) . map (| (_ , fn_decl) | { let (ty , span) = match fn_decl . output { hir :: FnRetTy :: DefaultReturn (span) => ("()" . to_string () , span) , hir :: FnRetTy :: Return (ty) => (ty_to_string (& self . tcx , ty) , ty . span) , } ; (span , format ! ("expected `{ty}` because of this return type")) }) ; } } } if let hir :: Node :: LetStmt (hir :: LetStmt { ty : Some (_) , pat , .. }) = node { return Some ((pat . span , "expected because of this assignment" . to_string ())) ; } None } pub (crate) fn if_cause (& self , expr_id : HirId , else_expr : & 'tcx hir :: Expr < 'tcx > , tail_defines_return_position_impl_trait : Option < LocalDefId > ,) -> ObligationCause < 'tcx > { let error_sp = self . find_block_span_from_hir_id (else_expr . hir_id) ; self . cause (error_sp , ObligationCauseCode :: IfExpression { expr_id , tail_defines_return_position_impl_trait } ,) } pub (super) fn demand_scrutinee_type (& self , scrut : & 'tcx hir :: Expr < 'tcx > , contains_ref_bindings : Option < hir :: Mutability > , no_arms : bool ,) -> Ty < 'tcx > { if let Some (m) = contains_ref_bindings { self . check_expr_with_needs (scrut , Needs :: maybe_mut_place (m)) } else if no_arms { self . check_expr (scrut) } else { let scrut_ty = self . next_ty_var (scrut . span) ; self . check_expr_has_type_or_error (scrut , scrut_ty , | _ | { }) ; scrut_ty } } pub (crate) fn return_position_impl_trait_from_match_expectation (& self , expectation : Expectation < 'tcx > ,) -> Option < LocalDefId > { let expected_ty = expectation . to_option (self) ? ; let (def_id , args) = match * expected_ty . kind () { ty :: Alias (ty :: Opaque , alias_ty) => (alias_ty . def_id . as_local () ? , alias_ty . args) , ty :: Infer (ty :: TyVar (_)) => self . inner . borrow_mut () . opaque_types () . iter_opaque_types () . find (| (_ , v) | v . ty == expected_ty) . map (| (k , _) | (k . def_id , k . args)) ? , _ => return None , } ; let hir :: OpaqueTyOrigin :: FnReturn { parent : parent_def_id , .. } = self . tcx . local_opaque_ty_origin (def_id) else { return None ; } ; if & args [0 .. self . tcx . generics_of (parent_def_id) . count ()] != ty :: GenericArgs :: identity_for_item (self . tcx , parent_def_id) . as_slice () { return None ; } Some (def_id) } }}}

macro_rules! arms_contain_ref_bindings_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function arms_contain_ref_bindings in module {}", module_path!());
    };
}

mkfn!{
    arms_contain_ref_bindings_introspect!();
    fn arms_contain_ref_bindings < 'tcx > (arms : & 'tcx [hir :: Arm < 'tcx >]) -> Option < hir :: Mutability > { arms . iter () . filter_map (| a | a . pat . contains_explicit_ref_binding ()) . max () }
}