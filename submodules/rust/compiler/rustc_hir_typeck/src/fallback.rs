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
mkuse!{use std :: cell :: OnceCell ;}
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_data_structures :: graph :: iterate :: DepthFirstSearch ;}
mkuse!{use rustc_data_structures :: graph :: vec_graph :: VecGraph ;}
mkuse!{use rustc_data_structures :: graph :: { self } ;}
mkuse!{use rustc_data_structures :: unord :: { UnordBag , UnordMap , UnordSet } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: HirId ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: intravisit :: { InferKind , Visitor } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt , TypeSuperVisitable , TypeVisitable } ;}
mkuse!{use rustc_session :: lint ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use rustc_trait_selection :: traits :: { ObligationCause , ObligationCtxt } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: typeck_root_ctxt :: InferVarInfo ;}
mkuse!{use crate :: { FnCtxt , errors } ;}
mkitem!{mkenum!{#[derive (Copy , Clone)] pub (crate) enum DivergingFallbackBehavior { #[doc = " Always fallback to `()` (aka \"always spontaneous decay\")"] ToUnit , #[doc = " Sometimes fallback to `!`, but mainly fallback to `()` so that most of the crates are not broken."] ContextDependent , #[doc = " Always fallback to `!` (which should be equivalent to never falling back + not making"] #[doc = " never-to-any coercions unless necessary)"] ToNever , #[doc = " Don't fallback at all"] NoFallback , }}}
mkitem!{mkimpl!{impl < 'tcx > FnCtxt < '_ , 'tcx > { #[doc = " Performs type inference fallback, setting `FnCtxt::fallback_has_occurred`"] #[doc = " if fallback has occurred."] pub (super) fn type_inference_fallback (& self) { debug ! ("type-inference-fallback start obligations: {:#?}" , self . fulfillment_cx . borrow_mut () . pending_obligations ()) ; self . select_obligations_where_possible (| _ | { }) ; debug ! ("type-inference-fallback post selection obligations: {:#?}" , self . fulfillment_cx . borrow_mut () . pending_obligations ()) ; let fallback_occurred = self . fallback_types () ; if ! fallback_occurred { return ; } self . select_obligations_where_possible (| _ | { }) ; } fn fallback_types (& self) -> bool { let unresolved_variables = self . unresolved_variables () ; if unresolved_variables . is_empty () { return false ; } let diverging_fallback = self . calculate_diverging_fallback (& unresolved_variables , self . diverging_fallback_behavior) ; let mut fallback_occurred = false ; for ty in unresolved_variables { debug ! ("unsolved_variable = {:?}" , ty) ; fallback_occurred |= self . fallback_if_possible (ty , & diverging_fallback) ; } fallback_occurred } fn fallback_if_possible (& self , ty : Ty < 'tcx > , diverging_fallback : & UnordMap < Ty < 'tcx > , Ty < 'tcx > > ,) -> bool { let fallback = match ty . kind () { _ if let Some (e) = self . tainted_by_errors () => Ty :: new_error (self . tcx , e) , ty :: Infer (ty :: IntVar (_)) => self . tcx . types . i32 , ty :: Infer (ty :: FloatVar (_)) => self . tcx . types . f64 , _ => match diverging_fallback . get (& ty) { Some (& fallback_ty) => fallback_ty , None => return false , } , } ; debug ! ("fallback_if_possible(ty={:?}): defaulting to `{:?}`" , ty , fallback) ; let span = ty . ty_vid () . map_or (DUMMY_SP , | vid | self . infcx . type_var_origin (vid) . span) ; self . demand_eqtype (span , ty , fallback) ; self . fallback_has_occurred . set (true) ; true } #[doc = " The \"diverging fallback\" system is rather complicated. This is"] #[doc = " a result of our need to balance 'do the right thing' with"] #[doc = " backwards compatibility."] #[doc = ""] #[doc = " \"Diverging\" type variables are variables created when we"] #[doc = " coerce a `!` type into an unbound type variable `?X`. If they"] #[doc = " never wind up being constrained, the \"right and natural\" thing"] #[doc = " is that `?X` should \"fallback\" to `!`. This means that e.g. an"] #[doc = " expression like `Some(return)` will ultimately wind up with a"] #[doc = " type like `Option<!>` (presuming it is not assigned or"] #[doc = " constrained to have some other type)."] #[doc = ""] #[doc = " However, the fallback used to be `()` (before the `!` type was"] #[doc = " added). Moreover, there are cases where the `!` type 'leaks"] #[doc = " out' from dead code into type variables that affect live"] #[doc = " code. The most common case is something like this:"] #[doc = ""] #[doc = " ```rust"] #[doc = " # fn foo() -> i32 { 4 }"] #[doc = " match foo() {"] #[doc = "     22 => Default::default(), // call this type `?D`"] #[doc = "     _ => return, // return has type `!`"] #[doc = " } // call the type of this match `?M`"] #[doc = " ```"] #[doc = ""] #[doc = " Here, coercing the type `!` into `?M` will create a diverging"] #[doc = " type variable `?X` where `?X <: ?M`. We also have that `?D <:"] #[doc = " ?M`. If `?M` winds up unconstrained, then `?X` will"] #[doc = " fallback. If it falls back to `!`, then all the type variables"] #[doc = " will wind up equal to `!` -- this includes the type `?D`"] #[doc = " (since `!` doesn't implement `Default`, we wind up a \"trait"] #[doc = " not implemented\" error in code like this). But since the"] #[doc = " original fallback was `()`, this code used to compile with `?D"] #[doc = " = ()`. This is somewhat surprising, since `Default::default()`"] #[doc = " on its own would give an error because the types are"] #[doc = " insufficiently constrained."] #[doc = ""] #[doc = " Our solution to this dilemma is to modify diverging variables"] #[doc = " so that they can *either* fallback to `!` (the default) or to"] #[doc = " `()` (the backwards compatibility case). We decide which"] #[doc = " fallback to use based on whether there is a coercion pattern"] #[doc = " like this:"] #[doc = ""] #[doc = " ```ignore (not-rust)"] #[doc = " ?Diverging -> ?V"] #[doc = " ?NonDiverging -> ?V"] #[doc = " ?V != ?NonDiverging"] #[doc = " ```"] #[doc = ""] #[doc = " Here `?Diverging` represents some diverging type variable and"] #[doc = " `?NonDiverging` represents some non-diverging type"] #[doc = " variable. `?V` can be any type variable (diverging or not), so"] #[doc = " long as it is not equal to `?NonDiverging`."] #[doc = ""] #[doc = " Intuitively, what we are looking for is a case where a"] #[doc = " \"non-diverging\" type variable (like `?M` in our example above)"] #[doc = " is coerced *into* some variable `?V` that would otherwise"] #[doc = " fallback to `!`. In that case, we make `?V` fallback to `!`,"] #[doc = " along with anything that would flow into `?V`."] #[doc = ""] #[doc = " The algorithm we use:"] #[doc = " * Identify all variables that are coerced *into* by a"] #[doc = "   diverging variable. Do this by iterating over each"] #[doc = "   diverging, unsolved variable and finding all variables"] #[doc = "   reachable from there. Call that set `D`."] #[doc = " * Walk over all unsolved, non-diverging variables, and find"] #[doc = "   any variable that has an edge into `D`."] fn calculate_diverging_fallback (& self , unresolved_variables : & [Ty < 'tcx >] , behavior : DivergingFallbackBehavior ,) -> UnordMap < Ty < 'tcx > , Ty < 'tcx > > { debug ! ("calculate_diverging_fallback({:?})" , unresolved_variables) ; let coercion_graph = self . create_coercion_graph () ; let unsolved_vids = unresolved_variables . iter () . filter_map (| ty | ty . ty_vid ()) ; let diverging_roots : UnordSet < ty :: TyVid > = self . diverging_type_vars . borrow () . items () . map (| & ty | self . shallow_resolve (ty)) . filter_map (| ty | ty . ty_vid ()) . map (| vid | self . root_var (vid)) . collect () ; debug ! ("calculate_diverging_fallback: diverging_type_vars={:?}" , self . diverging_type_vars . borrow ()) ; debug ! ("calculate_diverging_fallback: diverging_roots={:?}" , diverging_roots) ; let mut roots_reachable_from_diverging = DepthFirstSearch :: new (& coercion_graph) ; let mut diverging_vids = vec ! [] ; let mut non_diverging_vids = vec ! [] ; for unsolved_vid in unsolved_vids { let root_vid = self . root_var (unsolved_vid) ; debug ! ("calculate_diverging_fallback: unsolved_vid={:?} root_vid={:?} diverges={:?}" , unsolved_vid , root_vid , diverging_roots . contains (& root_vid) ,) ; if diverging_roots . contains (& root_vid) { diverging_vids . push (unsolved_vid) ; roots_reachable_from_diverging . push_start_node (root_vid) ; debug ! ("calculate_diverging_fallback: root_vid={:?} reaches {:?}" , root_vid , graph :: depth_first_search (& coercion_graph , root_vid) . collect ::< Vec < _ >> ()) ; roots_reachable_from_diverging . complete_search () ; } else { non_diverging_vids . push (unsolved_vid) ; } } debug ! ("calculate_diverging_fallback: roots_reachable_from_diverging={:?}" , roots_reachable_from_diverging ,) ; let mut roots_reachable_from_non_diverging = DepthFirstSearch :: new (& coercion_graph) ; for & non_diverging_vid in & non_diverging_vids { let root_vid = self . root_var (non_diverging_vid) ; if roots_reachable_from_diverging . visited (root_vid) { continue ; } roots_reachable_from_non_diverging . push_start_node (root_vid) ; roots_reachable_from_non_diverging . complete_search () ; } debug ! ("calculate_diverging_fallback: roots_reachable_from_non_diverging={:?}" , roots_reachable_from_non_diverging ,) ; debug ! ("obligations: {:#?}" , self . fulfillment_cx . borrow_mut () . pending_obligations ()) ; let mut diverging_fallback = UnordMap :: with_capacity (diverging_vids . len ()) ; let unsafe_infer_vars = OnceCell :: new () ; self . lint_obligations_broken_by_never_type_fallback_change (behavior , & diverging_vids , & coercion_graph ,) ; for & diverging_vid in & diverging_vids { let diverging_ty = Ty :: new_var (self . tcx , diverging_vid) ; let root_vid = self . root_var (diverging_vid) ; let can_reach_non_diverging = graph :: depth_first_search (& coercion_graph , root_vid) . any (| n | roots_reachable_from_non_diverging . visited (n)) ; let infer_var_infos : UnordBag < _ > = self . infer_var_info . borrow () . items () . filter (| & (vid , _) | self . infcx . root_var (* vid) == root_vid) . map (| (_ , info) | * info) . collect () ; let found_infer_var_info = InferVarInfo { self_in_trait : infer_var_infos . items () . any (| info | info . self_in_trait) , output : infer_var_infos . items () . any (| info | info . output) , } ; let mut fallback_to = | ty | { self . lint_never_type_fallback_flowing_into_unsafe_code (& unsafe_infer_vars , & coercion_graph , root_vid ,) ; diverging_fallback . insert (diverging_ty , ty) ; } ; match behavior { DivergingFallbackBehavior :: ToUnit => { debug ! ("fallback to () - legacy: {:?}" , diverging_vid) ; fallback_to (self . tcx . types . unit) ; } DivergingFallbackBehavior :: ContextDependent => { if found_infer_var_info . self_in_trait && found_infer_var_info . output { debug ! ("fallback to () - found trait and projection: {:?}" , diverging_vid) ; fallback_to (self . tcx . types . unit) ; } else if can_reach_non_diverging { debug ! ("fallback to () - reached non-diverging: {:?}" , diverging_vid) ; fallback_to (self . tcx . types . unit) ; } else { debug ! ("fallback to ! - all diverging: {:?}" , diverging_vid) ; fallback_to (self . tcx . types . never) ; } } DivergingFallbackBehavior :: ToNever => { debug ! ("fallback to ! - `rustc_never_type_mode = \"fallback_to_never\")`: {:?}" , diverging_vid) ; fallback_to (self . tcx . types . never) ; } DivergingFallbackBehavior :: NoFallback => { debug ! ("no fallback - `rustc_never_type_mode = \"no_fallback\"`: {:?}" , diverging_vid) ; } } } diverging_fallback } fn lint_never_type_fallback_flowing_into_unsafe_code (& self , unsafe_infer_vars : & OnceCell < UnordMap < ty :: TyVid , (HirId , Span , UnsafeUseReason) > > , coercion_graph : & VecGraph < ty :: TyVid , true > , root_vid : ty :: TyVid ,) { let unsafe_infer_vars = unsafe_infer_vars . get_or_init (| | { let unsafe_infer_vars = compute_unsafe_infer_vars (self , self . body_id) ; debug ! (? unsafe_infer_vars) ; unsafe_infer_vars }) ; let affected_unsafe_infer_vars = graph :: depth_first_search_as_undirected (& coercion_graph , root_vid) . filter_map (| x | unsafe_infer_vars . get (& x) . copied ()) . collect :: < Vec < _ > > () ; let sugg = self . try_to_suggest_annotations (& [root_vid] , coercion_graph) ; for (hir_id , span , reason) in affected_unsafe_infer_vars { self . tcx . emit_node_span_lint (lint :: builtin :: NEVER_TYPE_FALLBACK_FLOWING_INTO_UNSAFE , hir_id , span , match reason { UnsafeUseReason :: Call => { errors :: NeverTypeFallbackFlowingIntoUnsafe :: Call { sugg : sugg . clone () } } UnsafeUseReason :: Method => { errors :: NeverTypeFallbackFlowingIntoUnsafe :: Method { sugg : sugg . clone () } } UnsafeUseReason :: Path => { errors :: NeverTypeFallbackFlowingIntoUnsafe :: Path { sugg : sugg . clone () } } UnsafeUseReason :: UnionField => { errors :: NeverTypeFallbackFlowingIntoUnsafe :: UnionField { sugg : sugg . clone () , } } UnsafeUseReason :: Deref => { errors :: NeverTypeFallbackFlowingIntoUnsafe :: Deref { sugg : sugg . clone () } } } ,) ; } } fn lint_obligations_broken_by_never_type_fallback_change (& self , behavior : DivergingFallbackBehavior , diverging_vids : & [ty :: TyVid] , coercions : & VecGraph < ty :: TyVid , true > ,) { let DivergingFallbackBehavior :: ToUnit = behavior else { return } ; if diverging_vids . is_empty () { return ; } let remaining_errors_if_fallback_to = | fallback | { self . probe (| _ | { let obligations = self . fulfillment_cx . borrow () . pending_obligations () ; let ocx = ObligationCtxt :: new_with_diagnostics (& self . infcx) ; ocx . register_obligations (obligations . iter () . cloned ()) ; for & diverging_vid in diverging_vids { let diverging_ty = Ty :: new_var (self . tcx , diverging_vid) ; ocx . eq (& ObligationCause :: dummy () , self . param_env , diverging_ty , fallback) . expect ("expected diverging var to be unconstrained") ; } ocx . select_where_possible () }) } ; let unit_errors = remaining_errors_if_fallback_to (self . tcx . types . unit) ; if unit_errors . is_empty () && let mut never_errors = remaining_errors_if_fallback_to (self . tcx . types . never) && let [never_error , ..] = never_errors . as_mut_slice () { self . adjust_fulfillment_error_for_expr_obligation (never_error) ; let sugg = self . try_to_suggest_annotations (diverging_vids , coercions) ; self . tcx . emit_node_span_lint (lint :: builtin :: DEPENDENCY_ON_UNIT_NEVER_TYPE_FALLBACK , self . tcx . local_def_id_to_hir_id (self . body_id) , self . tcx . def_span (self . body_id) , errors :: DependencyOnUnitNeverTypeFallback { obligation_span : never_error . obligation . cause . span , obligation : never_error . obligation . predicate , sugg , } ,) } } #[doc = " Returns a graph whose nodes are (unresolved) inference variables and where"] #[doc = " an edge `?A -> ?B` indicates that the variable `?A` is coerced to `?B`."] fn create_coercion_graph (& self) -> VecGraph < ty :: TyVid , true > { let pending_obligations = self . fulfillment_cx . borrow_mut () . pending_obligations () ; debug ! ("create_coercion_graph: pending_obligations={:?}" , pending_obligations) ; let coercion_edges : Vec < (ty :: TyVid , ty :: TyVid) > = pending_obligations . into_iter () . filter_map (| obligation | { obligation . predicate . kind () . no_bound_vars () }) . filter_map (| atom | { let (a , b) = match atom { ty :: PredicateKind :: Coerce (ty :: CoercePredicate { a , b }) => (a , b) , ty :: PredicateKind :: Subtype (ty :: SubtypePredicate { a_is_expected : _ , a , b }) => { (a , b) } _ => return None , } ; let a_vid = self . root_vid (a) ? ; let b_vid = self . root_vid (b) ? ; Some ((a_vid , b_vid)) }) . collect () ; debug ! ("create_coercion_graph: coercion_edges={:?}" , coercion_edges) ; let num_ty_vars = self . num_ty_vars () ; VecGraph :: new (num_ty_vars , coercion_edges) } #[doc = " If `ty` is an unresolved type variable, returns its root vid."] fn root_vid (& self , ty : Ty < 'tcx >) -> Option < ty :: TyVid > { Some (self . root_var (self . shallow_resolve (ty) . ty_vid () ?)) } #[doc = " Given a set of diverging vids and coercions, walk the HIR to gather a"] #[doc = " set of suggestions which can be applied to preserve fallback to unit."] fn try_to_suggest_annotations (& self , diverging_vids : & [ty :: TyVid] , coercions : & VecGraph < ty :: TyVid , true > ,) -> errors :: SuggestAnnotations { let body = self . tcx . hir_maybe_body_owned_by (self . body_id) . expect ("body id must have an owner") ; let suggestions = diverging_vids . iter () . copied () . filter_map (| vid | { let reachable_vids = graph :: depth_first_search_as_undirected (coercions , vid) . collect () ; AnnotateUnitFallbackVisitor { reachable_vids , fcx : self } . visit_expr (body . value) . break_value () }) . collect () ; errors :: SuggestAnnotations { suggestions } } }}}
mkitem!{mkstruct!{#[doc = " Try to walk the HIR to find a place to insert a useful suggestion"] #[doc = " to preserve fallback to `()` in 2024."] struct AnnotateUnitFallbackVisitor < 'a , 'tcx > { reachable_vids : FxHashSet < ty :: TyVid > , fcx : & 'a FnCtxt < 'a , 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > AnnotateUnitFallbackVisitor < '_ , 'tcx > { fn suggest_for_segment (& self , arg_segment : & 'tcx hir :: PathSegment < 'tcx > , def_id : DefId , id : HirId ,) -> ControlFlow < errors :: SuggestAnnotation > { if arg_segment . args . is_none () && let Some (all_args) = self . fcx . typeck_results . borrow () . node_args_opt (id) && let generics = self . fcx . tcx . generics_of (def_id) && let args = all_args [generics . parent_count ..] . iter () . zip (& generics . own_params) && args . clone () . all (| (_ , param) | matches ! (param . kind , ty :: GenericParamDefKind :: Type { .. } | ty :: GenericParamDefKind :: Lifetime)) { let non_apit_type_args = args . filter (| (_ , param) | { matches ! (param . kind , ty :: GenericParamDefKind :: Type { synthetic : false , .. }) }) ; let n_tys = non_apit_type_args . clone () . count () ; for (idx , (arg , _)) in non_apit_type_args . enumerate () { if let Some (ty) = arg . as_type () && let Some (vid) = self . fcx . root_vid (ty) && self . reachable_vids . contains (& vid) { return ControlFlow :: Break (errors :: SuggestAnnotation :: Turbo (arg_segment . ident . span . shrink_to_hi () , n_tys , idx ,)) ; } } } ControlFlow :: Continue (()) } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for AnnotateUnitFallbackVisitor < '_ , 'tcx > { type Result = ControlFlow < errors :: SuggestAnnotation > ; fn visit_infer (& mut self , inf_id : HirId , inf_span : Span , _kind : InferKind < 'tcx > ,) -> Self :: Result { if let Some (ty) = self . fcx . typeck_results . borrow () . node_type_opt (inf_id) && let Some (vid) = self . fcx . root_vid (ty) && self . reachable_vids . contains (& vid) && inf_span . can_be_used_for_suggestions () { return ControlFlow :: Break (errors :: SuggestAnnotation :: Unit (inf_span)) ; } ControlFlow :: Continue (()) } fn visit_qpath (& mut self , qpath : & 'tcx rustc_hir :: QPath < 'tcx > , id : HirId , span : Span ,) -> Self :: Result { let arg_segment = match qpath { hir :: QPath :: Resolved (_ , path) => { path . segments . last () . expect ("paths should have a segment") } hir :: QPath :: TypeRelative (_ , segment) => segment , hir :: QPath :: LangItem (..) => { return hir :: intravisit :: walk_qpath (self , qpath , id) ; } } ; if let Some (def_id) = self . fcx . typeck_results . borrow () . qpath_res (qpath , id) . opt_def_id () && span . can_be_used_for_suggestions () { self . suggest_for_segment (arg_segment , def_id , id) ? ; } hir :: intravisit :: walk_qpath (self , qpath , id) } fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) -> Self :: Result { if let hir :: ExprKind :: Closure (& hir :: Closure { body , .. }) | hir :: ExprKind :: ConstBlock (hir :: ConstBlock { body , .. }) = expr . kind { self . visit_body (self . fcx . tcx . hir_body (body)) ? ; } if let hir :: ExprKind :: Path (hir :: QPath :: Resolved (None , path)) = expr . kind && let Res :: Def (DefKind :: AssocFn , def_id) = path . res && self . fcx . tcx . trait_of_assoc (def_id) . is_some () && let Some (args) = self . fcx . typeck_results . borrow () . node_args_opt (expr . hir_id) && let self_ty = args . type_at (0) && let Some (vid) = self . fcx . root_vid (self_ty) && self . reachable_vids . contains (& vid) && let [.. , trait_segment , _method_segment] = path . segments && expr . span . can_be_used_for_suggestions () { let span = path . span . shrink_to_lo () . to (trait_segment . ident . span) ; return ControlFlow :: Break (errors :: SuggestAnnotation :: Path (span)) ; } if let hir :: ExprKind :: MethodCall (segment , ..) = expr . kind && let Some (def_id) = self . fcx . typeck_results . borrow () . type_dependent_def_id (expr . hir_id) && expr . span . can_be_used_for_suggestions () { self . suggest_for_segment (segment , def_id , expr . hir_id) ? ; } hir :: intravisit :: walk_expr (self , expr) } fn visit_local (& mut self , local : & 'tcx hir :: LetStmt < 'tcx >) -> Self :: Result { if let hir :: LocalSource :: Normal = local . source && let None = local . ty && let Some (ty) = self . fcx . typeck_results . borrow () . node_type_opt (local . hir_id) && let Some (vid) = self . fcx . root_vid (ty) && self . reachable_vids . contains (& vid) && local . span . can_be_used_for_suggestions () { return ControlFlow :: Break (errors :: SuggestAnnotation :: Local (local . pat . span . shrink_to_hi () ,)) ; } hir :: intravisit :: walk_local (self , local) } }}}
mkitem!{mkenum!{#[derive (Debug , Copy , Clone)] pub (crate) enum UnsafeUseReason { Call , Method , Path , UnionField , Deref , }}}

macro_rules! compute_unsafe_infer_vars_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_unsafe_infer_vars in module {}", module_path!());
    };
}

mkfn!{
    compute_unsafe_infer_vars_introspect!();
    #[doc = " Finds all type variables which are passed to an `unsafe` operation."] #[doc = ""] #[doc = " For example, for this function `f`:"] #[doc = " ```ignore (demonstrative)"] #[doc = " fn f() {"] #[doc = "     unsafe {"] #[doc = "         let x /* ?X */ = core::mem::zeroed();"] #[doc = "         //               ^^^^^^^^^^^^^^^^^^^ -- hir_id, span, reason"] #[doc = ""] #[doc = "         let y = core::mem::zeroed::<Option<_ /* ?Y */>>();"] #[doc = "         //      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -- hir_id, span, reason"] #[doc = "     }"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " `compute_unsafe_infer_vars` will return `{ id(?X) -> (hir_id, span, Call) }`"] fn compute_unsafe_infer_vars < 'a , 'tcx > (fcx : & 'a FnCtxt < 'a , 'tcx > , body_id : LocalDefId ,) -> UnordMap < ty :: TyVid , (HirId , Span , UnsafeUseReason) > { let body = fcx . tcx . hir_maybe_body_owned_by (body_id) . expect ("body id must have an owner") ; let mut res = UnordMap :: default () ; struct UnsafeInferVarsVisitor < 'a , 'tcx > { fcx : & 'a FnCtxt < 'a , 'tcx > , res : & 'a mut UnordMap < ty :: TyVid , (HirId , Span , UnsafeUseReason) > , } impl Visitor < '_ > for UnsafeInferVarsVisitor < '_ , '_ > { fn visit_expr (& mut self , ex : & '_ hir :: Expr < '_ >) { let typeck_results = self . fcx . typeck_results . borrow () ; match ex . kind { hir :: ExprKind :: MethodCall (..) => { if let Some (def_id) = typeck_results . type_dependent_def_id (ex . hir_id) && let method_ty = self . fcx . tcx . type_of (def_id) . instantiate_identity () && let sig = method_ty . fn_sig (self . fcx . tcx) && sig . safety () . is_unsafe () { let mut collector = InferVarCollector { value : (ex . hir_id , ex . span , UnsafeUseReason :: Method) , res : self . res , } ; typeck_results . node_args (ex . hir_id) . types () . for_each (| t | t . visit_with (& mut collector)) ; } } hir :: ExprKind :: Call (func , ..) => { let func_ty = typeck_results . expr_ty (func) ; if func_ty . is_fn () && let sig = func_ty . fn_sig (self . fcx . tcx) && sig . safety () . is_unsafe () { let mut collector = InferVarCollector { value : (ex . hir_id , ex . span , UnsafeUseReason :: Call) , res : self . res , } ; typeck_results . node_args (func . hir_id) . types () . for_each (| t | t . visit_with (& mut collector)) ; sig . output () . visit_with (& mut collector) ; } } hir :: ExprKind :: Path (_) => { let ty = typeck_results . expr_ty (ex) ; if ty . is_fn () && let sig = ty . fn_sig (self . fcx . tcx) && sig . safety () . is_unsafe () { let mut collector = InferVarCollector { value : (ex . hir_id , ex . span , UnsafeUseReason :: Path) , res : self . res , } ; typeck_results . node_args (ex . hir_id) . types () . for_each (| t | t . visit_with (& mut collector)) ; } } hir :: ExprKind :: Unary (hir :: UnOp :: Deref , pointer) => { if let ty :: RawPtr (pointee , _) = typeck_results . expr_ty (pointer) . kind () { pointee . visit_with (& mut InferVarCollector { value : (ex . hir_id , ex . span , UnsafeUseReason :: Deref) , res : self . res , }) ; } } hir :: ExprKind :: Field (base , _) => { let base_ty = typeck_results . expr_ty (base) ; if base_ty . is_union () { typeck_results . expr_ty (ex) . visit_with (& mut InferVarCollector { value : (ex . hir_id , ex . span , UnsafeUseReason :: UnionField) , res : self . res , }) ; } } _ => () , } ; hir :: intravisit :: walk_expr (self , ex) ; } } struct InferVarCollector < 'r , V > { value : V , res : & 'r mut UnordMap < ty :: TyVid , V > , } impl < 'tcx , V : Copy > ty :: TypeVisitor < TyCtxt < 'tcx > > for InferVarCollector < '_ , V > { fn visit_ty (& mut self , t : Ty < 'tcx >) { if let Some (vid) = t . ty_vid () { _ = self . res . try_insert (vid , self . value) ; } else { t . super_visit_with (self) } } } UnsafeInferVarsVisitor { fcx , res : & mut res } . visit_expr (& body . value) ; debug ! (? res , "collected the following unsafe vars for {body_id:?}") ; res }
}