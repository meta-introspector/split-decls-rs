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
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: mem ;}
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_data_structures :: thinvec :: ExtractIf ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_infer :: infer :: InferCtxt ;}
mkuse!{use rustc_infer :: traits :: query :: NoSolution ;}
mkuse!{use rustc_infer :: traits :: { FromSolverError , PredicateObligation , PredicateObligations , TraitEngine , } ;}
mkuse!{use rustc_middle :: ty :: { self , DelayedSet , Ty , TyCtxt , TypeSuperVisitable , TypeVisitable , TypeVisitableExt , TypeVisitor , TypingMode , } ;}
mkuse!{use rustc_next_trait_solver :: delegate :: SolverDelegate as _ ;}
mkuse!{use rustc_next_trait_solver :: solve :: { GoalEvaluation , GoalStalledOn , HasChanged , SolverDelegateEvalExt as _ , } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use self :: derive_errors :: * ;}
mkuse!{use super :: Certainty ;}
mkuse!{use super :: delegate :: SolverDelegate ;}
mkuse!{use super :: inspect :: { self , ProofTreeInferCtxtExt } ;}
mkuse!{use crate :: traits :: { FulfillmentError , ScrubbedTraitError } ;}
mkmod!{derive_errors, { 
                getname!(derive_errors);
                getsrc!(derive_errors);
                getpath!(derive_errors);
                get_deps!(derive_errors);
                get_crates!(derive_errors);
                mkinclude!(derive_errors);
                 
            }}
