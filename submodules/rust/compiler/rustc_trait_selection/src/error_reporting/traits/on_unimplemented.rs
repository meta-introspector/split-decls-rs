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
mkuse!{use std :: path :: PathBuf ;}
mkuse!{use rustc_ast :: { LitKind , MetaItem , MetaItemInner , MetaItemKind , MetaItemLit } ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { ErrorGuaranteed , struct_span_code_err } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: { AttrArgs , Attribute } ;}
mkuse!{use rustc_macros :: LintDiagnostic ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: print :: PrintTraitRefExt ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgsRef , GenericParamDef , GenericParamDefKind , TyCtxt } ;}
mkuse!{use rustc_session :: lint :: builtin :: { MALFORMED_DIAGNOSTIC_ATTRIBUTES , MALFORMED_DIAGNOSTIC_FORMAT_LITERALS , } ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use tracing :: { debug , info } ;}
mkuse!{use super :: { ObligationCauseCode , PredicateObligation } ;}
mkuse!{use crate :: error_reporting :: TypeErrCtxt ;}
mkuse!{use crate :: error_reporting :: traits :: on_unimplemented_condition :: { ConditionOptions , OnUnimplementedCondition , } ;}
mkuse!{use crate :: error_reporting :: traits :: on_unimplemented_format :: { Ctx , FormatArgs , FormatString , FormatWarning , } ;}
mkuse!{use crate :: errors :: { InvalidOnClause , NoValueInOnUnimplemented } ;}
mkuse!{use crate :: infer :: InferCtxtExt ;}
mkitem!{mkimpl!{impl < 'tcx > TypeErrCtxt < '_ , 'tcx > { fn impl_similar_to (& self , trait_pred : ty :: PolyTraitPredicate < 'tcx > , obligation : & PredicateObligation < 'tcx > ,) -> Option < (DefId , GenericArgsRef < 'tcx >) > { let tcx = self . tcx ; let param_env = obligation . param_env ; self . enter_forall (trait_pred , | trait_pred | { let trait_self_ty = trait_pred . self_ty () ; let mut self_match_impls = vec ! [] ; let mut fuzzy_match_impls = vec ! [] ; self . tcx . for_each_relevant_impl (trait_pred . def_id () , trait_self_ty , | def_id | { let impl_args = self . fresh_args_for_item (obligation . cause . span , def_id) ; let impl_trait_ref = tcx . impl_trait_ref (def_id) . unwrap () . instantiate (tcx , impl_args) ; let impl_self_ty = impl_trait_ref . self_ty () ; if self . can_eq (param_env , trait_self_ty , impl_self_ty) { self_match_impls . push ((def_id , impl_args)) ; if iter :: zip (trait_pred . trait_ref . args . types () . skip (1) , impl_trait_ref . args . types () . skip (1) ,) . all (| (u , v) | self . fuzzy_match_tys (u , v , false) . is_some ()) { fuzzy_match_impls . push ((def_id , impl_args)) ; } } }) ; let impl_def_id_and_args = if let [impl_] = self_match_impls [..] { impl_ } else if let [impl_] = fuzzy_match_impls [..] { impl_ } else { return None ; } ; tcx . has_attr (impl_def_id_and_args . 0 , sym :: rustc_on_unimplemented) . then_some (impl_def_id_and_args) }) } #[doc = " Used to set on_unimplemented's `ItemContext`"] #[doc = " to be the enclosing (async) block/function/closure"] fn describe_enclosure (& self , def_id : LocalDefId) -> Option < & 'static str > { match self . tcx . hir_node_by_def_id (def_id) { hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Fn { .. } , .. }) => Some ("a function") , hir :: Node :: TraitItem (hir :: TraitItem { kind : hir :: TraitItemKind :: Fn (..) , .. }) => { Some ("a trait method") } hir :: Node :: ImplItem (hir :: ImplItem { kind : hir :: ImplItemKind :: Fn (..) , .. }) => { Some ("a method") } hir :: Node :: Expr (hir :: Expr { kind : hir :: ExprKind :: Closure (hir :: Closure { kind , .. }) , .. }) => Some (self . describe_closure (* kind)) , _ => None , } } pub fn on_unimplemented_note (& self , trait_pred : ty :: PolyTraitPredicate < 'tcx > , obligation : & PredicateObligation < 'tcx > , long_ty_path : & mut Option < PathBuf > ,) -> OnUnimplementedNote { if trait_pred . polarity () != ty :: PredicatePolarity :: Positive { return OnUnimplementedNote :: default () ; } let (def_id , args) = self . impl_similar_to (trait_pred , obligation) . unwrap_or_else (| | (trait_pred . def_id () , trait_pred . skip_binder () . trait_ref . args)) ; let trait_pred = trait_pred . skip_binder () ; let mut self_types = vec ! [] ; let mut generic_args : Vec < (Symbol , String) > = vec ! [] ; let mut crate_local = false ; let item_context = self . describe_enclosure (obligation . cause . body_id) . unwrap_or ("") ; let direct = match obligation . cause . code () { ObligationCauseCode :: BuiltinDerived (..) | ObligationCauseCode :: ImplDerived (..) | ObligationCauseCode :: WellFormedDerived (..) => false , _ => { true } } ; let from_desugaring = obligation . cause . span . desugaring_kind () ; let cause = if let ObligationCauseCode :: MainFunctionType = obligation . cause . code () { Some ("MainFunctionType" . to_string ()) } else { None } ; ty :: print :: with_no_trimmed_paths ! (ty :: print :: with_no_visible_paths ! ({ let generics = self . tcx . generics_of (def_id) ; let self_ty = trait_pred . self_ty () ; self_types . push (self_ty . to_string ()) ; if let Some (def) = self_ty . ty_adt_def () { self_types . push (self . tcx . type_of (def . did ()) . instantiate_identity () . to_string ()) ; } for GenericParamDef { name , kind , index , .. } in generics . own_params . iter () { let value = match kind { GenericParamDefKind :: Type { .. } | GenericParamDefKind :: Const { .. } => { args [* index as usize] . to_string () } GenericParamDefKind :: Lifetime => continue , } ; generic_args . push ((* name , value)) ; if let GenericParamDefKind :: Type { .. } = kind { let param_ty = args [* index as usize] . expect_ty () ; if let Some (def) = param_ty . ty_adt_def () { generic_args . push ((* name , self . tcx . type_of (def . did ()) . instantiate_identity () . to_string () ,)) ; } } } if let Some (true) = self_ty . ty_adt_def () . map (| def | def . did () . is_local ()) { crate_local = true ; } if self_ty . is_integral () { self_types . push ("{integral}" . to_owned ()) ; } if self_ty . is_array_slice () { self_types . push ("&[]" . to_owned ()) ; } if self_ty . is_fn () { let fn_sig = self_ty . fn_sig (self . tcx) ; let shortname = if let ty :: FnDef (def_id , _) = self_ty . kind () && self . tcx . codegen_fn_attrs (def_id) . safe_target_features { "#[target_feature] fn" } else { match fn_sig . safety () { hir :: Safety :: Safe => "fn" , hir :: Safety :: Unsafe => "unsafe fn" , } } ; self_types . push (shortname . to_owned ()) ; } if let ty :: Slice (aty) = self_ty . kind () { self_types . push ("[]" . to_owned ()) ; if let Some (def) = aty . ty_adt_def () { self_types . push (format ! ("[{}]" , self . tcx . type_of (def . did ()) . instantiate_identity ())) ; } if aty . is_integral () { self_types . push ("[{integral}]" . to_string ()) ; } } if let ty :: Array (aty , len) = self_ty . kind () { self_types . push ("[]" . to_string ()) ; let len = len . try_to_target_usize (self . tcx) ; self_types . push (format ! ("[{aty}; _]")) ; if let Some (n) = len { self_types . push (format ! ("[{aty}; {n}]")) ; } if let Some (def) = aty . ty_adt_def () { let def_ty = self . tcx . type_of (def . did ()) . instantiate_identity () ; self_types . push (format ! ("[{def_ty}; _]")) ; if let Some (n) = len { self_types . push (format ! ("[{def_ty}; {n}]")) ; } } if aty . is_integral () { self_types . push ("[{integral}; _]" . to_string ()) ; if let Some (n) = len { self_types . push (format ! ("[{{integral}}; {n}]")) ; } } } if let ty :: Dynamic (traits , _ , _) = self_ty . kind () { for t in traits . iter () { if let ty :: ExistentialPredicate :: Trait (trait_ref) = t . skip_binder () { self_types . push (self . tcx . def_path_str (trait_ref . def_id)) ; } } } if let ty :: Ref (_ , ref_ty , rustc_ast :: Mutability :: Not) = self_ty . kind () && let ty :: Slice (sty) = ref_ty . kind () && sty . is_integral () { self_types . push ("&[{integral}]" . to_owned ()) ; } })) ; let this = self . tcx . def_path_str (trait_pred . trait_ref . def_id) ; let trait_sugared = trait_pred . trait_ref . print_trait_sugared () ; let condition_options = ConditionOptions { self_types , from_desugaring , cause , crate_local , direct , generic_args , } ; let generic_args = self . tcx . generics_of (trait_pred . trait_ref . def_id) . own_params . iter () . filter_map (| param | { let value = match param . kind { GenericParamDefKind :: Type { .. } | GenericParamDefKind :: Const { .. } => { if let Some (ty) = trait_pred . trait_ref . args [param . index as usize] . as_type () { self . tcx . short_string (ty , long_ty_path) } else { trait_pred . trait_ref . args [param . index as usize] . to_string () } } GenericParamDefKind :: Lifetime => return None , } ; let name = param . name ; Some ((name , value)) }) . collect () ; let format_args = FormatArgs { this , trait_sugared , generic_args , item_context } ; if let Ok (Some (command)) = OnUnimplementedDirective :: of_item (self . tcx , def_id) { command . evaluate (self . tcx , trait_pred . trait_ref , & condition_options , & format_args) } else { OnUnimplementedNote :: default () } } }}}
mkitem!{mkstruct!{#[doc = " Represents a format string in a on_unimplemented attribute,"] #[doc = " like the \"content\" in `#[diagnostic::on_unimplemented(message = \"content\")]`"] #[derive (Clone , Debug)] pub struct OnUnimplementedFormatString { #[doc = " Symbol of the format string, i.e. `\"content\"`"] symbol : Symbol , #[doc = "The span of the format string, i.e. `\"content\"`"] span : Span , is_diagnostic_namespace_variant : bool , }}}
mkitem!{mkstruct!{#[derive (Debug)] pub struct OnUnimplementedDirective { condition : Option < OnUnimplementedCondition > , subcommands : Vec < OnUnimplementedDirective > , message : Option < (Span , OnUnimplementedFormatString) > , label : Option < (Span , OnUnimplementedFormatString) > , notes : Vec < OnUnimplementedFormatString > , parent_label : Option < OnUnimplementedFormatString > , append_const_msg : Option < AppendConstMessage > , }}}
mkitem!{mkstruct!{#[doc = " For the `#[rustc_on_unimplemented]` attribute"] #[derive (Default)] pub struct OnUnimplementedNote { pub message : Option < String > , pub label : Option < String > , pub notes : Vec < String > , pub parent_label : Option < String > , pub append_const_msg : Option < AppendConstMessage > , }}}
mkitem!{mkenum!{#[doc = " Append a message for `[const] Trait` errors."] #[derive (Clone , Copy , PartialEq , Eq , Debug , Default)] pub enum AppendConstMessage { #[default] Default , Custom (Symbol , Span) , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (trait_selection_malformed_on_unimplemented_attr)] #[help] pub struct MalformedOnUnimplementedAttrLint { #[label] pub span : Span , }}}
mkitem!{mkimpl!{impl MalformedOnUnimplementedAttrLint { pub fn new (span : Span) -> Self { Self { span } } }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (trait_selection_missing_options_for_on_unimplemented_attr)] #[help] pub struct MissingOptionsForOnUnimplementedAttr ;}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (trait_selection_ignored_diagnostic_option)] pub struct IgnoredDiagnosticOption { pub option_name : & 'static str , #[label] pub span : Span , #[label (trait_selection_other_label)] pub prev_span : Span , }}}
mkitem!{mkimpl!{impl IgnoredDiagnosticOption { pub fn maybe_emit_warning < 'tcx > (tcx : TyCtxt < 'tcx > , item_def_id : DefId , new : Option < Span > , old : Option < Span > , option_name : & 'static str ,) { if let (Some (new_item) , Some (old_item)) = (new , old) && let Some (item_def_id) = item_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_ATTRIBUTES , tcx . local_def_id_to_hir_id (item_def_id) , new_item , IgnoredDiagnosticOption { span : new_item , prev_span : old_item , option_name } ,) ; } } }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (trait_selection_wrapped_parser_error)] pub struct WrappedParserError { pub description : String , pub label : String , }}}
mkitem!{mkimpl!{impl < 'tcx > OnUnimplementedDirective { fn parse (tcx : TyCtxt < 'tcx > , item_def_id : DefId , items : & [MetaItemInner] , span : Span , is_root : bool , is_diagnostic_namespace_variant : bool ,) -> Result < Option < Self > , ErrorGuaranteed > { let mut errored = None ; let mut item_iter = items . iter () ; let parse_value = | value_str , span | { OnUnimplementedFormatString :: try_parse (tcx , item_def_id , value_str , span , is_diagnostic_namespace_variant ,) . map (Some) } ; let condition = if is_root { None } else { let cond = item_iter . next () . ok_or_else (| | tcx . dcx () . emit_err (InvalidOnClause :: Empty { span })) ? ; let generics : Vec < Symbol > = tcx . generics_of (item_def_id) . own_params . iter () . filter_map (| param | { if matches ! (param . kind , GenericParamDefKind :: Lifetime) { None } else { Some (param . name) } }) . collect () ; match OnUnimplementedCondition :: parse (cond , & generics) { Ok (condition) => Some (condition) , Err (e) => return Err (tcx . dcx () . emit_err (e)) , } } ; let mut message = None ; let mut label = None ; let mut notes = Vec :: new () ; let mut parent_label = None ; let mut subcommands = vec ! [] ; let mut append_const_msg = None ; let get_value_and_span = | item : & _ , key | { if let MetaItemInner :: MetaItem (MetaItem { path , kind : MetaItemKind :: NameValue (MetaItemLit { span , kind : LitKind :: Str (s , _) , .. }) , .. }) = item && * path == key { Some ((* s , * span)) } else { None } } ; for item in item_iter { if let Some ((message_ , span)) = get_value_and_span (item , sym :: message) && message . is_none () { message = parse_value (message_ , span) ? . map (| l | (item . span () , l)) ; continue ; } else if let Some ((label_ , span)) = get_value_and_span (item , sym :: label) && label . is_none () { label = parse_value (label_ , span) ? . map (| l | (item . span () , l)) ; continue ; } else if let Some ((note_ , span)) = get_value_and_span (item , sym :: note) { if let Some (note) = parse_value (note_ , span) ? { notes . push (note) ; continue ; } } else if item . has_name (sym :: parent_label) && parent_label . is_none () && ! is_diagnostic_namespace_variant { if let Some (parent_label_) = item . value_str () { parent_label = parse_value (parent_label_ , item . span ()) ? ; continue ; } } else if item . has_name (sym :: on) && is_root && message . is_none () && label . is_none () && notes . is_empty () && ! is_diagnostic_namespace_variant { if let Some (items) = item . meta_item_list () { match Self :: parse (tcx , item_def_id , items , item . span () , false , is_diagnostic_namespace_variant ,) { Ok (Some (subcommand)) => subcommands . push (subcommand) , Ok (None) => bug ! ("This cannot happen for now as we only reach that if `is_diagnostic_namespace_variant` is false") , Err (reported) => errored = Some (reported) , } ; continue ; } } else if item . has_name (sym :: append_const_msg) && append_const_msg . is_none () && ! is_diagnostic_namespace_variant { if let Some (msg) = item . value_str () { append_const_msg = Some (AppendConstMessage :: Custom (msg , item . span ())) ; continue ; } else if item . is_word () { append_const_msg = Some (AppendConstMessage :: Default) ; continue ; } } if is_diagnostic_namespace_variant { if let Some (def_id) = item_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_ATTRIBUTES , tcx . local_def_id_to_hir_id (def_id) , vec ! [item . span ()] , MalformedOnUnimplementedAttrLint :: new (item . span ()) ,) ; } } else { tcx . dcx () . emit_err (NoValueInOnUnimplemented { span : item . span () }) ; } } if let Some (reported) = errored { if is_diagnostic_namespace_variant { Ok (None) } else { Err (reported) } } else { Ok (Some (OnUnimplementedDirective { condition , subcommands , message , label , notes , parent_label , append_const_msg , })) } } pub fn of_item (tcx : TyCtxt < 'tcx > , item_def_id : DefId) -> Result < Option < Self > , ErrorGuaranteed > { if ! tcx . is_trait (item_def_id) { return Ok (None) ; } if let Some (attr) = tcx . get_attr (item_def_id , sym :: rustc_on_unimplemented) { return Self :: parse_attribute (attr , false , tcx , item_def_id) ; } else { tcx . get_attrs_by_path (item_def_id , & [sym :: diagnostic , sym :: on_unimplemented]) . filter_map (| attr | Self :: parse_attribute (attr , true , tcx , item_def_id) . transpose ()) . try_fold (None , | aggr : Option < Self > , directive | { let directive = directive ? ; if let Some (aggr) = aggr { let mut subcommands = aggr . subcommands ; subcommands . extend (directive . subcommands) ; let mut notes = aggr . notes ; notes . extend (directive . notes) ; IgnoredDiagnosticOption :: maybe_emit_warning (tcx , item_def_id , directive . message . as_ref () . map (| f | f . 0) , aggr . message . as_ref () . map (| f | f . 0) , "message" ,) ; IgnoredDiagnosticOption :: maybe_emit_warning (tcx , item_def_id , directive . label . as_ref () . map (| f | f . 0) , aggr . label . as_ref () . map (| f | f . 0) , "label" ,) ; IgnoredDiagnosticOption :: maybe_emit_warning (tcx , item_def_id , directive . condition . as_ref () . map (| i | i . span ()) , aggr . condition . as_ref () . map (| i | i . span ()) , "condition" ,) ; IgnoredDiagnosticOption :: maybe_emit_warning (tcx , item_def_id , directive . parent_label . as_ref () . map (| f | f . span) , aggr . parent_label . as_ref () . map (| f | f . span) , "parent_label" ,) ; IgnoredDiagnosticOption :: maybe_emit_warning (tcx , item_def_id , directive . append_const_msg . as_ref () . and_then (| c | { if let AppendConstMessage :: Custom (_ , s) = c { Some (* s) } else { None } }) , aggr . append_const_msg . as_ref () . and_then (| c | { if let AppendConstMessage :: Custom (_ , s) = c { Some (* s) } else { None } }) , "append_const_msg" ,) ; Ok (Some (Self { condition : aggr . condition . or (directive . condition) , subcommands , message : aggr . message . or (directive . message) , label : aggr . label . or (directive . label) , notes , parent_label : aggr . parent_label . or (directive . parent_label) , append_const_msg : aggr . append_const_msg . or (directive . append_const_msg) , })) } else { Ok (Some (directive)) } }) } } fn parse_attribute (attr : & Attribute , is_diagnostic_namespace_variant : bool , tcx : TyCtxt < 'tcx > , item_def_id : DefId ,) -> Result < Option < Self > , ErrorGuaranteed > { let result = if let Some (items) = attr . meta_item_list () { Self :: parse (tcx , item_def_id , & items , attr . span () , true , is_diagnostic_namespace_variant ,) } else if let Some (value) = attr . value_str () { if ! is_diagnostic_namespace_variant { Ok (Some (OnUnimplementedDirective { condition : None , message : None , subcommands : vec ! [] , label : Some ((attr . span () , OnUnimplementedFormatString :: try_parse (tcx , item_def_id , value , attr . value_span () . unwrap_or (attr . span ()) , is_diagnostic_namespace_variant ,) ? ,)) , notes : Vec :: new () , parent_label : None , append_const_msg : None , })) } else { let item = attr . get_normal_item () ; let report_span = match & item . args { AttrArgs :: Empty => item . path . span , AttrArgs :: Delimited (args) => args . dspan . entire () , AttrArgs :: Eq { eq_span , expr } => eq_span . to (expr . span) , } ; if let Some (item_def_id) = item_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_ATTRIBUTES , tcx . local_def_id_to_hir_id (item_def_id) , report_span , MalformedOnUnimplementedAttrLint :: new (report_span) ,) ; } Ok (None) } } else if is_diagnostic_namespace_variant { match attr { Attribute :: Unparsed (p) if ! matches ! (p . args , AttrArgs :: Empty) => { if let Some (item_def_id) = item_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_ATTRIBUTES , tcx . local_def_id_to_hir_id (item_def_id) , attr . span () , MalformedOnUnimplementedAttrLint :: new (attr . span ()) ,) ; } } _ => { if let Some (item_def_id) = item_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_ATTRIBUTES , tcx . local_def_id_to_hir_id (item_def_id) , attr . span () , MissingOptionsForOnUnimplementedAttr ,) } } } ; Ok (None) } else { let reported = tcx . dcx () . delayed_bug ("of_item: neither meta_item_list nor value_str") ; return Err (reported) ; } ; debug ! ("of_item({:?}) = {:?}" , item_def_id , result) ; result } pub (crate) fn evaluate (& self , tcx : TyCtxt < 'tcx > , trait_ref : ty :: TraitRef < 'tcx > , condition_options : & ConditionOptions , args : & FormatArgs < 'tcx > ,) -> OnUnimplementedNote { let mut message = None ; let mut label = None ; let mut notes = Vec :: new () ; let mut parent_label = None ; let mut append_const_msg = None ; info ! ("evaluate({:?}, trait_ref={:?}, options={:?}, args ={:?})" , self , trait_ref , condition_options , args) ; for command in self . subcommands . iter () . chain (Some (self)) . rev () { debug ! (? command) ; if let Some (ref condition) = command . condition && ! condition . matches_predicate (condition_options) { debug ! ("evaluate: skipping {:?} due to condition" , command) ; continue ; } debug ! ("evaluate: {:?} succeeded" , command) ; if let Some (ref message_) = command . message { message = Some (message_ . clone ()) ; } if let Some (ref label_) = command . label { label = Some (label_ . clone ()) ; } notes . extend (command . notes . clone ()) ; if let Some (ref parent_label_) = command . parent_label { parent_label = Some (parent_label_ . clone ()) ; } append_const_msg = command . append_const_msg ; } OnUnimplementedNote { label : label . map (| l | l . 1 . format (tcx , trait_ref , args)) , message : message . map (| m | m . 1 . format (tcx , trait_ref , args)) , notes : notes . into_iter () . map (| n | n . format (tcx , trait_ref , args)) . collect () , parent_label : parent_label . map (| e_s | e_s . format (tcx , trait_ref , args)) , append_const_msg , } } }}}
mkitem!{mkimpl!{impl < 'tcx > OnUnimplementedFormatString { fn try_parse (tcx : TyCtxt < 'tcx > , item_def_id : DefId , from : Symbol , span : Span , is_diagnostic_namespace_variant : bool ,) -> Result < Self , ErrorGuaranteed > { let result = OnUnimplementedFormatString { symbol : from , span , is_diagnostic_namespace_variant } ; result . verify (tcx , item_def_id) ? ; Ok (result) } fn verify (& self , tcx : TyCtxt < 'tcx > , trait_def_id : DefId) -> Result < () , ErrorGuaranteed > { if ! tcx . is_trait (trait_def_id) { return Ok (()) ; } ; let ctx = if self . is_diagnostic_namespace_variant { Ctx :: DiagnosticOnUnimplemented { tcx , trait_def_id } } else { Ctx :: RustcOnUnimplemented { tcx , trait_def_id } } ; let mut result = Ok (()) ; let snippet = tcx . sess . source_map () . span_to_snippet (self . span) . ok () ; match FormatString :: parse (self . symbol , snippet , self . span , & ctx) { Ok (FormatString { warnings , .. }) => { if self . is_diagnostic_namespace_variant { for w in warnings { w . emit_warning (tcx , trait_def_id) } } else { for w in warnings { match w { FormatWarning :: UnknownParam { argument_name , span } => { let reported = struct_span_code_err ! (tcx . dcx () , span , E0230 , "cannot find parameter {} on this trait" , argument_name ,) . emit () ; result = Err (reported) ; } FormatWarning :: PositionalArgument { span , .. } => { let reported = struct_span_code_err ! (tcx . dcx () , span , E0231 , "positional format arguments are not allowed here") . emit () ; result = Err (reported) ; } FormatWarning :: InvalidSpecifier { .. } | FormatWarning :: FutureIncompat { .. } => { } } } } } Err (e) => { if self . is_diagnostic_namespace_variant { if let Some (trait_def_id) = trait_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_FORMAT_LITERALS , tcx . local_def_id_to_hir_id (trait_def_id) , self . span , WrappedParserError { description : e . description , label : e . label } ,) ; } } else { let reported = struct_span_code_err ! (tcx . dcx () , self . span , E0231 , "{}" , e . description ,) . emit () ; result = Err (reported) ; } } } result } pub fn format (& self , tcx : TyCtxt < 'tcx > , trait_ref : ty :: TraitRef < 'tcx > , args : & FormatArgs < 'tcx > ,) -> String { let trait_def_id = trait_ref . def_id ; let ctx = if self . is_diagnostic_namespace_variant { Ctx :: DiagnosticOnUnimplemented { tcx , trait_def_id } } else { Ctx :: RustcOnUnimplemented { tcx , trait_def_id } } ; if let Ok (s) = FormatString :: parse (self . symbol , None , self . span , & ctx) { s . format (args) } else { self . symbol . as_str () . into () } } }}}