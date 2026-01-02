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
mkuse!{use rustc_hir :: { self as hir , LangItem } ;}
mkuse!{use rustc_infer :: infer :: { BoundRegionConversionTime , DefineOpaqueTypes } ;}
mkuse!{use rustc_infer :: traits :: { ImplDerivedHostCause , ImplSource , Obligation , ObligationCause , ObligationCauseCode , PredicateObligation , } ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: traits :: query :: NoSolution ;}
mkuse!{use rustc_middle :: ty :: elaborate :: elaborate ;}
mkuse!{use rustc_middle :: ty :: fast_reject :: DeepRejectCtxt ;}
mkuse!{use rustc_middle :: ty :: { self , TypingMode } ;}
mkuse!{use thin_vec :: { ThinVec , thin_vec } ;}
mkuse!{use super :: SelectionContext ;}
mkuse!{use super :: normalize :: normalize_with_depth_to ;}
mkitem!{pub type HostEffectObligation < 'tcx > = Obligation < 'tcx , ty :: HostEffectPredicate < 'tcx > > ;}
mkitem!{mkenum!{pub enum EvaluationFailure { Ambiguous , NoSolution , }}}

macro_rules! evaluate_host_effect_obligation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_obligation in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_obligation_introspect!();
    pub fn evaluate_host_effect_obligation < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { if matches ! (selcx . infcx . typing_mode () , TypingMode :: Coherence) { span_bug ! (obligation . cause . span , "should not select host obligation in old solver in intercrate mode") ; } let ref obligation = selcx . infcx . resolve_vars_if_possible (obligation . clone ()) ; if obligation . predicate . self_ty () . is_ty_var () { return Err (EvaluationFailure :: Ambiguous) ; } match evaluate_host_effect_from_bounds (selcx , obligation) { Ok (result) => return Ok (result) , Err (EvaluationFailure :: Ambiguous) => return Err (EvaluationFailure :: Ambiguous) , Err (EvaluationFailure :: NoSolution) => { } } match evaluate_host_effect_from_conditionally_const_item_bounds (selcx , obligation) { Ok (result) => return Ok (result) , Err (EvaluationFailure :: Ambiguous) => return Err (EvaluationFailure :: Ambiguous) , Err (EvaluationFailure :: NoSolution) => { } } match evaluate_host_effect_from_item_bounds (selcx , obligation) { Ok (result) => return Ok (result) , Err (EvaluationFailure :: Ambiguous) => return Err (EvaluationFailure :: Ambiguous) , Err (EvaluationFailure :: NoSolution) => { } } match evaluate_host_effect_from_builtin_impls (selcx , obligation) { Ok (result) => return Ok (result) , Err (EvaluationFailure :: Ambiguous) => return Err (EvaluationFailure :: Ambiguous) , Err (EvaluationFailure :: NoSolution) => { } } match evaluate_host_effect_from_selection_candidate (selcx , obligation) { Ok (result) => return Ok (result) , Err (EvaluationFailure :: Ambiguous) => return Err (EvaluationFailure :: Ambiguous) , Err (EvaluationFailure :: NoSolution) => { } } Err (EvaluationFailure :: NoSolution) }
}

macro_rules! match_candidate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function match_candidate in module {}", module_path!());
    };
}

