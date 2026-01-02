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
mkuse!{use std :: ops :: Deref ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_hir :: LangItem ;}
mkuse!{use rustc_hir :: def_id :: { CRATE_DEF_ID , DefId } ;}
mkuse!{use rustc_infer :: infer :: canonical :: query_response :: make_query_region_constraints ;}
mkuse!{use rustc_infer :: infer :: canonical :: { Canonical , CanonicalExt as _ , CanonicalQueryInput , CanonicalVarKind , CanonicalVarValues , } ;}
mkuse!{use rustc_infer :: infer :: { InferCtxt , RegionVariableOrigin , SubregionOrigin , TyCtxtInferExt } ;}
mkuse!{use rustc_infer :: traits :: solve :: Goal ;}
mkuse!{use rustc_middle :: traits :: query :: NoSolution ;}
mkuse!{use rustc_middle :: traits :: solve :: Certainty ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeFlags , TypeFoldable , TypeVisitableExt as _ , TypingMode , } ;}
mkuse!{use rustc_span :: { DUMMY_SP , ErrorGuaranteed , Span } ;}
mkuse!{use crate :: traits :: { EvaluateConstErr , ObligationCause , sizedness_fast_path , specialization_graph } ;}
mkitem!{mkstruct!{#[repr (transparent)] pub struct SolverDelegate < 'tcx > (InferCtxt < 'tcx >) ;}}
mkitem!{mkimpl!{impl < 'a , 'tcx > From < & 'a InferCtxt < 'tcx > > for & 'a SolverDelegate < 'tcx > { fn from (infcx : & 'a InferCtxt < 'tcx >) -> Self { unsafe { std :: mem :: transmute (infcx) } } }}}
mkitem!{mkimpl!{impl < 'tcx > Deref for SolverDelegate < 'tcx > { type Target = InferCtxt < 'tcx > ; fn deref (& self) -> & Self :: Target { & self . 0 } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_next_trait_solver :: delegate :: SolverDelegate for SolverDelegate < 'tcx > { type Infcx = InferCtxt < 'tcx > ; type Interner = TyCtxt < 'tcx > ; fn cx (& self) -> TyCtxt < 'tcx > { self . 0 . tcx } fn build_with_canonical < V > (interner : TyCtxt < 'tcx > , canonical : & CanonicalQueryInput < 'tcx , V > ,) -> (Self , V , CanonicalVarValues < 'tcx >) where V : TypeFoldable < TyCtxt < 'tcx > > , { let (infcx , value , vars) = interner . infer_ctxt () . with_next_trait_solver (true) . build_with_canonical (DUMMY_SP , canonical) ; (SolverDelegate (infcx) , value , vars) } fn compute_goal_fast_path (& self , goal : Goal < 'tcx , ty :: Predicate < 'tcx > > , span : Span ,) -> Option < Certainty > { if let Some (trait_pred) = goal . predicate . as_trait_clause () { if self . shallow_resolve (trait_pred . self_ty () . skip_binder ()) . is_ty_var () && self . inner . borrow_mut () . opaque_types () . is_empty () { return Some (Certainty :: AMBIGUOUS) ; } if trait_pred . polarity () == ty :: PredicatePolarity :: Positive { match self . 0 . tcx . as_lang_item (trait_pred . def_id ()) { Some (LangItem :: Sized) | Some (LangItem :: MetaSized) => { let predicate = self . resolve_vars_if_possible (goal . predicate) ; if sizedness_fast_path (self . tcx , predicate , goal . param_env) { return Some (Certainty :: Yes) ; } } Some (LangItem :: Copy | LangItem :: Clone) => { let self_ty = self . resolve_vars_if_possible (trait_pred . self_ty () . skip_binder ()) ; if ! self_ty . has_type_flags (TypeFlags :: HAS_FREE_REGIONS | TypeFlags :: HAS_INFER) && self_ty . is_trivially_pure_clone_copy () { return Some (Certainty :: Yes) ; } } _ => { } } } } let pred = goal . predicate . kind () ; match pred . no_bound_vars () ? { ty :: PredicateKind :: DynCompatible (def_id) if self . 0 . tcx . is_dyn_compatible (def_id) => { Some (Certainty :: Yes) } ty :: PredicateKind :: Clause (ty :: ClauseKind :: RegionOutlives (outlives)) => { self . 0 . sub_regions (SubregionOrigin :: RelateRegionParamBound (span , None) , outlives . 1 , outlives . 0 ,) ; Some (Certainty :: Yes) } ty :: PredicateKind :: Clause (ty :: ClauseKind :: TypeOutlives (outlives)) => { self . 0 . register_type_outlives_constraint (outlives . 0 , outlives . 1 , & ObligationCause :: dummy_with_span (span) ,) ; Some (Certainty :: Yes) } ty :: PredicateKind :: Subtype (ty :: SubtypePredicate { a , b , .. }) | ty :: PredicateKind :: Coerce (ty :: CoercePredicate { a , b }) => { match (self . shallow_resolve (a) . kind () , self . shallow_resolve (b) . kind ()) { (& ty :: Infer (ty :: TyVar (a_vid)) , & ty :: Infer (ty :: TyVar (b_vid))) => { self . sub_unify_ty_vids_raw (a_vid , b_vid) ; Some (Certainty :: AMBIGUOUS) } _ => None , } } ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstArgHasType (ct , _)) => { if self . shallow_resolve_const (ct) . is_ct_infer () { Some (Certainty :: AMBIGUOUS) } else { None } } ty :: PredicateKind :: Clause (ty :: ClauseKind :: WellFormed (arg)) => { let arg = self . shallow_resolve_term (arg) ; if arg . is_trivially_wf (self . tcx) { Some (Certainty :: Yes) } else if arg . is_infer () { Some (Certainty :: AMBIGUOUS) } else { None } } _ => None , } } fn fresh_var_for_kind_with_span (& self , arg : ty :: GenericArg < 'tcx > , span : Span ,) -> ty :: GenericArg < 'tcx > { match arg . kind () { ty :: GenericArgKind :: Lifetime (_) => { self . next_region_var (RegionVariableOrigin :: Misc (span)) . into () } ty :: GenericArgKind :: Type (_) => self . next_ty_var (span) . into () , ty :: GenericArgKind :: Const (_) => self . next_const_var (span) . into () , } } fn leak_check (& self , max_input_universe : ty :: UniverseIndex) -> Result < () , NoSolution > { self . 0 . leak_check (max_input_universe , None) . map_err (| _ | NoSolution) } fn evaluate_const (& self , param_env : ty :: ParamEnv < 'tcx > , uv : ty :: UnevaluatedConst < 'tcx > ,) -> Option < ty :: Const < 'tcx > > { let ct = ty :: Const :: new_unevaluated (self . tcx , uv) ; match crate :: traits :: try_evaluate_const (& self . 0 , ct , param_env) { Ok (ct) => Some (ct) , Err (EvaluateConstErr :: EvaluationFailure (e)) => Some (ty :: Const :: new_error (self . tcx , e)) , Err (EvaluateConstErr :: InvalidConstParamTy (_) | EvaluateConstErr :: HasGenericsOrInfers ,) => None , } } fn well_formed_goals (& self , param_env : ty :: ParamEnv < 'tcx > , term : ty :: Term < 'tcx > ,) -> Option < Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > > > { crate :: traits :: wf :: unnormalized_obligations (& self . 0 , param_env , term , DUMMY_SP , CRATE_DEF_ID ,) . map (| obligations | obligations . into_iter () . map (| obligation | obligation . as_goal ()) . collect ()) } fn make_deduplicated_outlives_constraints (& self) -> Vec < ty :: ArgOutlivesPredicate < 'tcx > > { let region_obligations = self . 0 . inner . borrow () . region_obligations () . to_owned () ; let region_assumptions = self . 0 . inner . borrow () . region_assumptions () . to_owned () ; let region_constraints = self . 0 . with_region_constraints (| region_constraints | { make_query_region_constraints (region_obligations , region_constraints , region_assumptions ,) }) ; let mut seen = FxHashSet :: default () ; region_constraints . outlives . into_iter () . filter (| & (outlives , _) | seen . insert (outlives)) . map (| (outlives , _) | outlives) . collect () } fn instantiate_canonical < V > (& self , canonical : Canonical < 'tcx , V > , values : CanonicalVarValues < 'tcx > ,) -> V where V : TypeFoldable < TyCtxt < 'tcx > > , { canonical . instantiate (self . tcx , & values) } fn instantiate_canonical_var (& self , kind : CanonicalVarKind < 'tcx > , span : Span , var_values : & [ty :: GenericArg < 'tcx >] , universe_map : impl Fn (ty :: UniverseIndex) -> ty :: UniverseIndex ,) -> ty :: GenericArg < 'tcx > { self . 0 . instantiate_canonical_var (span , kind , var_values , universe_map) } fn add_item_bounds_for_hidden_type (& self , def_id : DefId , args : ty :: GenericArgsRef < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , hidden_ty : Ty < 'tcx > , goals : & mut Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > > ,) { self . 0 . add_item_bounds_for_hidden_type (def_id , args , param_env , hidden_ty , goals) ; } fn fetch_eligible_assoc_item (& self , goal_trait_ref : ty :: TraitRef < 'tcx > , trait_assoc_def_id : DefId , impl_def_id : DefId ,) -> Result < Option < DefId > , ErrorGuaranteed > { let node_item = specialization_graph :: assoc_def (self . tcx , impl_def_id , trait_assoc_def_id) ? ; let eligible = if node_item . is_final () { true } else { match self . typing_mode () { TypingMode :: Coherence | TypingMode :: Analysis { .. } | TypingMode :: Borrowck { .. } | TypingMode :: PostBorrowckAnalysis { .. } => false , TypingMode :: PostAnalysis => { let poly_trait_ref = self . resolve_vars_if_possible (goal_trait_ref) ; ! poly_trait_ref . still_further_specializable () } } } ; if eligible { Ok (Some (node_item . item . def_id)) } else { Ok (None) } } fn is_transmutable (& self , dst : Ty < 'tcx > , src : Ty < 'tcx > , assume : ty :: Const < 'tcx > ,) -> Result < Certainty , NoSolution > { let (dst , src) = self . tcx . erase_and_anonymize_regions ((dst , src)) ; let Some (assume) = rustc_transmute :: Assume :: from_const (self . tcx , assume) else { return Err (NoSolution) ; } ; match rustc_transmute :: TransmuteTypeEnv :: new (self . 0 . tcx) . is_transmutable (rustc_transmute :: Types { src , dst } , assume) { rustc_transmute :: Answer :: Yes => Ok (Certainty :: Yes) , rustc_transmute :: Answer :: No (_) | rustc_transmute :: Answer :: If (_) => Err (NoSolution) , } } }}}