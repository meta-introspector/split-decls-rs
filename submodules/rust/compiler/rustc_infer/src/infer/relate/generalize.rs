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
mkuse!{use std :: mem ;}
mkuse!{use rustc_data_structures :: sso :: SsoHashMap ;}
mkuse!{use rustc_data_structures :: stack :: ensure_sufficient_stack ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: error :: TypeError ;}
mkuse!{use rustc_middle :: ty :: { self , AliasRelationDirection , InferConst , Term , Ty , TyCtxt , TypeSuperVisitable , TypeVisitable , TypeVisitableExt , TypeVisitor , TypingMode , } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use tracing :: { debug , instrument , warn } ;}
mkuse!{use super :: { PredicateEmittingRelation , Relate , RelateResult , StructurallyRelateAliases , TypeRelation , } ;}
mkuse!{use crate :: infer :: type_variable :: TypeVariableValue ;}
mkuse!{use crate :: infer :: unify_key :: ConstVariableValue ;}
mkuse!{use crate :: infer :: { InferCtxt , RegionVariableOrigin , relate } ;}
mkitem!{mkenum!{#[derive (Copy , Clone , Eq , PartialEq , Debug)] enum TermVid { Ty (ty :: TyVid) , Const (ty :: ConstVid) , }}}
mkitem!{mkimpl!{impl From < ty :: TyVid > for TermVid { fn from (value : ty :: TyVid) -> Self { TermVid :: Ty (value) } }}}
mkitem!{mkimpl!{impl From < ty :: ConstVid > for TermVid { fn from (value : ty :: ConstVid) -> Self { TermVid :: Const (value) } }}}
mkitem!{mkimpl!{impl < 'tcx > InferCtxt < 'tcx > { #[doc = " The idea is that we should ensure that the type variable `target_vid`"] #[doc = " is equal to, a subtype of, or a supertype of `source_ty`."] #[doc = ""] #[doc = " For this, we will instantiate `target_vid` with a *generalized* version"] #[doc = " of `source_ty`. Generalization introduces other inference variables wherever"] #[doc = " subtyping could occur. This also does the occurs checks, detecting whether"] #[doc = " instantiating `target_vid` would result in a cyclic type. We eagerly error"] #[doc = " in this case."] #[doc = ""] #[doc = " This is *not* expected to be used anywhere except for an implementation of"] #[doc = " `TypeRelation`. Do not use this, and instead please use `At::eq`, for all"] #[doc = " other usecases (i.e. setting the value of a type var)."] #[instrument (level = "debug" , skip (self , relation))] pub fn instantiate_ty_var < R : PredicateEmittingRelation < InferCtxt < 'tcx > > > (& self , relation : & mut R , target_is_expected : bool , target_vid : ty :: TyVid , instantiation_variance : ty :: Variance , source_ty : Ty < 'tcx > ,) -> RelateResult < 'tcx , () > { debug_assert ! (self . inner . borrow_mut () . type_variables () . probe (target_vid) . is_unknown ()) ; let Generalization { value_may_be_infer : generalized_ty , has_unconstrained_ty_var } = self . generalize (relation . span () , relation . structurally_relate_aliases () , target_vid , instantiation_variance , source_ty ,) ? ; if let & ty :: Infer (ty :: TyVar (generalized_vid)) = generalized_ty . kind () { self . inner . borrow_mut () . type_variables () . equate (target_vid , generalized_vid) ; } else { self . inner . borrow_mut () . type_variables () . instantiate (target_vid , generalized_ty) ; } if has_unconstrained_ty_var { relation . register_predicates ([ty :: ClauseKind :: WellFormed (generalized_ty . into ())]) ; } if generalized_ty . is_ty_var () { if self . next_trait_solver () { let (lhs , rhs , direction) = match instantiation_variance { ty :: Invariant => { (generalized_ty . into () , source_ty . into () , AliasRelationDirection :: Equate) } ty :: Covariant => { (generalized_ty . into () , source_ty . into () , AliasRelationDirection :: Subtype) } ty :: Contravariant => { (source_ty . into () , generalized_ty . into () , AliasRelationDirection :: Subtype) } ty :: Bivariant => unreachable ! ("bivariant generalization") , } ; relation . register_predicates ([ty :: PredicateKind :: AliasRelate (lhs , rhs , direction)]) ; } else { match source_ty . kind () { & ty :: Alias (ty :: Projection , data) => { relation . register_predicates ([ty :: ProjectionPredicate { projection_term : data . into () , term : generalized_ty . into () , }]) ; } ty :: Alias (ty :: Inherent | ty :: Free | ty :: Opaque , _) => { return Err (TypeError :: CyclicTy (source_ty)) ; } _ => bug ! ("generalized `{source_ty:?} to infer, not an alias") , } } } else { if target_is_expected { relation . relate (generalized_ty , source_ty) ? ; } else { debug ! ("flip relation") ; relation . relate (source_ty , generalized_ty) ? ; } } Ok (()) } #[doc = " Instantiates the const variable `target_vid` with the given constant."] #[doc = ""] #[doc = " This also tests if the given const `ct` contains an inference variable which was previously"] #[doc = " unioned with `target_vid`. If this is the case, inferring `target_vid` to `ct`"] #[doc = " would result in an infinite type as we continuously replace an inference variable"] #[doc = " in `ct` with `ct` itself."] #[doc = ""] #[doc = " This is especially important as unevaluated consts use their parents generics."] #[doc = " They therefore often contain unused args, making these errors far more likely."] #[doc = ""] #[doc = " A good example of this is the following:"] #[doc = ""] #[doc = " ```compile_fail,E0308"] #[doc = " #![feature(generic_const_exprs)]"] #[doc = ""] #[doc = " fn bind<const N: usize>(value: [u8; N]) -> [u8; 3 + 4] {"] #[doc = "     todo!()"] #[doc = " }"] #[doc = ""] #[doc = " fn main() {"] #[doc = "     let mut arr = Default::default();"] #[doc = "     arr = bind(arr);"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " Here `3 + 4` ends up as `ConstKind::Unevaluated` which uses the generics"] #[doc = " of `fn bind` (meaning that its args contain `N`)."] #[doc = ""] #[doc = " `bind(arr)` now infers that the type of `arr` must be `[u8; N]`."] #[doc = " The assignment `arr = bind(arr)` now tries to equate `N` with `3 + 4`."] #[doc = ""] #[doc = " As `3 + 4` contains `N` in its args, this must not succeed."] #[doc = ""] #[doc = " See `tests/ui/const-generics/occurs-check/` for more examples where this is relevant."] #[instrument (level = "debug" , skip (self , relation))] pub (crate) fn instantiate_const_var < R : PredicateEmittingRelation < InferCtxt < 'tcx > > > (& self , relation : & mut R , target_is_expected : bool , target_vid : ty :: ConstVid , source_ct : ty :: Const < 'tcx > ,) -> RelateResult < 'tcx , () > { let Generalization { value_may_be_infer : generalized_ct , has_unconstrained_ty_var } = self . generalize (relation . span () , relation . structurally_relate_aliases () , target_vid , ty :: Invariant , source_ct ,) ? ; debug_assert ! (! generalized_ct . is_ct_infer ()) ; if has_unconstrained_ty_var { bug ! ("unconstrained ty var when generalizing `{source_ct:?}`") ; } self . inner . borrow_mut () . const_unification_table () . union_value (target_vid , ConstVariableValue :: Known { value : generalized_ct }) ; if target_is_expected { relation . relate_with_variance (ty :: Invariant , ty :: VarianceDiagInfo :: default () , generalized_ct , source_ct ,) ? ; } else { relation . relate_with_variance (ty :: Invariant , ty :: VarianceDiagInfo :: default () , source_ct , generalized_ct ,) ? ; } Ok (()) } #[doc = " Attempts to generalize `source_term` for the type variable `target_vid`."] #[doc = " This checks for cycles -- that is, whether `source_term` references `target_vid`."] fn generalize < T : Into < Term < 'tcx > > + Relate < TyCtxt < 'tcx > > > (& self , span : Span , structurally_relate_aliases : StructurallyRelateAliases , target_vid : impl Into < TermVid > , ambient_variance : ty :: Variance , source_term : T ,) -> RelateResult < 'tcx , Generalization < T > > { assert ! (! source_term . has_escaping_bound_vars ()) ; let (for_universe , root_vid) = match target_vid . into () { TermVid :: Ty (ty_vid) => { (self . probe_ty_var (ty_vid) . unwrap_err () , TermVid :: Ty (self . root_var (ty_vid))) } TermVid :: Const (ct_vid) => (self . probe_const_var (ct_vid) . unwrap_err () , TermVid :: Const (self . inner . borrow_mut () . const_unification_table () . find (ct_vid) . vid) ,) , } ; let mut generalizer = Generalizer { infcx : self , span , structurally_relate_aliases , root_vid , for_universe , root_term : source_term . into () , ambient_variance , in_alias : false , cache : Default :: default () , has_unconstrained_ty_var : false , } ; let value_may_be_infer = generalizer . relate (source_term , source_term) ? ; let has_unconstrained_ty_var = generalizer . has_unconstrained_ty_var ; Ok (Generalization { value_may_be_infer , has_unconstrained_ty_var }) } }}}
mkitem!{mkstruct!{#[doc = " Finds the max universe present"] struct MaxUniverse { max_universe : ty :: UniverseIndex , }}}
mkitem!{mkimpl!{impl MaxUniverse { fn new () -> Self { MaxUniverse { max_universe : ty :: UniverseIndex :: ROOT } } fn max_universe (self) -> ty :: UniverseIndex { self . max_universe } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for MaxUniverse { fn visit_ty (& mut self , t : Ty < 'tcx >) { if let ty :: Placeholder (placeholder) = t . kind () { self . max_universe = self . max_universe . max (placeholder . universe) ; } t . super_visit_with (self) } fn visit_const (& mut self , c : ty :: Const < 'tcx >) { if let ty :: ConstKind :: Placeholder (placeholder) = c . kind () { self . max_universe = self . max_universe . max (placeholder . universe) ; } c . super_visit_with (self) } fn visit_region (& mut self , r : ty :: Region < 'tcx >) { if let ty :: RePlaceholder (placeholder) = r . kind () { self . max_universe = self . max_universe . max (placeholder . universe) ; } } }}}
mkitem!{mkstruct!{#[doc = " The \"generalizer\" is used when handling inference variables."] #[doc = ""] #[doc = " The basic strategy for handling a constraint like `?A <: B` is to"] #[doc = " apply a \"generalization strategy\" to the term `B` -- this replaces"] #[doc = " all the lifetimes in the term `B` with fresh inference variables."] #[doc = " (You can read more about the strategy in this [blog post].)"] #[doc = ""] #[doc = " As an example, if we had `?A <: &'x u32`, we would generalize `&'x"] #[doc = " u32` to `&'0 u32` where `'0` is a fresh variable. This becomes the"] #[doc = " value of `A`. Finally, we relate `&'0 u32 <: &'x u32`, which"] #[doc = " establishes `'0: 'x` as a constraint."] #[doc = ""] #[doc = " [blog post]: https://is.gd/0hKvIr"] struct Generalizer < 'me , 'tcx > { infcx : & 'me InferCtxt < 'tcx > , span : Span , #[doc = " Whether aliases should be related structurally. If not, we have to"] #[doc = " be careful when generalizing aliases."] structurally_relate_aliases : StructurallyRelateAliases , #[doc = " The vid of the type variable that is in the process of being"] #[doc = " instantiated. If we find this within the value we are folding,"] #[doc = " that means we would have created a cyclic value."] root_vid : TermVid , #[doc = " The universe of the type variable that is in the process of being"] #[doc = " instantiated. If we find anything that this universe cannot name,"] #[doc = " we reject the relation."] for_universe : ty :: UniverseIndex , #[doc = " The root term (const or type) we're generalizing. Used for cycle errors."] root_term : Term < 'tcx > , #[doc = " After we generalize this type, we are going to relate it to"] #[doc = " some other type. What will be the variance at this point?"] ambient_variance : ty :: Variance , #[doc = " This is set once we're generalizing the arguments of an alias."] #[doc = ""] #[doc = " This is necessary to correctly handle"] #[doc = " `<T as Bar<<?0 as Foo>::Assoc>::Assoc == ?0`. This equality can"] #[doc = " hold by either normalizing the outer or the inner associated type."] in_alias : bool , cache : SsoHashMap < (Ty < 'tcx > , ty :: Variance , bool) , Ty < 'tcx > > , #[doc = " See the field `has_unconstrained_ty_var` in `Generalization`."] has_unconstrained_ty_var : bool , }}}
mkitem!{mkimpl!{impl < 'tcx > Generalizer < '_ , 'tcx > { #[doc = " Create an error that corresponds to the term kind in `root_term`"] fn cyclic_term_error (& self) -> TypeError < 'tcx > { match self . root_term . kind () { ty :: TermKind :: Ty (ty) => TypeError :: CyclicTy (ty) , ty :: TermKind :: Const (ct) => TypeError :: CyclicConst (ct) , } } #[doc = " Create a new type variable in the universe of the target when"] #[doc = " generalizing an alias. This has to set `has_unconstrained_ty_var`"] #[doc = " if we're currently in a bivariant context."] fn next_ty_var_for_alias (& mut self) -> Ty < 'tcx > { self . has_unconstrained_ty_var |= self . ambient_variance == ty :: Bivariant ; self . infcx . next_ty_var_in_universe (self . span , self . for_universe) } #[doc = " An occurs check failure inside of an alias does not mean"] #[doc = " that the types definitely don't unify. We may be able"] #[doc = " to normalize the alias after all."] #[doc = ""] #[doc = " We handle this by lazily equating the alias and generalizing"] #[doc = " it to an inference variable. In the new solver, we always"] #[doc = " generalize to an infer var unless the alias contains escaping"] #[doc = " bound variables."] #[doc = ""] #[doc = " Correctly handling aliases with escaping bound variables is"] #[doc = " difficult and currently incomplete in two opposite ways:"] #[doc = " - if we get an occurs check failure in the alias, replace it with a new infer var."] #[doc = "   This causes us to later emit an alias-relate goal and is incomplete in case the"] #[doc = "   alias normalizes to type containing one of the bound variables."] #[doc = " - if the alias contains an inference variable not nameable by `for_universe`, we"] #[doc = "   continue generalizing the alias. This ends up pulling down the universe of the"] #[doc = "   inference variable and is incomplete in case the alias would normalize to a type"] #[doc = "   which does not mention that inference variable."] fn generalize_alias_ty (& mut self , alias : ty :: AliasTy < 'tcx > ,) -> Result < Ty < 'tcx > , TypeError < 'tcx > > { if self . infcx . next_trait_solver () && ! alias . has_escaping_bound_vars () && ! self . in_alias { return Ok (self . next_ty_var_for_alias ()) ; } let is_nested_alias = mem :: replace (& mut self . in_alias , true) ; let result = match self . relate (alias , alias) { Ok (alias) => Ok (alias . to_ty (self . cx ())) , Err (e) => { if is_nested_alias { return Err (e) ; } else { let mut visitor = MaxUniverse :: new () ; alias . visit_with (& mut visitor) ; let infer_replacement_is_complete = self . for_universe . can_name (visitor . max_universe ()) && ! alias . has_escaping_bound_vars () ; if ! infer_replacement_is_complete { warn ! ("may incompletely handle alias type: {alias:?}") ; } debug ! ("generalization failure in alias") ; Ok (self . next_ty_var_for_alias ()) } } } ; self . in_alias = is_nested_alias ; result } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeRelation < TyCtxt < 'tcx > > for Generalizer < '_ , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . infcx . tcx } fn relate_item_args (& mut self , item_def_id : DefId , a_arg : ty :: GenericArgsRef < 'tcx > , b_arg : ty :: GenericArgsRef < 'tcx > ,) -> RelateResult < 'tcx , ty :: GenericArgsRef < 'tcx > > { if self . ambient_variance == ty :: Invariant { relate :: relate_args_invariantly (self , a_arg , b_arg) } else { let tcx = self . cx () ; let opt_variances = tcx . variances_of (item_def_id) ; relate :: relate_args_with_variances (self , item_def_id , opt_variances , a_arg , b_arg , false ,) } } #[instrument (level = "debug" , skip (self , variance , b) , ret)] fn relate_with_variance < T : Relate < TyCtxt < 'tcx > > > (& mut self , variance : ty :: Variance , _info : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , a : T , b : T ,) -> RelateResult < 'tcx , T > { let old_ambient_variance = self . ambient_variance ; self . ambient_variance = self . ambient_variance . xform (variance) ; debug ! (? self . ambient_variance , "new ambient variance") ; let r = ensure_sufficient_stack (| | self . relate (a , b)) ; self . ambient_variance = old_ambient_variance ; r } #[instrument (level = "debug" , skip (self , t2) , ret)] fn tys (& mut self , t : Ty < 'tcx > , t2 : Ty < 'tcx >) -> RelateResult < 'tcx , Ty < 'tcx > > { assert_eq ! (t , t2) ; if let Some (& result) = self . cache . get (& (t , self . ambient_variance , self . in_alias)) { return Ok (result) ; } let g = match * t . kind () { ty :: Infer (ty :: FreshTy (_) | ty :: FreshIntTy (_) | ty :: FreshFloatTy (_)) => { bug ! ("unexpected infer type: {t}") } ty :: Infer (ty :: TyVar (vid)) => { let mut inner = self . infcx . inner . borrow_mut () ; let vid = inner . type_variables () . root_var (vid) ; if TermVid :: Ty (vid) == self . root_vid { Err (self . cyclic_term_error ()) } else { let probe = inner . type_variables () . probe (vid) ; match probe { TypeVariableValue :: Known { value : u } => { drop (inner) ; self . relate (u , u) } TypeVariableValue :: Unknown { universe } => { match self . ambient_variance { ty :: Invariant => { if self . for_universe . can_name (universe) { return Ok (t) ; } } ty :: Bivariant => self . has_unconstrained_ty_var = true , ty :: Covariant | ty :: Contravariant => () , } let origin = inner . type_variables () . var_origin (vid) ; let new_var_id = inner . type_variables () . new_var (self . for_universe , origin) ; inner . type_variables () . sub_unify (vid , new_var_id) ; if self . infcx . next_trait_solver () && ! matches ! (self . infcx . typing_mode () , TypingMode :: Coherence) && self . in_alias { inner . type_variables () . equate (vid , new_var_id) ; } debug ! ("replacing original vid={:?} with new={:?}" , vid , new_var_id) ; Ok (Ty :: new_var (self . cx () , new_var_id)) } } } } ty :: Infer (ty :: IntVar (_) | ty :: FloatVar (_)) => { Ok (t) } ty :: Placeholder (placeholder) => { if self . for_universe . can_name (placeholder . universe) { Ok (t) } else { debug ! ("root universe {:?} cannot name placeholder in universe {:?}" , self . for_universe , placeholder . universe) ; Err (TypeError :: Mismatch) } } ty :: Alias (_ , data) => match self . structurally_relate_aliases { StructurallyRelateAliases :: No => self . generalize_alias_ty (data) , StructurallyRelateAliases :: Yes => relate :: structurally_relate_tys (self , t , t) , } , _ => relate :: structurally_relate_tys (self , t , t) , } ? ; self . cache . insert ((t , self . ambient_variance , self . in_alias) , g) ; Ok (g) } #[instrument (level = "debug" , skip (self , r2) , ret)] fn regions (& mut self , r : ty :: Region < 'tcx > , r2 : ty :: Region < 'tcx > ,) -> RelateResult < 'tcx , ty :: Region < 'tcx > > { assert_eq ! (r , r2) ; match r . kind () { ty :: ReBound (..) | ty :: ReErased => { return Ok (r) ; } ty :: ReError (_) => { return Ok (r) ; } ty :: RePlaceholder (..) | ty :: ReVar (..) | ty :: ReStatic | ty :: ReEarlyParam (..) | ty :: ReLateParam (..) => { } } if let ty :: Invariant = self . ambient_variance { let r_universe = self . infcx . universe_of_region (r) ; if self . for_universe . can_name (r_universe) { return Ok (r) ; } } Ok (self . infcx . next_region_var_in_universe (RegionVariableOrigin :: Misc (self . span) , self . for_universe)) } #[instrument (level = "debug" , skip (self , c2) , ret)] fn consts (& mut self , c : ty :: Const < 'tcx > , c2 : ty :: Const < 'tcx > ,) -> RelateResult < 'tcx , ty :: Const < 'tcx > > { assert_eq ! (c , c2) ; match c . kind () { ty :: ConstKind :: Infer (InferConst :: Var (vid)) => { if TermVid :: Const (self . infcx . inner . borrow_mut () . const_unification_table () . find (vid) . vid ,) == self . root_vid { return Err (self . cyclic_term_error ()) ; } let mut inner = self . infcx . inner . borrow_mut () ; let variable_table = & mut inner . const_unification_table () ; match variable_table . probe_value (vid) { ConstVariableValue :: Known { value : u } => { drop (inner) ; self . relate (u , u) } ConstVariableValue :: Unknown { origin , universe } => { if self . for_universe . can_name (universe) { Ok (c) } else { let new_var_id = variable_table . new_key (ConstVariableValue :: Unknown { origin , universe : self . for_universe , }) . vid ; if self . infcx . next_trait_solver () && ! matches ! (self . infcx . typing_mode () , TypingMode :: Coherence) && self . in_alias { variable_table . union (vid , new_var_id) ; } Ok (ty :: Const :: new_var (self . cx () , new_var_id)) } } } } ty :: ConstKind :: Unevaluated (ty :: UnevaluatedConst { def , args }) => { let args = self . relate_with_variance (ty :: Invariant , ty :: VarianceDiagInfo :: default () , args , args ,) ? ; Ok (ty :: Const :: new_unevaluated (self . cx () , ty :: UnevaluatedConst { def , args })) } ty :: ConstKind :: Placeholder (placeholder) => { if self . for_universe . can_name (placeholder . universe) { Ok (c) } else { debug ! ("root universe {:?} cannot name placeholder in universe {:?}" , self . for_universe , placeholder . universe) ; Err (TypeError :: Mismatch) } } _ => relate :: structurally_relate_consts (self , c , c) , } } #[instrument (level = "debug" , skip (self) , ret)] fn binders < T > (& mut self , a : ty :: Binder < 'tcx , T > , _ : ty :: Binder < 'tcx , T > ,) -> RelateResult < 'tcx , ty :: Binder < 'tcx , T > > where T : Relate < TyCtxt < 'tcx > > , { let result = self . relate (a . skip_binder () , a . skip_binder ()) ? ; Ok (a . rebind (result)) } }}}
mkitem!{mkstruct!{#[doc = " Result from a generalization operation. This includes"] #[doc = " not only the generalized type, but also a bool flag"] #[doc = " indicating whether further WF checks are needed."] #[derive (Debug)] struct Generalization < T > { #[doc = " When generalizing `<?0 as Trait>::Assoc` or"] #[doc = " `<T as Bar<<?0 as Foo>::Assoc>>::Assoc`"] #[doc = " for `?0` generalization returns an inference"] #[doc = " variable."] #[doc = ""] #[doc = " This has to be handled wotj care as it can"] #[doc = " otherwise very easily result in infinite"] #[doc = " recursion."] pub value_may_be_infer : T , #[doc = " In general, we do not check whether all types which occur during"] #[doc = " type checking are well-formed. We only check wf of user-provided types"] #[doc = " and when actually using a type, e.g. for method calls."] #[doc = ""] #[doc = " This means that when subtyping, we may end up with unconstrained"] #[doc = " inference variables if a generalized type has bivariant parameters."] #[doc = " A parameter may only be bivariant if it is constrained by a projection"] #[doc = " bound in a where-clause. As an example, imagine a type:"] #[doc = ""] #[doc = "     struct Foo<A, B> where A: Iterator<Item = B> {"] #[doc = "         data: A"] #[doc = "     }"] #[doc = ""] #[doc = " here, `A` will be covariant, but `B` is unconstrained."] #[doc = ""] #[doc = " However, whatever it is, for `Foo` to be WF, it must be equal to `A::Item`."] #[doc = " If we have an input `Foo<?A, ?B>`, then after generalization we will wind"] #[doc = " up with a type like `Foo<?C, ?D>`. When we enforce `Foo<?A, ?B> <: Foo<?C, ?D>`,"] #[doc = " we will wind up with the requirement that `?A <: ?C`, but no particular"] #[doc = " relationship between `?B` and `?D` (after all, these types may be completely"] #[doc = " different). If we do nothing else, this may mean that `?D` goes unconstrained"] #[doc = " (as in #41677). To avoid this we emit a `WellFormed` obligation in these cases."] pub has_unconstrained_ty_var : bool , }}}