mkfn!{
    match_candidate_introspect!();
    fn match_candidate < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > , candidate : ty :: Binder < 'tcx , ty :: HostEffectPredicate < 'tcx > > , candidate_is_unnormalized : bool , more_nested : impl FnOnce (& mut SelectionContext < '_ , 'tcx > , & mut ThinVec < PredicateObligation < 'tcx > >) ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , NoSolution > { if ! candidate . skip_binder () . constness . satisfies (obligation . predicate . constness) { return Err (NoSolution) ; } let mut candidate = selcx . infcx . instantiate_binder_with_fresh_vars (obligation . cause . span , BoundRegionConversionTime :: HigherRankedType , candidate ,) ; let mut nested = thin_vec ! [] ; if candidate_is_unnormalized { candidate = normalize_with_depth_to (selcx , obligation . param_env , obligation . cause . clone () , obligation . recursion_depth , candidate , & mut nested ,) ; } nested . extend (selcx . infcx . at (& obligation . cause , obligation . param_env) . eq (DefineOpaqueTypes :: Yes , obligation . predicate . trait_ref , candidate . trait_ref) ? . into_obligations () ,) ; more_nested (selcx , & mut nested) ; Ok (nested) }
}

macro_rules! evaluate_host_effect_from_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_from_bounds in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_from_bounds_introspect!();
    fn evaluate_host_effect_from_bounds < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let infcx = selcx . infcx ; let drcx = DeepRejectCtxt :: relate_rigid_rigid (selcx . tcx ()) ; let mut candidate = None ; for clause in obligation . param_env . caller_bounds () { let bound_clause = clause . kind () ; let ty :: ClauseKind :: HostEffect (data) = bound_clause . skip_binder () else { continue ; } ; let data = bound_clause . rebind (data) ; if data . skip_binder () . trait_ref . def_id != obligation . predicate . trait_ref . def_id { continue ; } if ! drcx . args_may_unify (obligation . predicate . trait_ref . args , data . skip_binder () . trait_ref . args) { continue ; } let is_match = infcx . probe (| _ | match_candidate (selcx , obligation , data , false , | _ , _ | { }) . is_ok ()) ; if is_match { if candidate . is_some () { return Err (EvaluationFailure :: Ambiguous) ; } else { candidate = Some (data) ; } } } if let Some (data) = candidate { Ok (match_candidate (selcx , obligation , data , false , | _ , _ | { }) . expect ("candidate matched before, so it should match again")) } else { Err (EvaluationFailure :: NoSolution) } }
}

macro_rules! evaluate_host_effect_from_conditionally_const_item_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_from_conditionally_const_item_bounds in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_from_conditionally_const_item_bounds_introspect!();
    #[doc = " Assembles constness bounds from `~const` item bounds on alias types, which only"] #[doc = " hold if the `~const` where bounds also hold and the parent trait is `~const`."] fn evaluate_host_effect_from_conditionally_const_item_bounds < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let infcx = selcx . infcx ; let tcx = infcx . tcx ; let drcx = DeepRejectCtxt :: relate_rigid_rigid (selcx . tcx ()) ; let mut candidate = None ; let mut consider_ty = obligation . predicate . self_ty () ; while let ty :: Alias (kind @ (ty :: Projection | ty :: Opaque) , alias_ty) = * consider_ty . kind () { if tcx . is_conditionally_const (alias_ty . def_id) { for clause in elaborate (tcx , tcx . explicit_implied_const_bounds (alias_ty . def_id) . iter_instantiated_copied (tcx , alias_ty . args) . map (| (trait_ref , _) | { trait_ref . to_host_effect_clause (tcx , obligation . predicate . constness) }) ,) { let bound_clause = clause . kind () ; let ty :: ClauseKind :: HostEffect (data) = bound_clause . skip_binder () else { unreachable ! ("should not elaborate non-HostEffect from HostEffect") } ; let data = bound_clause . rebind (data) ; if data . skip_binder () . trait_ref . def_id != obligation . predicate . trait_ref . def_id { continue ; } if ! drcx . args_may_unify (obligation . predicate . trait_ref . args , data . skip_binder () . trait_ref . args ,) { continue ; } let is_match = infcx . probe (| _ | match_candidate (selcx , obligation , data , true , | _ , _ | { }) . is_ok ()) ; if is_match { if candidate . is_some () { return Err (EvaluationFailure :: Ambiguous) ; } else { candidate = Some ((data , alias_ty)) ; } } } } if kind != ty :: Projection { break ; } consider_ty = alias_ty . self_ty () ; } if let Some ((data , alias_ty)) = candidate { Ok (match_candidate (selcx , obligation , data , true , | selcx , nested | { let const_conditions = normalize_with_depth_to (selcx , obligation . param_env , obligation . cause . clone () , obligation . recursion_depth , tcx . const_conditions (alias_ty . def_id) . instantiate (tcx , alias_ty . args) , nested ,) ; nested . extend (const_conditions . into_iter () . map (| (trait_ref , _) | { obligation . with (tcx , trait_ref . to_host_effect_clause (tcx , obligation . predicate . constness)) })) ; }) . expect ("candidate matched before, so it should match again")) } else { Err (EvaluationFailure :: NoSolution) } }
}

