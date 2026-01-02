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
mkuse!{use std :: collections :: VecDeque ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashSet , FxIndexMap } ;}
mkuse!{use rustc_hir :: LangItem ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_infer :: infer :: InferCtxt ;}
mkuse!{use rustc_infer :: traits :: PolyTraitObligation ;}
mkuse!{pub use rustc_infer :: traits :: util :: * ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: fast_reject :: DeepRejectCtxt ;}
mkuse!{use rustc_middle :: ty :: { self , PolyTraitPredicate , PredicatePolarity , SizedTraitKind , TraitPredicate , TraitRef , Ty , TyCtxt , TypeFoldable , TypeFolder , TypeSuperFoldable , TypeVisitableExt , } ;}
mkuse!{pub use rustc_next_trait_solver :: placeholder :: BoundVarReplacer ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use smallvec :: { SmallVec , smallvec } ;}
mkuse!{use tracing :: debug ;}

macro_rules! expand_trait_aliases_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_trait_aliases in module {}", module_path!());
    };
}

mkfn!{
    expand_trait_aliases_introspect!();
    # [doc = " Return the trait and projection predicates that come from eagerly expanding the"] # [doc = " trait aliases in the list of clauses. For each trait predicate, record a stack"] # [doc = " of spans that trace from the user-written trait alias bound. For projection predicates,"] # [doc = " just record the span of the projection itself."] # [doc = ""] # [doc = " For trait aliases, we don't deduplicte the predicates, since we currently do not"] # [doc = " consider duplicated traits as a single trait for the purposes of our \"one trait principal\""] # [doc = " restriction; however, for projections we do deduplicate them."] # [doc = ""] # [doc = " ```rust,ignore (fails)"] # [doc = " trait Bar {}"] # [doc = " trait Foo = Bar + Bar;"] # [doc = ""] # [doc = " let dyn_incompatible: dyn Foo; // bad, two `Bar` principals."] # [doc = " ```"] pub fn expand_trait_aliases < 'tcx > (tcx : TyCtxt < 'tcx > , clauses : impl IntoIterator < Item = (ty :: Clause < 'tcx > , Span) > ,) -> (Vec < (ty :: PolyTraitPredicate < 'tcx > , SmallVec < [Span ; 1] >) > , Vec < (ty :: PolyProjectionPredicate < 'tcx > , Span) > ,) { let mut trait_preds = vec ! [] ; let mut projection_preds = vec ! [] ; let mut seen_projection_preds = FxHashSet :: default () ; let mut queue : VecDeque < _ > = clauses . into_iter () . map (| (p , s) | (p , smallvec ! [s])) . collect () ; while let Some ((clause , spans)) = queue . pop_front () { match clause . kind () . skip_binder () { ty :: ClauseKind :: Trait (trait_pred) => { if tcx . is_trait_alias (trait_pred . def_id ()) { queue . extend (tcx . explicit_super_predicates_of (trait_pred . def_id ()) . iter_identity_copied () . map (| (super_clause , span) | { let mut spans = spans . clone () ; spans . push (span) ; (super_clause . instantiate_supertrait (tcx , clause . kind () . rebind (trait_pred . trait_ref) ,) , spans ,) }) ,) ; } else { trait_preds . push ((clause . kind () . rebind (trait_pred) , spans)) ; } } ty :: ClauseKind :: Projection (projection_pred) => { let projection_pred = clause . kind () . rebind (projection_pred) ; if ! seen_projection_preds . insert (tcx . anonymize_bound_vars (projection_pred)) { continue ; } projection_preds . push ((projection_pred , * spans . last () . unwrap ())) ; } ty :: ClauseKind :: RegionOutlives (..) | ty :: ClauseKind :: TypeOutlives (..) | ty :: ClauseKind :: ConstArgHasType (_ , _) | ty :: ClauseKind :: WellFormed (_) | ty :: ClauseKind :: ConstEvaluatable (_) | ty :: ClauseKind :: UnstableFeature (_) | ty :: ClauseKind :: HostEffect (..) => { } } } (trait_preds , projection_preds) }
}

