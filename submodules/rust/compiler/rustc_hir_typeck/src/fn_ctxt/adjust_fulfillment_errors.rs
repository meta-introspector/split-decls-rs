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
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_infer :: traits :: ObligationCauseCode ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeSuperVisitable , TypeVisitable , TypeVisitor } ;}
mkuse!{use rustc_span :: { Span , kw } ;}
mkuse!{use rustc_trait_selection :: traits ;}
mkuse!{use crate :: FnCtxt ;}
mkitem!{mkenum!{enum ClauseFlavor { # [doc = " Predicate comes from `predicates_of`."] Where , # [doc = " Predicate comes from `const_conditions`."] Const , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Eq , Debug)] enum ParamTerm { Ty (ty :: ParamTy) , Const (ty :: ParamConst) , }}}
mkitem!{mkimpl!{impl ParamTerm { fn index (self) -> usize { match self { ParamTerm :: Ty (ty) => ty . index as usize , ParamTerm :: Const (ct) => ct . index as usize , } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { pub (crate) fn adjust_fulfillment_error_for_expr_obligation (& self , error : & mut traits :: FulfillmentError < 'tcx > ,) -> bool { let (def_id , hir_id , idx , flavor) = match * error . obligation . cause . code () . peel_derives () { ObligationCauseCode :: WhereClauseInExpr (def_id , _ , hir_id , idx) => { (def_id , hir_id , idx , ClauseFlavor :: Where) } ObligationCauseCode :: HostEffectInExpr (def_id , _ , hir_id , idx) => { (def_id , hir_id , idx , ClauseFlavor :: Const) } _ => return false , } ; let uninstantiated_pred = match flavor { ClauseFlavor :: Where => { if let Some (pred) = self . tcx . predicates_of (def_id) . instantiate_identity (self . tcx) . predicates . into_iter () . nth (idx) { pred } else { return false ; } } ClauseFlavor :: Const => { if let Some ((pred , _)) = self . tcx . const_conditions (def_id) . instantiate_identity (self . tcx) . into_iter () . nth (idx) { pred . to_host_effect_clause (self . tcx , ty :: BoundConstness :: Maybe) } else { return false ; } } } ; let generics = self . tcx . generics_of (def_id) ; let (predicate_args , predicate_self_type_to_point_at) = match uninstantiated_pred . kind () . skip_binder () { ty :: ClauseKind :: Trait (pred) => { (pred . trait_ref . args . to_vec () , Some (pred . self_ty () . into ())) } ty :: ClauseKind :: HostEffect (pred) => { (pred . trait_ref . args . to_vec () , Some (pred . self_ty () . into ())) } ty :: ClauseKind :: Projection (pred) => (pred . projection_term . args . to_vec () , None) , ty :: ClauseKind :: ConstArgHasType (arg , ty) => (vec ! [ty . into () , arg . into ()] , None) , ty :: ClauseKind :: ConstEvaluatable (e) => (vec ! [e . into ()] , None) , _ => return false , } ; let find_param_matching = | matches : & dyn Fn (ParamTerm) -> bool | { predicate_args . iter () . find_map (| arg | { arg . walk () . find_map (| arg | { if let ty :: GenericArgKind :: Type (ty) = arg . kind () && let ty :: Param (param_ty) = * ty . kind () && matches (ParamTerm :: Ty (param_ty)) { Some (arg) } else if let ty :: GenericArgKind :: Const (ct) = arg . kind () && let ty :: ConstKind :: Param (param_ct) = ct . kind () && matches (ParamTerm :: Const (param_ct)) { Some (arg) } else { None } }) }) } ; let mut param_to_point_at = find_param_matching (& | param_term | { self . tcx . parent (generics . param_at (param_term . index () , self . tcx) . def_id) == def_id }) ; let mut fallback_param_to_point_at = find_param_matching (& | param_term | { self . tcx . parent (generics . param_at (param_term . index () , self . tcx) . def_id) != def_id && ! matches ! (param_term , ParamTerm :: Ty (ty) if ty . name == kw :: SelfUpper) }) ; let mut self_param_to_point_at = find_param_matching (& | param_term | matches ! (param_term , ParamTerm :: Ty (ty) if ty . name == kw :: SelfUpper) ,) ; if let traits :: FulfillmentErrorCode :: Ambiguity { .. } = error . code { fallback_param_to_point_at = None ; self_param_to_point_at = None ; param_to_point_at = self . find_ambiguous_parameter_in (def_id , error . root_obligation . predicate) ; } match self . tcx . hir_node (hir_id) { hir :: Node :: Expr (expr) => self . point_at_expr_if_possible (error , def_id , expr , predicate_self_type_to_point_at , param_to_point_at , fallback_param_to_point_at , self_param_to_point_at ,) , hir :: Node :: Ty (hir :: Ty { kind : hir :: TyKind :: Path (qpath) , .. }) => { for param in [predicate_self_type_to_point_at , param_to_point_at , fallback_param_to_point_at , self_param_to_point_at ,] . into_iter () . flatten () { if self . point_at_path_if_possible (error , def_id , param , & qpath) { return true ; } } false } _ => false , } } fn point_at_expr_if_possible (& self , error : & mut traits :: FulfillmentError < 'tcx > , callee_def_id : DefId , expr : & 'tcx hir :: Expr < 'tcx > , predicate_self_type_to_point_at : Option < ty :: GenericArg < 'tcx > > , param_to_point_at : Option < ty :: GenericArg < 'tcx > > , fallback_param_to_point_at : Option < ty :: GenericArg < 'tcx > > , self_param_to_point_at : Option < ty :: GenericArg < 'tcx > > ,) -> bool { if self . closure_span_overlaps_error (error , expr . span) { return false ; } match expr . kind { hir :: ExprKind :: Call (hir :: Expr { kind : hir :: ExprKind :: Path (qpath) , span : callee_span , .. } , args ,) => { if let Some (param) = predicate_self_type_to_point_at && self . point_at_path_if_possible (error , callee_def_id , param , & qpath) { return true ; } for param in [predicate_self_type_to_point_at , param_to_point_at , fallback_param_to_point_at , self_param_to_point_at ,] . into_iter () . flatten () { if self . blame_specific_arg_if_possible (error , callee_def_id , param , expr . hir_id , * callee_span , None , args ,) { return true ; } } for param in [param_to_point_at , fallback_param_to_point_at , self_param_to_point_at] . into_iter () . flatten () { if self . point_at_path_if_possible (error , callee_def_id , param , & qpath) { return true ; } } } hir :: ExprKind :: Path (qpath) => { if let hir :: Node :: Expr (call_expr @ hir :: Expr { kind : hir :: ExprKind :: Call (callee , ..) , .. } ,) = self . tcx . parent_hir_node (expr . hir_id) && callee . hir_id == expr . hir_id { return self . point_at_expr_if_possible (error , callee_def_id , call_expr , predicate_self_type_to_point_at , param_to_point_at , fallback_param_to_point_at , self_param_to_point_at ,) ; } if let Some (param) = predicate_self_type_to_point_at && self . point_at_path_if_possible (error , callee_def_id , param , & qpath) { return true ; } for param in [param_to_point_at , fallback_param_to_point_at , self_param_to_point_at] . into_iter () . flatten () { if self . point_at_path_if_possible (error , callee_def_id , param , & qpath) { return true ; } } } hir :: ExprKind :: MethodCall (segment , receiver , args , ..) => { if let Some (param) = predicate_self_type_to_point_at && self . point_at_generic_if_possible (error , callee_def_id , param , segment) { error . obligation . cause . map_code (| parent_code | { ObligationCauseCode :: FunctionArg { arg_hir_id : receiver . hir_id , call_hir_id : expr . hir_id , parent_code , } }) ; return true ; } for param in [param_to_point_at , fallback_param_to_point_at , self_param_to_point_at] . into_iter () . flatten () { if self . blame_specific_arg_if_possible (error , callee_def_id , param , expr . hir_id , segment . ident . span , Some (receiver) , args ,) { return true ; } } if let Some (param_to_point_at) = param_to_point_at && self . point_at_generic_if_possible (error , callee_def_id , param_to_point_at , segment ,) { return true ; } if self_param_to_point_at . is_some () { error . obligation . cause . span = receiver . span . find_ancestor_in_same_ctxt (error . obligation . cause . span) . unwrap_or (receiver . span) ; return true ; } } hir :: ExprKind :: Struct (qpath , fields , ..) => { if let Res :: Def (DefKind :: Struct | DefKind :: Variant , variant_def_id) = self . typeck_results . borrow () . qpath_res (qpath , expr . hir_id) { for param in [param_to_point_at , fallback_param_to_point_at , self_param_to_point_at] . into_iter () . flatten () { let refined_expr = self . point_at_field_if_possible (callee_def_id , param , variant_def_id , fields ,) ; match refined_expr { None => { } Some ((refined_expr , _)) => { error . obligation . cause . span = refined_expr . span . find_ancestor_in_same_ctxt (error . obligation . cause . span) . unwrap_or (refined_expr . span) ; return true ; } } } } for param in [predicate_self_type_to_point_at , param_to_point_at , fallback_param_to_point_at , self_param_to_point_at ,] . into_iter () . flatten () { if self . point_at_path_if_possible (error , callee_def_id , param , qpath) { return true ; } } } _ => { } } false } fn point_at_path_if_possible (& self , error : & mut traits :: FulfillmentError < 'tcx > , def_id : DefId , arg : ty :: GenericArg < 'tcx > , qpath : & hir :: QPath < 'tcx > ,) -> bool { match qpath { hir :: QPath :: Resolved (self_ty , path) => { for segment in path . segments . iter () . rev () { if let Res :: Def (kind , def_id) = segment . res && ! matches ! (kind , DefKind :: Mod | DefKind :: ForeignMod) && self . point_at_generic_if_possible (error , def_id , arg , segment) { return true ; } } if let Some (self_ty) = self_ty && let ty :: GenericArgKind :: Type (ty) = arg . kind () && ty == self . tcx . types . self_param { error . obligation . cause . span = self_ty . span . find_ancestor_in_same_ctxt (error . obligation . cause . span) . unwrap_or (self_ty . span) ; return true ; } } hir :: QPath :: TypeRelative (self_ty , segment) => { if self . point_at_generic_if_possible (error , def_id , arg , segment) { return true ; } if let ty :: GenericArgKind :: Type (ty) = arg . kind () && ty == self . tcx . types . self_param { error . obligation . cause . span = self_ty . span . find_ancestor_in_same_ctxt (error . obligation . cause . span) . unwrap_or (self_ty . span) ; return true ; } } _ => { } } false } fn point_at_generic_if_possible (& self , error : & mut traits :: FulfillmentError < 'tcx > , def_id : DefId , param_to_point_at : ty :: GenericArg < 'tcx > , segment : & hir :: PathSegment < 'tcx > ,) -> bool { let own_args = self . tcx . generics_of (def_id) . own_args (ty :: GenericArgs :: identity_for_item (self . tcx , def_id)) ; let Some (mut index) = own_args . iter () . position (| arg | * arg == param_to_point_at) else { return false ; } ; let segment_args = segment . args () . args ; if matches ! (own_args [0] . kind () , ty :: GenericArgKind :: Lifetime (_)) && segment_args . first () . is_some_and (| arg | arg . is_ty_or_const ()) && let Some (offset) = own_args . iter () . position (| arg | { matches ! (arg . kind () , ty :: GenericArgKind :: Type (_) | ty :: GenericArgKind :: Const (_)) }) && let Some (new_index) = index . checked_sub (offset) { index = new_index ; } let Some (arg) = segment_args . get (index) else { return false ; } ; error . obligation . cause . span = arg . span () . find_ancestor_in_same_ctxt (error . obligation . cause . span) . unwrap_or (arg . span ()) ; true } fn find_ambiguous_parameter_in < T : TypeVisitable < TyCtxt < 'tcx > > > (& self , item_def_id : DefId , t : T ,) -> Option < ty :: GenericArg < 'tcx > > { struct FindAmbiguousParameter < 'a , 'tcx > (& 'a FnCtxt < 'a , 'tcx > , DefId) ; impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for FindAmbiguousParameter < '_ , 'tcx > { type Result = ControlFlow < ty :: GenericArg < 'tcx > > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { if let ty :: Infer (ty :: TyVar (vid)) = * ty . kind () && let Some (def_id) = self . 0 . type_var_origin (vid) . param_def_id && let generics = self . 0 . tcx . generics_of (self . 1) && let Some (index) = generics . param_def_id_to_index (self . 0 . tcx , def_id) && let Some (arg) = ty :: GenericArgs :: identity_for_item (self . 0 . tcx , self . 1) . get (index as usize) { ControlFlow :: Break (* arg) } else { ty . super_visit_with (self) } } } t . visit_with (& mut FindAmbiguousParameter (self , item_def_id)) . break_value () } fn closure_span_overlaps_error (& self , error : & traits :: FulfillmentError < 'tcx > , span : Span ,) -> bool { if let traits :: FulfillmentErrorCode :: Select (traits :: SelectionError :: SignatureMismatch (box traits :: SignatureMismatchData { expected_trait_ref , .. } ,)) = error . code && let ty :: Closure (def_id , _) | ty :: Coroutine (def_id , ..) = expected_trait_ref . self_ty () . kind () && span . overlaps (self . tcx . def_span (* def_id)) { true } else { false } } fn point_at_field_if_possible (& self , def_id : DefId , param_to_point_at : ty :: GenericArg < 'tcx > , variant_def_id : DefId , expr_fields : & [hir :: ExprField < 'tcx >] ,) -> Option < (& 'tcx hir :: Expr < 'tcx > , Ty < 'tcx >) > { let def = self . tcx . adt_def (def_id) ; let identity_args = ty :: GenericArgs :: identity_for_item (self . tcx , def_id) ; let fields_referencing_param : Vec < _ > = def . variant_with_id (variant_def_id) . fields . iter () . filter (| field | { let field_ty = field . ty (self . tcx , identity_args) ; find_param_in_ty (field_ty . into () , param_to_point_at) }) . collect () ; if let [field] = fields_referencing_param . as_slice () { for expr_field in expr_fields { if self . tcx . adjust_ident (expr_field . ident , variant_def_id) == field . ident (self . tcx) { return Some ((expr_field . expr , self . tcx . type_of (field . did) . instantiate_identity () ,)) ; } } } None } # [doc = " - `blame_specific_*` means that the function will recursively traverse the expression,"] # [doc = " looking for the most-specific-possible span to blame."] # [doc = ""] # [doc = " - `point_at_*` means that the function will only go \"one level\", pointing at the specific"] # [doc = " expression mentioned."] # [doc = ""] # [doc = " `blame_specific_arg_if_possible` will find the most-specific expression anywhere inside"] # [doc = " the provided function call expression, and mark it as responsible for the fulfillment"] # [doc = " error."] fn blame_specific_arg_if_possible (& self , error : & mut traits :: FulfillmentError < 'tcx > , def_id : DefId , param_to_point_at : ty :: GenericArg < 'tcx > , call_hir_id : hir :: HirId , callee_span : Span , receiver : Option < & 'tcx hir :: Expr < 'tcx > > , args : & 'tcx [hir :: Expr < 'tcx >] ,) -> bool { let ty = self . tcx . type_of (def_id) . instantiate_identity () ; if ! ty . is_fn () { return false ; } let sig = ty . fn_sig (self . tcx) . skip_binder () ; let args_referencing_param : Vec < _ > = sig . inputs () . iter () . enumerate () . filter (| (_ , ty) | find_param_in_ty ((* * ty) . into () , param_to_point_at)) . collect () ; if let [(idx , _)] = args_referencing_param . as_slice () && let Some (arg) = receiver . map_or (args . get (* idx) , | rcvr | { if * idx == 0 { Some (rcvr) } else { args . get (* idx - 1) } }) { error . obligation . cause . span = arg . span . find_ancestor_in_same_ctxt (error . obligation . cause . span) . unwrap_or (arg . span) ; if let hir :: Node :: Expr (arg_expr) = self . tcx . hir_node (arg . hir_id) { self . blame_specific_expr_if_possible (error , arg_expr) } error . obligation . cause . map_code (| parent_code | ObligationCauseCode :: FunctionArg { arg_hir_id : arg . hir_id , call_hir_id , parent_code , }) ; return true ; } else if args_referencing_param . len () > 0 { error . obligation . cause . span = callee_span ; } false } # [doc = "\n     * Recursively searches for the most-specific blameable expression.\n     * For example, if you have a chain of constraints like:\n     * - want `Vec<i32>: Copy`\n     * - because `Option<Vec<i32>>: Copy` needs `Vec<i32>: Copy` because `impl <T: Copy> Copy for Option<T>`\n     * - because `(Option<Vec<i32>, bool)` needs `Option<Vec<i32>>: Copy` because `impl <A: Copy, B: Copy> Copy for (A, B)`\n     * then if you pass in `(Some(vec![1, 2, 3]), false)`, this helper `point_at_specific_expr_if_possible`\n     * will find the expression `vec![1, 2, 3]` as the \"most blameable\" reason for this missing constraint.\n     *\n     * This function only updates the error span.\n     "] pub (crate) fn blame_specific_expr_if_possible (& self , error : & mut traits :: FulfillmentError < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > ,) { let expr = match self . blame_specific_expr_if_possible_for_obligation_cause_code (error . obligation . cause . code () , expr ,) { Ok (expr) => expr , Err (expr) => expr , } ; error . obligation . cause . span = expr . span . find_ancestor_in_same_ctxt (error . obligation . cause . span) . unwrap_or (error . obligation . cause . span) ; } fn blame_specific_expr_if_possible_for_obligation_cause_code (& self , obligation_cause_code : & traits :: ObligationCauseCode < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > ,) -> Result < & 'tcx hir :: Expr < 'tcx > , & 'tcx hir :: Expr < 'tcx > > { match obligation_cause_code { traits :: ObligationCauseCode :: WhereClauseInExpr (_ , _ , _ , _) | ObligationCauseCode :: HostEffectInExpr (..) => { Ok (expr) } traits :: ObligationCauseCode :: ImplDerived (impl_derived) => self . blame_specific_expr_if_possible_for_derived_predicate_obligation (impl_derived , expr ,) , _ => { Err (expr) } } } # [doc = " We want to achieve the error span in the following example:"] # [doc = ""] # [doc = " ```ignore (just for demonstration)"] # [doc = " struct Burrito<Filling> {"] # [doc = "   filling: Filling,"] # [doc = " }"] # [doc = " impl <Filling: Delicious> Delicious for Burrito<Filling> {}"] # [doc = " fn eat_delicious_food<Food: Delicious>(_food: Food) {}"] # [doc = ""] # [doc = " fn will_type_error() {"] # [doc = "   eat_delicious_food(Burrito { filling: Kale });"] # [doc = " } //                                    ^--- The trait bound `Kale: Delicious`"] # [doc = "   //                                         is not satisfied"] # [doc = " ```"] # [doc = ""] # [doc = " Without calling this function, the error span will cover the entire argument expression."] # [doc = ""] # [doc = " Before we do any of this logic, we recursively call `point_at_specific_expr_if_possible` on the parent"] # [doc = " obligation. Hence we refine the `expr` \"outwards-in\" and bail at the first kind of expression/impl we don't recognize."] # [doc = ""] # [doc = " This function returns a `Result<&Expr, &Expr>` - either way, it returns the `Expr` whose span should be"] # [doc = " reported as an error. If it is `Ok`, then it means it refined successful. If it is `Err`, then it may be"] # [doc = " only a partial success - but it cannot be refined even further."] fn blame_specific_expr_if_possible_for_derived_predicate_obligation (& self , obligation : & traits :: ImplDerivedCause < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > ,) -> Result < & 'tcx hir :: Expr < 'tcx > , & 'tcx hir :: Expr < 'tcx > > { let expr = self . blame_specific_expr_if_possible_for_obligation_cause_code (& * obligation . derived . parent_code , expr ,) ? ; let impl_trait_self_ref = if self . tcx . is_trait_alias (obligation . impl_or_alias_def_id) { ty :: TraitRef :: new_from_args (self . tcx , obligation . impl_or_alias_def_id , ty :: GenericArgs :: identity_for_item (self . tcx , obligation . impl_or_alias_def_id) ,) } else { self . tcx . impl_trait_ref (obligation . impl_or_alias_def_id) . map (| impl_def | impl_def . skip_binder ()) . ok_or (expr) ? } ; let impl_self_ty : Ty < 'tcx > = impl_trait_self_ref . self_ty () ; let impl_predicates : ty :: GenericPredicates < 'tcx > = self . tcx . predicates_of (obligation . impl_or_alias_def_id) ; let Some (impl_predicate_index) = obligation . impl_def_predicate_index else { return Err (expr) ; } ; if impl_predicate_index >= impl_predicates . predicates . len () { return Err (expr) ; } match impl_predicates . predicates [impl_predicate_index] . 0 . kind () . skip_binder () { ty :: ClauseKind :: Trait (broken_trait) => { self . blame_specific_part_of_expr_corresponding_to_generic_param (broken_trait . trait_ref . self_ty () . into () , expr , impl_self_ty . into () ,) } _ => Err (expr) , } } # [doc = " Drills into `expr` to arrive at the equivalent location of `find_generic_param` in `in_ty`."] # [doc = " For example, given"] # [doc = " - expr: `(Some(vec![1, 2, 3]), false)`"] # [doc = " - param: `T`"] # [doc = " - in_ty: `(Option<Vec<T>, bool)`"] # [doc = " we would drill until we arrive at `vec![1, 2, 3]`."] # [doc = ""] # [doc = " If successful, we return `Ok(refined_expr)`. If unsuccessful, we return `Err(partially_refined_expr`),"] # [doc = " which will go as far as possible. For example, given `(foo(), false)` instead, we would drill to"] # [doc = " `foo()` and then return `Err(\"foo()\")`."] # [doc = ""] # [doc = " This means that you can (and should) use the `?` try operator to chain multiple calls to this"] # [doc = " function with different types, since you can only continue drilling the second time if you"] # [doc = " succeeded the first time."] fn blame_specific_part_of_expr_corresponding_to_generic_param (& self , param : ty :: GenericArg < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > , in_ty : ty :: GenericArg < 'tcx > ,) -> Result < & 'tcx hir :: Expr < 'tcx > , & 'tcx hir :: Expr < 'tcx > > { if param == in_ty { return Ok (expr) ; } let ty :: GenericArgKind :: Type (in_ty) = in_ty . kind () else { return Err (expr) ; } ; if let (hir :: ExprKind :: AddrOf (_borrow_kind , _borrow_mutability , borrowed_expr) , ty :: Ref (_ty_region , ty_ref_type , _ty_mutability) ,) = (& expr . kind , in_ty . kind ()) { return self . blame_specific_part_of_expr_corresponding_to_generic_param (param , borrowed_expr , (* ty_ref_type) . into () ,) ; } if let (hir :: ExprKind :: Tup (expr_elements) , ty :: Tuple (in_ty_elements)) = (& expr . kind , in_ty . kind ()) { if in_ty_elements . len () != expr_elements . len () { return Err (expr) ; } let Some ((drill_expr , drill_ty)) = is_iterator_singleton (expr_elements . iter () . zip (in_ty_elements . iter ()) . filter (| (_expr_elem , in_ty_elem) | find_param_in_ty ((* in_ty_elem) . into () , param) ,)) else { return Err (expr) ; } ; return self . blame_specific_part_of_expr_corresponding_to_generic_param (param , drill_expr , drill_ty . into () ,) ; } if let (hir :: ExprKind :: Struct (expr_struct_path , expr_struct_fields , _expr_struct_rest) , ty :: Adt (in_ty_adt , in_ty_adt_generic_args) ,) = (& expr . kind , in_ty . kind ()) { let Res :: Def (expr_struct_def_kind , expr_struct_def_id) = self . typeck_results . borrow () . qpath_res (expr_struct_path , expr . hir_id) else { return Err (expr) ; } ; let variant_def_id = match expr_struct_def_kind { DefKind :: Struct => { if in_ty_adt . did () != expr_struct_def_id { return Err (expr) ; } expr_struct_def_id } DefKind :: Variant => { if in_ty_adt . did () != self . tcx . parent (expr_struct_def_id) { return Err (expr) ; } expr_struct_def_id } _ => { return Err (expr) ; } } ; let Some ((drill_generic_index , generic_argument_type)) = is_iterator_singleton (in_ty_adt_generic_args . iter () . enumerate () . filter (| (_index , in_ty_generic) | find_param_in_ty (* in_ty_generic , param)) ,) else { return Err (expr) ; } ; let struct_generic_parameters : & ty :: Generics = self . tcx . generics_of (in_ty_adt . did ()) ; if drill_generic_index >= struct_generic_parameters . own_params . len () { return Err (expr) ; } let param_to_point_at_in_struct = self . tcx . mk_param_from_def (struct_generic_parameters . param_at (drill_generic_index , self . tcx) ,) ; let (field_expr , field_type) = self . point_at_field_if_possible (in_ty_adt . did () , param_to_point_at_in_struct , variant_def_id , expr_struct_fields ,) . ok_or (expr) ? ; let expr = self . blame_specific_part_of_expr_corresponding_to_generic_param (param_to_point_at_in_struct , field_expr , field_type . into () ,) ? ; return self . blame_specific_part_of_expr_corresponding_to_generic_param (param , expr , generic_argument_type ,) ; } if let (hir :: ExprKind :: Call (expr_callee , expr_args) , ty :: Adt (in_ty_adt , in_ty_adt_generic_args) ,) = (& expr . kind , in_ty . kind ()) { let hir :: ExprKind :: Path (expr_callee_path) = & expr_callee . kind else { return Err (expr) ; } ; let Res :: Def (expr_struct_def_kind , expr_ctor_def_id) = self . typeck_results . borrow () . qpath_res (expr_callee_path , expr_callee . hir_id) else { return Err (expr) ; } ; let variant_def_id = match expr_struct_def_kind { DefKind :: Ctor (hir :: def :: CtorOf :: Struct , hir :: def :: CtorKind :: Fn) => { if in_ty_adt . did () != self . tcx . parent (expr_ctor_def_id) { return Err (expr) ; } self . tcx . parent (expr_ctor_def_id) } DefKind :: Ctor (hir :: def :: CtorOf :: Variant , hir :: def :: CtorKind :: Fn) => { if in_ty_adt . did () == self . tcx . parent (self . tcx . parent (expr_ctor_def_id)) { self . tcx . parent (expr_ctor_def_id) } else { return Err (expr) ; } } _ => { return Err (expr) ; } } ; let Some ((drill_generic_index , generic_argument_type)) = is_iterator_singleton (in_ty_adt_generic_args . iter () . enumerate () . filter (| (_index , in_ty_generic) | find_param_in_ty (* in_ty_generic , param)) ,) else { return Err (expr) ; } ; let struct_generic_parameters : & ty :: Generics = self . tcx . generics_of (in_ty_adt . did ()) ; if drill_generic_index >= struct_generic_parameters . own_params . len () { return Err (expr) ; } let param_to_point_at_in_struct = self . tcx . mk_param_from_def (struct_generic_parameters . param_at (drill_generic_index , self . tcx) ,) ; let Some ((field_index , field_type)) = is_iterator_singleton (in_ty_adt . variant_with_id (variant_def_id) . fields . iter () . map (| field | field . ty (self . tcx , * in_ty_adt_generic_args)) . enumerate () . filter (| (_index , field_type) | find_param_in_ty ((* field_type) . into () , param)) ,) else { return Err (expr) ; } ; if field_index >= expr_args . len () { return Err (expr) ; } let expr = self . blame_specific_part_of_expr_corresponding_to_generic_param (param_to_point_at_in_struct , & expr_args [field_index] , field_type . into () ,) ? ; return self . blame_specific_part_of_expr_corresponding_to_generic_param (param , expr , generic_argument_type ,) ; } Err (expr) } }}}

macro_rules! find_param_in_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_param_in_ty in module {}", module_path!());
    };
}

mkfn!{
    find_param_in_ty_introspect!();
    # [doc = " Traverses the given ty (either a `ty::Ty` or a `ty::GenericArg`) and searches for references"] # [doc = " to the given `param_to_point_at`. Returns `true` if it finds any use of the param."] fn find_param_in_ty < 'tcx > (ty : ty :: GenericArg < 'tcx > , param_to_point_at : ty :: GenericArg < 'tcx > ,) -> bool { let mut walk = ty . walk () ; while let Some (arg) = walk . next () { if arg == param_to_point_at { return true ; } if let ty :: GenericArgKind :: Type (ty) = arg . kind () && let ty :: Alias (ty :: Projection | ty :: Inherent , ..) = ty . kind () { walk . skip_current_subtree () ; } } false }
}

macro_rules! is_iterator_singleton_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_iterator_singleton in module {}", module_path!());
    };
}

mkfn!{
    is_iterator_singleton_introspect!();
    # [doc = " Returns `Some(iterator.next())` if it has exactly one item, and `None` otherwise."] fn is_iterator_singleton < T > (mut iterator : impl Iterator < Item = T >) -> Option < T > { match (iterator . next () , iterator . next ()) { (_ , Some (_)) => None , (first , _) => first , } }
}