macro_rules! evaluate_host_effect_from_item_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_from_item_bounds in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_from_item_bounds_introspect!();
    #[doc = " Assembles constness bounds \"normal\" item bounds on aliases, which may include"] #[doc = " unconditionally `const` bounds that are *not* conditional and thus always hold."] fn evaluate_host_effect_from_item_bounds < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let infcx = selcx . infcx ; let tcx = infcx . tcx ; let drcx = DeepRejectCtxt :: relate_rigid_rigid (selcx . tcx ()) ; let mut candidate = None ; let mut consider_ty = obligation . predicate . self_ty () ; while let ty :: Alias (kind @ (ty :: Projection | ty :: Opaque) , alias_ty) = * consider_ty . kind () { for clause in tcx . item_bounds (alias_ty . def_id) . iter_instantiated (tcx , alias_ty . args) { let bound_clause = clause . kind () ; let ty :: ClauseKind :: HostEffect (data) = bound_clause . skip_binder () else { continue ; } ; let data = bound_clause . rebind (data) ; if data . skip_binder () . trait_ref . def_id != obligation . predicate . trait_ref . def_id { continue ; } if ! drcx . args_may_unify (obligation . predicate . trait_ref . args , data . skip_binder () . trait_ref . args ,) { continue ; } let is_match = infcx . probe (| _ | match_candidate (selcx , obligation , data , true , | _ , _ | { }) . is_ok ()) ; if is_match { if candidate . is_some () { return Err (EvaluationFailure :: Ambiguous) ; } else { candidate = Some (data) ; } } } if kind != ty :: Projection { break ; } consider_ty = alias_ty . self_ty () ; } if let Some (data) = candidate { Ok (match_candidate (selcx , obligation , data , true , | _ , _ | { }) . expect ("candidate matched before, so it should match again")) } else { Err (EvaluationFailure :: NoSolution) } }
}

macro_rules! evaluate_host_effect_from_builtin_impls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_from_builtin_impls in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_from_builtin_impls_introspect!();
    fn evaluate_host_effect_from_builtin_impls < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { match selcx . tcx () . as_lang_item (obligation . predicate . def_id ()) { Some (LangItem :: Destruct) => evaluate_host_effect_for_destruct_goal (selcx , obligation) , Some (LangItem :: Fn | LangItem :: FnMut | LangItem :: FnOnce) => { evaluate_host_effect_for_fn_goal (selcx , obligation) } _ => Err (EvaluationFailure :: NoSolution) , } }
}

macro_rules! evaluate_host_effect_for_destruct_goal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_for_destruct_goal in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_for_destruct_goal_introspect!();
    fn evaluate_host_effect_for_destruct_goal < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let tcx = selcx . tcx () ; let destruct_def_id = tcx . require_lang_item (LangItem :: Destruct , obligation . cause . span) ; let self_ty = obligation . predicate . self_ty () ; let const_conditions = match * self_ty . kind () { ty :: Adt (adt_def , _) if adt_def . is_manually_drop () => thin_vec ! [] , ty :: Adt (adt_def , args) => { let mut const_conditions : ThinVec < _ > = adt_def . all_fields () . map (| field | ty :: TraitRef :: new (tcx , destruct_def_id , [field . ty (tcx , args)])) . collect () ; match adt_def . destructor (tcx) . map (| dtor | tcx . constness (dtor . did)) { Some (hir :: Constness :: NotConst) => return Err (EvaluationFailure :: NoSolution) , Some (hir :: Constness :: Const) => { let drop_def_id = tcx . require_lang_item (LangItem :: Drop , obligation . cause . span) ; let drop_trait_ref = ty :: TraitRef :: new (tcx , drop_def_id , [self_ty]) ; const_conditions . push (drop_trait_ref) ; } None => { } } const_conditions } ty :: Array (ty , _) | ty :: Pat (ty , _) | ty :: Slice (ty) => { thin_vec ! [ty :: TraitRef :: new (tcx , destruct_def_id , [ty])] } ty :: Tuple (tys) => { tys . iter () . map (| field_ty | ty :: TraitRef :: new (tcx , destruct_def_id , [field_ty])) . collect () } ty :: Bool | ty :: Char | ty :: Int (..) | ty :: Uint (..) | ty :: Float (..) | ty :: Str | ty :: RawPtr (..) | ty :: Ref (..) | ty :: FnDef (..) | ty :: FnPtr (..) | ty :: Never | ty :: Infer (ty :: InferTy :: FloatVar (_) | ty :: InferTy :: IntVar (_)) | ty :: Error (_) => thin_vec ! [] , ty :: Closure (_ , _) | ty :: CoroutineClosure (_ , _) | ty :: Coroutine (_ , _) | ty :: CoroutineWitness (_ , _) => return Err (EvaluationFailure :: NoSolution) , ty :: UnsafeBinder (_) => return Err (EvaluationFailure :: NoSolution) , ty :: Dynamic (..) | ty :: Param (_) | ty :: Alias (..) | ty :: Placeholder (_) | ty :: Foreign (_) => { return Err (EvaluationFailure :: NoSolution) ; } ty :: Bound (..) | ty :: Infer (ty :: TyVar (_) | ty :: FreshTy (_) | ty :: FreshIntTy (_) | ty :: FreshFloatTy (_)) => { panic ! ("unexpected type `{self_ty:?}`") } } ; Ok (const_conditions . into_iter () . map (| trait_ref | { obligation . with (tcx , ty :: Binder :: dummy (trait_ref) . to_host_effect_clause (tcx , obligation . predicate . constness) ,) }) . collect ()) }
}