macro_rules! upcast_choices_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function upcast_choices in module {}", module_path!());
    };
}

mkfn!{
    upcast_choices_introspect!();
    # [doc = " Casts a trait reference into a reference to one of its super"] # [doc = " traits; returns `None` if `target_trait_def_id` is not a"] # [doc = " supertrait."] pub fn upcast_choices < 'tcx > (tcx : TyCtxt < 'tcx > , source_trait_ref : ty :: PolyTraitRef < 'tcx > , target_trait_def_id : DefId ,) -> Vec < ty :: PolyTraitRef < 'tcx > > { if source_trait_ref . def_id () == target_trait_def_id { return vec ! [source_trait_ref] ; } supertraits (tcx , source_trait_ref) . filter (| r | r . def_id () == target_trait_def_id) . collect () }
}

macro_rules! closure_trait_ref_and_return_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function closure_trait_ref_and_return_type in module {}", module_path!());
    };
}

mkfn!{
    closure_trait_ref_and_return_type_introspect!();
    pub (crate) fn closure_trait_ref_and_return_type < 'tcx > (tcx : TyCtxt < 'tcx > , fn_trait_def_id : DefId , self_ty : Ty < 'tcx > , sig : ty :: PolyFnSig < 'tcx > , tuple_arguments : TupleArgumentsFlag ,) -> ty :: Binder < 'tcx , (ty :: TraitRef < 'tcx > , Ty < 'tcx >) > { assert ! (! self_ty . has_escaping_bound_vars ()) ; let arguments_tuple = match tuple_arguments { TupleArgumentsFlag :: No => sig . skip_binder () . inputs () [0] , TupleArgumentsFlag :: Yes => Ty :: new_tup (tcx , sig . skip_binder () . inputs ()) , } ; let trait_ref = ty :: TraitRef :: new (tcx , fn_trait_def_id , [self_ty , arguments_tuple]) ; sig . map_bound (| sig | (trait_ref , sig . output ())) }
}

macro_rules! coroutine_trait_ref_and_outputs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function coroutine_trait_ref_and_outputs in module {}", module_path!());
    };
}

mkfn!{
    coroutine_trait_ref_and_outputs_introspect!();
    pub (crate) fn coroutine_trait_ref_and_outputs < 'tcx > (tcx : TyCtxt < 'tcx > , fn_trait_def_id : DefId , self_ty : Ty < 'tcx > , sig : ty :: GenSig < TyCtxt < 'tcx > > ,) -> (ty :: TraitRef < 'tcx > , Ty < 'tcx > , Ty < 'tcx >) { assert ! (! self_ty . has_escaping_bound_vars ()) ; let trait_ref = ty :: TraitRef :: new (tcx , fn_trait_def_id , [self_ty , sig . resume_ty]) ; (trait_ref , sig . yield_ty , sig . return_ty) }
}

macro_rules! future_trait_ref_and_outputs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function future_trait_ref_and_outputs in module {}", module_path!());
    };
}

mkfn!{
    future_trait_ref_and_outputs_introspect!();
    pub (crate) fn future_trait_ref_and_outputs < 'tcx > (tcx : TyCtxt < 'tcx > , fn_trait_def_id : DefId , self_ty : Ty < 'tcx > , sig : ty :: GenSig < TyCtxt < 'tcx > > ,) -> (ty :: TraitRef < 'tcx > , Ty < 'tcx >) { assert ! (! self_ty . has_escaping_bound_vars ()) ; let trait_ref = ty :: TraitRef :: new (tcx , fn_trait_def_id , [self_ty]) ; (trait_ref , sig . return_ty) }
}