mkitem!{type PendingObligations < 'tcx > = ThinVec < (PredicateObligation < 'tcx > , Option < GoalStalledOn < TyCtxt < 'tcx > > >) > ;}
mkitem!{mkstruct!{#[doc = " A trait engine using the new trait solver."] #[doc = ""] #[doc = " This is mostly identical to how `evaluate_all` works inside of the"] #[doc = " solver, except that the requirements are slightly different."] #[doc = ""] #[doc = " Unlike `evaluate_all` it is possible to add new obligations later on"] #[doc = " and we also have to track diagnostics information by using `Obligation`"] #[doc = " instead of `Goal`."] #[doc = ""] #[doc = " It is also likely that we want to use slightly different datastructures"] #[doc = " here as this will have to deal with far more root goals than `evaluate_all`."] pub struct FulfillmentCtxt < 'tcx , E : 'tcx > { obligations : ObligationStorage < 'tcx > , #[doc = " The snapshot in which this context was created. Using the context"] #[doc = " outside of this snapshot leads to subtle bugs if the snapshot"] #[doc = " gets rolled back. Because of this we explicitly check that we only"] #[doc = " use the context in exactly this snapshot."] usable_in_snapshot : usize , _errors : PhantomData < E > , }}}
mkitem!{mkstruct!{#[derive (Default , Debug)] struct ObligationStorage < 'tcx > { #[doc = " Obligations which resulted in an overflow in fulfillment itself."] #[doc = ""] #[doc = " We cannot eagerly return these as error so we instead store them here"] #[doc = " to avoid recomputing them each time `select_where_possible` is called."] #[doc = " This also allows us to return the correct `FulfillmentError` for them."] overflowed : Vec < PredicateObligation < 'tcx > > , pending : PendingObligations < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > ObligationStorage < 'tcx > { fn register (& mut self , obligation : PredicateObligation < 'tcx > , stalled_on : Option < GoalStalledOn < TyCtxt < 'tcx > > > ,) { self . pending . push ((obligation , stalled_on)) ; } fn has_pending_obligations (& self) -> bool { ! self . pending . is_empty () || ! self . overflowed . is_empty () } fn clone_pending (& self) -> PredicateObligations < 'tcx > { let mut obligations : PredicateObligations < 'tcx > = self . pending . iter () . map (| (o , _) | o . clone ()) . collect () ; obligations . extend (self . overflowed . iter () . cloned ()) ; obligations } fn drain_pending (& mut self , cond : impl Fn (& PredicateObligation < 'tcx >) -> bool ,) -> PendingObligations < 'tcx > { let (unstalled , pending) = mem :: take (& mut self . pending) . into_iter () . partition (| (o , _) | cond (o)) ; self . pending = pending ; unstalled } fn on_fulfillment_overflow (& mut self , infcx : & InferCtxt < 'tcx >) { infcx . probe (| _ | { self . overflowed . extend (ExtractIf :: new (& mut self . pending , | (o , stalled_on) | { let goal = o . as_goal () ; let result = < & SolverDelegate < 'tcx > > :: from (infcx) . evaluate_root_goal (goal , o . cause . span , stalled_on . take () ,) ; matches ! (result , Ok (GoalEvaluation { has_changed : HasChanged :: Yes , .. })) }) . map (| (o , _) | o) ,) ; }) } }}}
mkitem!{mkimpl!{impl < 'tcx , E : 'tcx > FulfillmentCtxt < 'tcx , E > { pub fn new (infcx : & InferCtxt < 'tcx >) -> FulfillmentCtxt < 'tcx , E > { assert ! (infcx . next_trait_solver () , "new trait solver fulfillment context created when \
            infcx is set up for old trait solver") ; FulfillmentCtxt { obligations : Default :: default () , usable_in_snapshot : infcx . num_open_snapshots () , _errors : PhantomData , } } fn inspect_evaluated_obligation (& self , infcx : & InferCtxt < 'tcx > , obligation : & PredicateObligation < 'tcx > , result : & Result < GoalEvaluation < TyCtxt < 'tcx > > , NoSolution > ,) { if let Some (inspector) = infcx . obligation_inspector . get () { let result = match result { Ok (GoalEvaluation { certainty , .. }) => Ok (* certainty) , Err (NoSolution) => Err (NoSolution) , } ; (inspector) (infcx , & obligation , result) ; } } }}}
mkitem!{mkimpl!{impl < 'tcx , E > TraitEngine < 'tcx , E > for FulfillmentCtxt < 'tcx , E > where E : FromSolverError < 'tcx , NextSolverError < 'tcx > > , { #[instrument (level = "trace" , skip (self , infcx))] fn register_predicate_obligation (& mut self , infcx : & InferCtxt < 'tcx > , obligation : PredicateObligation < 'tcx > ,) { assert_eq ! (self . usable_in_snapshot , infcx . num_open_snapshots ()) ; self . obligations . register (obligation , None) ; } fn collect_remaining_errors (& mut self , infcx : & InferCtxt < 'tcx >) -> Vec < E > { self . obligations . pending . drain (..) . map (| (obligation , _) | NextSolverError :: Ambiguity (obligation)) . chain (self . obligations . overflowed . drain (..) . map (| obligation | NextSolverError :: Overflow (obligation)) ,) . map (| e | E :: from_solver_error (infcx , e)) . collect () } fn select_where_possible (& mut self , infcx : & InferCtxt < 'tcx >) -> Vec < E > { assert_eq ! (self . usable_in_snapshot , infcx . num_open_snapshots ()) ; let mut errors = Vec :: new () ; loop { let mut any_changed = false ; for (mut obligation , stalled_on) in self . obligations . drain_pending (| _ | true) { if ! infcx . tcx . recursion_limit () . value_within_limit (obligation . recursion_depth) { self . obligations . on_fulfillment_overflow (infcx) ; return errors ; } let goal = obligation . as_goal () ; let delegate = < & SolverDelegate < 'tcx > > :: from (infcx) ; if let Some (certainty) = delegate . compute_goal_fast_path (goal , obligation . cause . span) { match certainty { Certainty :: Yes => { } Certainty :: Maybe (_) => { self . obligations . register (obligation , None) ; } } continue ; } let result = delegate . evaluate_root_goal (goal , obligation . cause . span , stalled_on) ; self . inspect_evaluated_obligation (infcx , & obligation , & result) ; let GoalEvaluation { goal , certainty , has_changed , stalled_on } = match result { Ok (result) => result , Err (NoSolution) => { errors . push (E :: from_solver_error (infcx , NextSolverError :: TrueError (obligation) ,)) ; continue ; } } ; obligation . predicate = goal . predicate ; if has_changed == HasChanged :: Yes { obligation . recursion_depth += 1 ; any_changed = true ; } match certainty { Certainty :: Yes => { if infcx . in_hir_typeck && (obligation . has_non_region_infer () || obligation . has_free_regions ()) { infcx . push_hir_typeck_potentially_region_dependent_goal (obligation) ; } } Certainty :: Maybe (_) => self . obligations . register (obligation , stalled_on) , } } if ! any_changed { break ; } } errors } fn has_pending_obligations (& self) -> bool { self . obligations . has_pending_obligations () } fn pending_obligations (& self) -> PredicateObligations < 'tcx > { self . obligations . clone_pending () } fn drain_stalled_obligations_for_coroutines (& mut self , infcx : & InferCtxt < 'tcx > ,) -> PredicateObligations < 'tcx > { let stalled_coroutines = match infcx . typing_mode () { TypingMode :: Analysis { defining_opaque_types_and_generators } => { defining_opaque_types_and_generators } TypingMode :: Coherence | TypingMode :: Borrowck { defining_opaque_types : _ } | TypingMode :: PostBorrowckAnalysis { defined_opaque_types : _ } | TypingMode :: PostAnalysis => return Default :: default () , } ; if stalled_coroutines . is_empty () { return Default :: default () ; } self . obligations . drain_pending (| obl | { infcx . probe (| _ | { infcx . visit_proof_tree (obl . as_goal () , & mut StalledOnCoroutines { stalled_coroutines , span : obl . cause . span , cache : Default :: default () , } ,) . is_break () }) }) . into_iter () . map (| (o , _) | o) . collect () } }}}
mkitem!{mkstruct!{#[doc = " Detect if a goal is stalled on a coroutine that is owned by the current typeck root."] #[doc = ""] #[doc = " This function can (erroneously) fail to detect a predicate, i.e. it doesn't need to"] #[doc = " be complete. However, this will lead to ambiguity errors, so we want to make it"] #[doc = " accurate."] #[doc = ""] #[doc = " This function can be also return false positives, which will lead to poor diagnostics"] #[doc = " so we want to keep this visitor *precise* too."] pub struct StalledOnCoroutines < 'tcx > { pub stalled_coroutines : & 'tcx ty :: List < LocalDefId > , pub span : Span , pub cache : DelayedSet < Ty < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'tcx > inspect :: ProofTreeVisitor < 'tcx > for StalledOnCoroutines < 'tcx > { type Result = ControlFlow < () > ; fn span (& self) -> rustc_span :: Span { self . span } fn visit_goal (& mut self , inspect_goal : & super :: inspect :: InspectGoal < '_ , 'tcx >) -> Self :: Result { inspect_goal . goal () . predicate . visit_with (self) ? ; if let Some (candidate) = inspect_goal . unique_applicable_candidate () { candidate . visit_nested_no_probe (self) } else { ControlFlow :: Continue (()) } } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for StalledOnCoroutines < 'tcx > { type Result = ControlFlow < () > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { if ! self . cache . insert (ty) { return ControlFlow :: Continue (()) ; } if let ty :: Coroutine (def_id , _) = * ty . kind () && def_id . as_local () . is_some_and (| def_id | self . stalled_coroutines . contains (& def_id)) { ControlFlow :: Break (()) } else if ty . has_coroutines () { ty . super_visit_with (self) } else { ControlFlow :: Continue (()) } } }}}
mkitem!{mkenum!{pub enum NextSolverError < 'tcx > { TrueError (PredicateObligation < 'tcx >) , Ambiguity (PredicateObligation < 'tcx >) , Overflow (PredicateObligation < 'tcx >) , }}}
mkitem!{mkimpl!{impl < 'tcx > FromSolverError < 'tcx , NextSolverError < 'tcx > > for FulfillmentError < 'tcx > { fn from_solver_error (infcx : & InferCtxt < 'tcx > , error : NextSolverError < 'tcx >) -> Self { match error { NextSolverError :: TrueError (obligation) => { fulfillment_error_for_no_solution (infcx , obligation) } NextSolverError :: Ambiguity (obligation) => { fulfillment_error_for_stalled (infcx , obligation) } NextSolverError :: Overflow (obligation) => { fulfillment_error_for_overflow (infcx , obligation) } } } }}}
mkitem!{mkimpl!{impl < 'tcx > FromSolverError < 'tcx , NextSolverError < 'tcx > > for ScrubbedTraitError < 'tcx > { fn from_solver_error (_infcx : & InferCtxt < 'tcx > , error : NextSolverError < 'tcx >) -> Self { match error { NextSolverError :: TrueError (_) => ScrubbedTraitError :: TrueError , NextSolverError :: Ambiguity (_) | NextSolverError :: Overflow (_) => { ScrubbedTraitError :: Ambiguity } } } }}}