macro_rules! evaluate_host_effect_for_fn_goal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_for_fn_goal in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_for_fn_goal_introspect!();
    fn evaluate_host_effect_for_fn_goal < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let tcx = selcx . tcx () ; let self_ty = obligation . predicate . self_ty () ; let (def , args) = match * self_ty . kind () { ty :: FnDef (def , args) => (def , args) , ty :: FnPtr (..) => return Err (EvaluationFailure :: NoSolution) , ty :: Closure (..) | ty :: CoroutineClosure (_ , _) => { return Err (EvaluationFailure :: NoSolution) ; } _ => return Err (EvaluationFailure :: NoSolution) , } ; match tcx . constness (def) { hir :: Constness :: Const => Ok (tcx . const_conditions (def) . instantiate (tcx , args) . into_iter () . map (| (c , span) | { let code = ObligationCauseCode :: WhereClause (def , span) ; let cause = ObligationCause :: new (obligation . cause . span , obligation . cause . body_id , code) ; Obligation :: new (tcx , cause , obligation . param_env , c . to_host_effect_clause (tcx , obligation . predicate . constness) ,) }) . collect ()) , hir :: Constness :: NotConst => Err (EvaluationFailure :: NoSolution) , } }
}

macro_rules! evaluate_host_effect_from_selection_candidate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function evaluate_host_effect_from_selection_candidate in module {}", module_path!());
    };
}

mkfn!{
    evaluate_host_effect_from_selection_candidate_introspect!();
    fn evaluate_host_effect_from_selection_candidate < 'tcx > (selcx : & mut SelectionContext < '_ , 'tcx > , obligation : & HostEffectObligation < 'tcx > ,) -> Result < ThinVec < PredicateObligation < 'tcx > > , EvaluationFailure > { let tcx = selcx . tcx () ; selcx . infcx . commit_if_ok (| _ | { match selcx . select (& obligation . with (tcx , obligation . predicate . trait_ref)) { Ok (None) => Err (EvaluationFailure :: Ambiguous) , Err (_) => Err (EvaluationFailure :: NoSolution) , Ok (Some (source)) => match source { ImplSource :: UserDefined (impl_) => { if tcx . impl_trait_header (impl_ . impl_def_id) . unwrap () . constness != hir :: Constness :: Const { return Err (EvaluationFailure :: NoSolution) ; } let mut nested = impl_ . nested ; nested . extend (tcx . const_conditions (impl_ . impl_def_id) . instantiate (tcx , impl_ . args) . into_iter () . map (| (trait_ref , span) | { Obligation :: new (tcx , obligation . cause . clone () . derived_host_cause (ty :: Binder :: dummy (obligation . predicate) , | derived | { ObligationCauseCode :: ImplDerivedHost (Box :: new (ImplDerivedHostCause { derived , impl_def_id : impl_ . impl_def_id , span , } ,)) } ,) , obligation . param_env , trait_ref . to_host_effect_clause (tcx , obligation . predicate . constness) ,) }) ,) ; Ok (nested) } _ => Err (EvaluationFailure :: NoSolution) , } , } }) }
}