macro_rules! iterator_trait_ref_and_outputs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iterator_trait_ref_and_outputs in module {}", module_path!());
    };
}

mkfn!{
    iterator_trait_ref_and_outputs_introspect!();
    pub (crate) fn iterator_trait_ref_and_outputs < 'tcx > (tcx : TyCtxt < 'tcx > , iterator_def_id : DefId , self_ty : Ty < 'tcx > , sig : ty :: GenSig < TyCtxt < 'tcx > > ,) -> (ty :: TraitRef < 'tcx > , Ty < 'tcx >) { assert ! (! self_ty . has_escaping_bound_vars ()) ; let trait_ref = ty :: TraitRef :: new (tcx , iterator_def_id , [self_ty]) ; (trait_ref , sig . yield_ty) }
}

macro_rules! async_iterator_trait_ref_and_outputs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function async_iterator_trait_ref_and_outputs in module {}", module_path!());
    };
}

mkfn!{
    async_iterator_trait_ref_and_outputs_introspect!();
    pub (crate) fn async_iterator_trait_ref_and_outputs < 'tcx > (tcx : TyCtxt < 'tcx > , async_iterator_def_id : DefId , self_ty : Ty < 'tcx > , sig : ty :: GenSig < TyCtxt < 'tcx > > ,) -> (ty :: TraitRef < 'tcx > , Ty < 'tcx >) { assert ! (! self_ty . has_escaping_bound_vars ()) ; let trait_ref = ty :: TraitRef :: new (tcx , async_iterator_def_id , [self_ty]) ; (trait_ref , sig . yield_ty) }
}

macro_rules! impl_item_is_final_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function impl_item_is_final in module {}", module_path!());
    };
}

mkfn!{
    impl_item_is_final_introspect!();
    pub fn impl_item_is_final (tcx : TyCtxt < '_ > , assoc_item : & ty :: AssocItem) -> bool { assoc_item . defaultness (tcx) . is_final () && tcx . defaultness (assoc_item . container_id (tcx)) . is_final () }
}
mkitem!{mkenum!{pub (crate) enum TupleArgumentsFlag { Yes , No , }}}

macro_rules! with_replaced_escaping_bound_vars_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_replaced_escaping_bound_vars in module {}", module_path!());
    };
}

