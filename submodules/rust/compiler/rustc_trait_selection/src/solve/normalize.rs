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
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use rustc_data_structures :: stack :: ensure_sufficient_stack ;}
mkuse!{use rustc_infer :: infer :: InferCtxt ;}
mkuse!{use rustc_infer :: infer :: at :: At ;}
mkuse!{use rustc_infer :: traits :: solve :: Goal ;}
mkuse!{use rustc_infer :: traits :: { FromSolverError , Obligation , TraitEngine } ;}
mkuse!{use rustc_middle :: traits :: ObligationCause ;}
mkuse!{use rustc_middle :: ty :: { self , FallibleTypeFolder , Ty , TyCtxt , TypeFoldable , TypeFolder , TypeSuperFoldable , TypeVisitableExt , UniverseIndex , } ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use super :: { FulfillmentCtxt , NextSolverError } ;}
mkuse!{use crate :: error_reporting :: InferCtxtErrorExt ;}
mkuse!{use crate :: error_reporting :: traits :: OverflowCause ;}
mkuse!{use crate :: traits :: { BoundVarReplacer , PlaceholderReplacer , ScrubbedTraitError } ;}

macro_rules! deeply_normalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function deeply_normalize in module {}", module_path!());
    };
}

mkfn!{
    deeply_normalize_introspect!();
    #[doc = " Deeply normalize all aliases in `value`. This does not handle inference and expects"] #[doc = " its input to be already fully resolved."] pub fn deeply_normalize < 'tcx , T , E > (at : At < '_ , 'tcx > , value : T) -> Result < T , Vec < E > > where T : TypeFoldable < TyCtxt < 'tcx > > , E : FromSolverError < 'tcx , NextSolverError < 'tcx > > , { assert ! (! value . has_escaping_bound_vars ()) ; deeply_normalize_with_skipped_universes (at , value , vec ! []) }
}

macro_rules! deeply_normalize_with_skipped_universes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function deeply_normalize_with_skipped_universes in module {}", module_path!());
    };
}

mkfn!{
    deeply_normalize_with_skipped_universes_introspect!();
    #[doc = " Deeply normalize all aliases in `value`. This does not handle inference and expects"] #[doc = " its input to be already fully resolved."] #[doc = ""] #[doc = " Additionally takes a list of universes which represents the binders which have been"] #[doc = " entered before passing `value` to the function. This is currently needed for"] #[doc = " `normalize_erasing_regions`, which skips binders as it walks through a type."] pub fn deeply_normalize_with_skipped_universes < 'tcx , T , E > (at : At < '_ , 'tcx > , value : T , universes : Vec < Option < UniverseIndex > > ,) -> Result < T , Vec < E > > where T : TypeFoldable < TyCtxt < 'tcx > > , E : FromSolverError < 'tcx , NextSolverError < 'tcx > > , { let (value , coroutine_goals) = deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals (at , value , universes ,) ? ; assert_eq ! (coroutine_goals , vec ! []) ; Ok (value) }
}

macro_rules! deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals in module {}", module_path!());
    };
}