mkfn!{
    with_replaced_escaping_bound_vars_introspect!();
    # [doc = " Executes `f` on `value` after replacing all escaping bound variables with placeholders"] # [doc = " and then replaces these placeholders with the original bound variables in the result."] # [doc = ""] # [doc = " In most places, bound variables should be replaced right when entering a binder, making"] # [doc = " this function unnecessary. However, normalization currently does not do that, so we have"] # [doc = " to do this lazily."] # [doc = ""] # [doc = " You should not add any additional uses of this function, at least not without first"] # [doc = " discussing it with t-types."] # [doc = ""] # [doc = " FIXME(@lcnr): We may even consider experimenting with eagerly replacing bound vars during"] # [doc = " normalization as well, at which point this function will be unnecessary and can be removed."] pub fn with_replaced_escaping_bound_vars < 'a , 'tcx , T : TypeFoldable < TyCtxt < 'tcx > > , R : TypeFoldable < TyCtxt < 'tcx > > , > (infcx : & 'a InferCtxt < 'tcx > , universe_indices : & 'a mut Vec < Option < ty :: UniverseIndex > > , value : T , f : impl FnOnce (T) -> R ,) -> R { if value . has_escaping_bound_vars () { let (value , mapped_regions , mapped_types , mapped_consts) = BoundVarReplacer :: replace_bound_vars (infcx , universe_indices , value) ; let result = f (value) ; PlaceholderReplacer :: replace_placeholders (infcx , mapped_regions , mapped_types , mapped_consts , universe_indices , result ,) } else { f (value) } }
}
mkitem!{mkstruct!{# [doc = " The inverse of [`BoundVarReplacer`]: replaces placeholders with the bound vars from which they came."] pub struct PlaceholderReplacer < 'a , 'tcx > { infcx : & 'a InferCtxt < 'tcx > , mapped_regions : FxIndexMap < ty :: PlaceholderRegion , ty :: BoundRegion > , mapped_types : FxIndexMap < ty :: PlaceholderType , ty :: BoundTy > , mapped_consts : FxIndexMap < ty :: PlaceholderConst , ty :: BoundConst > , universe_indices : & 'a [Option < ty :: UniverseIndex >] , current_index : ty :: DebruijnIndex , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > PlaceholderReplacer < 'a , 'tcx > { pub fn replace_placeholders < T : TypeFoldable < TyCtxt < 'tcx > > > (infcx : & 'a InferCtxt < 'tcx > , mapped_regions : FxIndexMap < ty :: PlaceholderRegion , ty :: BoundRegion > , mapped_types : FxIndexMap < ty :: PlaceholderType , ty :: BoundTy > , mapped_consts : FxIndexMap < ty :: PlaceholderConst , ty :: BoundConst > , universe_indices : & 'a [Option < ty :: UniverseIndex >] , value : T ,) -> T { let mut replacer = PlaceholderReplacer { infcx , mapped_regions , mapped_types , mapped_consts , universe_indices , current_index : ty :: INNERMOST , } ; value . fold_with (& mut replacer) } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for PlaceholderReplacer < '_ , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . infcx . tcx } fn fold_binder < T : TypeFoldable < TyCtxt < 'tcx > > > (& mut self , t : ty :: Binder < 'tcx , T > ,) -> ty :: Binder < 'tcx , T > { if ! t . has_placeholders () && ! t . has_infer () { return t ; } self . current_index . shift_in (1) ; let t = t . super_fold_with (self) ; self . current_index . shift_out (1) ; t } fn fold_region (& mut self , r0 : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { let r1 = match r0 . kind () { ty :: ReVar (vid) => self . infcx . inner . borrow_mut () . unwrap_region_constraints () . opportunistic_resolve_var (self . infcx . tcx , vid) , _ => r0 , } ; let r2 = match r1 . kind () { ty :: RePlaceholder (p) => { let replace_var = self . mapped_regions . get (& p) ; match replace_var { Some (replace_var) => { let index = self . universe_indices . iter () . position (| u | matches ! (u , Some (pu) if * pu == p . universe)) . unwrap_or_else (| | bug ! ("Unexpected placeholder universe.")) ; let db = ty :: DebruijnIndex :: from_usize (self . universe_indices . len () - index + self . current_index . as_usize () - 1 ,) ; ty :: Region :: new_bound (self . cx () , db , * replace_var) } None => r1 , } } _ => r1 , } ; debug ! (? r0 , ? r1 , ? r2 , "fold_region") ; r2 } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { let ty = self . infcx . shallow_resolve (ty) ; match * ty . kind () { ty :: Placeholder (p) => { let replace_var = self . mapped_types . get (& p) ; match replace_var { Some (replace_var) => { let index = self . universe_indices . iter () . position (| u | matches ! (u , Some (pu) if * pu == p . universe)) . unwrap_or_else (| | bug ! ("Unexpected placeholder universe.")) ; let db = ty :: DebruijnIndex :: from_usize (self . universe_indices . len () - index + self . current_index . as_usize () - 1 ,) ; Ty :: new_bound (self . infcx . tcx , db , * replace_var) } None => { if ty . has_infer () { ty . super_fold_with (self) } else { ty } } } } _ if ty . has_placeholders () || ty . has_infer () => ty . super_fold_with (self) , _ => ty , } } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { let ct = self . infcx . shallow_resolve_const (ct) ; if let ty :: ConstKind :: Placeholder (p) = ct . kind () { let replace_var = self . mapped_consts . get (& p) ; match replace_var { Some (replace_var) => { let index = self . universe_indices . iter () . position (| u | matches ! (u , Some (pu) if * pu == p . universe)) . unwrap_or_else (| | bug ! ("Unexpected placeholder universe.")) ; let db = ty :: DebruijnIndex :: from_usize (self . universe_indices . len () - index + self . current_index . as_usize () - 1 ,) ; ty :: Const :: new_bound (self . infcx . tcx , db , * replace_var) } None => { if ct . has_infer () { ct . super_fold_with (self) } else { ct } } } } else { ct . super_fold_with (self) } } }}}

macro_rules! sizedness_fast_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sizedness_fast_path in module {}", module_path!());
    };
}

mkfn!{
    sizedness_fast_path_introspect!();
    pub fn sizedness_fast_path < 'tcx > (tcx : TyCtxt < 'tcx > , predicate : ty :: Predicate < 'tcx > , param_env : ty :: ParamEnv < 'tcx > ,) -> bool { if let ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (trait_pred)) = predicate . kind () . skip_binder () && trait_pred . polarity == ty :: PredicatePolarity :: Positive { let sizedness = match tcx . as_lang_item (trait_pred . def_id ()) { Some (LangItem :: Sized) => SizedTraitKind :: Sized , Some (LangItem :: MetaSized) => SizedTraitKind :: MetaSized , _ => return false , } ; if ! tcx . features () . sized_hierarchy () && matches ! (sizedness , SizedTraitKind :: MetaSized) { return true ; } if trait_pred . self_ty () . has_trivial_sizedness (tcx , sizedness) { debug ! ("fast path -- trivial sizedness") ; return true ; } if matches ! (trait_pred . self_ty () . kind () , ty :: Param (_) | ty :: Placeholder (_)) { for clause in param_env . caller_bounds () { if let ty :: ClauseKind :: Trait (clause_pred) = clause . kind () . skip_binder () && clause_pred . polarity == ty :: PredicatePolarity :: Positive && clause_pred . self_ty () == trait_pred . self_ty () && (clause_pred . def_id () == trait_pred . def_id () || (sizedness == SizedTraitKind :: MetaSized && tcx . is_lang_item (clause_pred . def_id () , LangItem :: Sized))) { return true ; } } } } false }
}

macro_rules! lazily_elaborate_sizedness_candidate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lazily_elaborate_sizedness_candidate in module {}", module_path!());
    };
}

mkfn!{
    lazily_elaborate_sizedness_candidate_introspect!();
    # [doc = " To improve performance, sizedness traits are not elaborated and so special-casing is required"] # [doc = " in the trait solver to find a `Sized` candidate for a `MetaSized` obligation. Returns the"] # [doc = " predicate to used in the candidate for such a `obligation`, given a `candidate`."] pub (crate) fn lazily_elaborate_sizedness_candidate < 'tcx > (infcx : & InferCtxt < 'tcx > , obligation : & PolyTraitObligation < 'tcx > , candidate : PolyTraitPredicate < 'tcx > ,) -> PolyTraitPredicate < 'tcx > { if ! infcx . tcx . is_lang_item (obligation . predicate . def_id () , LangItem :: MetaSized) || ! infcx . tcx . is_lang_item (candidate . def_id () , LangItem :: Sized) { return candidate ; } if obligation . predicate . polarity () != PredicatePolarity :: Positive || candidate . polarity () != PredicatePolarity :: Positive { return candidate ; } let drcx = DeepRejectCtxt :: relate_rigid_rigid (infcx . tcx) ; if ! drcx . args_may_unify (obligation . predicate . skip_binder () . trait_ref . args , candidate . skip_binder () . trait_ref . args ,) { return candidate ; } candidate . map_bound (| c | TraitPredicate { trait_ref : TraitRef :: new_from_args (infcx . tcx , obligation . predicate . def_id () , c . trait_ref . args ,) , polarity : c . polarity , }) }
}