mkfn!{
    deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals_introspect!();
    #[doc = " Deeply normalize all aliases in `value`. This does not handle inference and expects"] #[doc = " its input to be already fully resolved."] #[doc = ""] #[doc = " Additionally takes a list of universes which represents the binders which have been"] #[doc = " entered before passing `value` to the function. This is currently needed for"] #[doc = " `normalize_erasing_regions`, which skips binders as it walks through a type."] #[doc = ""] #[doc = " This returns a set of stalled obligations involving coroutines if the typing mode of"] #[doc = " the underlying infcx has any stalled coroutine def ids."] pub fn deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals < 'tcx , T , E > (at : At < '_ , 'tcx > , value : T , universes : Vec < Option < UniverseIndex > > ,) -> Result < (T , Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > >) , Vec < E > > where T : TypeFoldable < TyCtxt < 'tcx > > , E : FromSolverError < 'tcx , NextSolverError < 'tcx > > , { let fulfill_cx = FulfillmentCtxt :: new (at . infcx) ; let mut folder = NormalizationFolder { at , fulfill_cx , depth : 0 , universes , stalled_coroutine_goals : vec ! [] , } ; let value = value . try_fold_with (& mut folder) ? ; let errors = folder . fulfill_cx . select_all_or_error (at . infcx) ; if errors . is_empty () { Ok ((value , folder . stalled_coroutine_goals)) } else { Err (errors) } }
}
mkitem!{mkstruct!{struct NormalizationFolder < 'me , 'tcx , E > { at : At < 'me , 'tcx > , fulfill_cx : FulfillmentCtxt < 'tcx , E > , depth : usize , universes : Vec < Option < UniverseIndex > > , stalled_coroutine_goals : Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > > , }}}
mkitem!{mkimpl!{impl < 'tcx , E > NormalizationFolder < '_ , 'tcx , E > where E : FromSolverError < 'tcx , NextSolverError < 'tcx > > , { fn normalize_alias_term (& mut self , alias_term : ty :: Term < 'tcx > ,) -> Result < ty :: Term < 'tcx > , Vec < E > > { let infcx = self . at . infcx ; let tcx = infcx . tcx ; let recursion_limit = tcx . recursion_limit () ; if ! recursion_limit . value_within_limit (self . depth) { let term = alias_term . to_alias_term () . unwrap () ; self . at . infcx . err_ctxt () . report_overflow_error (OverflowCause :: DeeplyNormalize (term) , self . at . cause . span , true , | _ | { } ,) ; } self . depth += 1 ; let infer_term = infcx . next_term_var_of_kind (alias_term , self . at . cause . span) ; let obligation = Obligation :: new (tcx , self . at . cause . clone () , self . at . param_env , ty :: PredicateKind :: AliasRelate (alias_term . into () , infer_term . into () , ty :: AliasRelationDirection :: Equate ,) ,) ; self . fulfill_cx . register_predicate_obligation (infcx , obligation) ; self . select_all_and_stall_coroutine_predicates () ? ; let term = infcx . resolve_vars_if_possible (infer_term) ; let result = match term . kind () { ty :: TermKind :: Ty (ty) => ty . try_super_fold_with (self) ? . into () , ty :: TermKind :: Const (ct) => ct . try_super_fold_with (self) ? . into () , } ; self . depth -= 1 ; Ok (result) } fn select_all_and_stall_coroutine_predicates (& mut self) -> Result < () , Vec < E > > { let errors = self . fulfill_cx . select_where_possible (self . at . infcx) ; if ! errors . is_empty () { return Err (errors) ; } self . stalled_coroutine_goals . extend (self . fulfill_cx . drain_stalled_obligations_for_coroutines (self . at . infcx) . into_iter () . map (| obl | obl . as_goal ()) ,) ; let errors = self . fulfill_cx . collect_remaining_errors (self . at . infcx) ; if ! errors . is_empty () { return Err (errors) ; } Ok (()) } }}}
mkitem!{mkimpl!{impl < 'tcx , E > FallibleTypeFolder < TyCtxt < 'tcx > > for NormalizationFolder < '_ , 'tcx , E > where E : FromSolverError < 'tcx , NextSolverError < 'tcx > > + Debug , { type Error = Vec < E > ; fn cx (& self) -> TyCtxt < 'tcx > { self . at . infcx . tcx } fn try_fold_binder < T : TypeFoldable < TyCtxt < 'tcx > > > (& mut self , t : ty :: Binder < 'tcx , T > ,) -> Result < ty :: Binder < 'tcx , T > , Self :: Error > { self . universes . push (None) ; let t = t . try_super_fold_with (self) ? ; self . universes . pop () ; Ok (t) } #[instrument (level = "trace" , skip (self) , ret)] fn try_fold_ty (& mut self , ty : Ty < 'tcx >) -> Result < Ty < 'tcx > , Self :: Error > { let infcx = self . at . infcx ; debug_assert_eq ! (ty , infcx . shallow_resolve (ty)) ; if ! ty . has_aliases () { return Ok (ty) ; } let ty :: Alias (..) = * ty . kind () else { return ty . try_super_fold_with (self) } ; if ty . has_escaping_bound_vars () { let (ty , mapped_regions , mapped_types , mapped_consts) = BoundVarReplacer :: replace_bound_vars (infcx , & mut self . universes , ty) ; let result = ensure_sufficient_stack (| | self . normalize_alias_term (ty . into ())) ? . expect_type () ; Ok (PlaceholderReplacer :: replace_placeholders (infcx , mapped_regions , mapped_types , mapped_consts , & self . universes , result ,)) } else { Ok (ensure_sufficient_stack (| | self . normalize_alias_term (ty . into ())) ? . expect_type ()) } } #[instrument (level = "trace" , skip (self) , ret)] fn try_fold_const (& mut self , ct : ty :: Const < 'tcx >) -> Result < ty :: Const < 'tcx > , Self :: Error > { let infcx = self . at . infcx ; debug_assert_eq ! (ct , infcx . shallow_resolve_const (ct)) ; if ! ct . has_aliases () { return Ok (ct) ; } let ty :: ConstKind :: Unevaluated (..) = ct . kind () else { return ct . try_super_fold_with (self) } ; if ct . has_escaping_bound_vars () { let (ct , mapped_regions , mapped_types , mapped_consts) = BoundVarReplacer :: replace_bound_vars (infcx , & mut self . universes , ct) ; let result = ensure_sufficient_stack (| | self . normalize_alias_term (ct . into ())) ? . expect_const () ; Ok (PlaceholderReplacer :: replace_placeholders (infcx , mapped_regions , mapped_types , mapped_consts , & self . universes , result ,)) } else { Ok (ensure_sufficient_stack (| | self . normalize_alias_term (ct . into ())) ? . expect_const ()) } } }}}

macro_rules! deeply_normalize_for_diagnostics_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function deeply_normalize_for_diagnostics in module {}", module_path!());
    };
}

mkfn!{
    deeply_normalize_for_diagnostics_introspect!();
    pub (crate) fn deeply_normalize_for_diagnostics < 'tcx , T : TypeFoldable < TyCtxt < 'tcx > > > (infcx : & InferCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , t : T ,) -> T { t . fold_with (& mut DeeplyNormalizeForDiagnosticsFolder { at : infcx . at (& ObligationCause :: dummy () , param_env) , }) }
}
mkitem!{mkstruct!{struct DeeplyNormalizeForDiagnosticsFolder < 'a , 'tcx > { at : At < 'a , 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for DeeplyNormalizeForDiagnosticsFolder < '_ , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . at . infcx . tcx } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { let infcx = self . at . infcx ; let result : Result < _ , Vec < ScrubbedTraitError < 'tcx > > > = infcx . commit_if_ok (| _ | { deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals (self . at , ty , vec ! [None ; ty . outer_exclusive_binder () . as_usize ()] ,) }) ; match result { Ok ((ty , _)) => ty , Err (_) => ty . super_fold_with (self) , } } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { let infcx = self . at . infcx ; let result : Result < _ , Vec < ScrubbedTraitError < 'tcx > > > = infcx . commit_if_ok (| _ | { deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals (self . at , ct , vec ! [None ; ct . outer_exclusive_binder () . as_usize ()] ,) }) ; match result { Ok ((ct , _)) => ct , Err (_) => ct . super_fold_with (self) , } } }}}