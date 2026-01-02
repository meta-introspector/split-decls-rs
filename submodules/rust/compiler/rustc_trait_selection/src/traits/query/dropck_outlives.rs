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
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_infer :: traits :: query :: type_op :: DropckOutlives ;}
mkuse!{use rustc_middle :: traits :: query :: { DropckConstraint , DropckOutlivesResult } ;}
mkuse!{use rustc_middle :: ty :: { self , EarlyBinder , ParamEnvAnd , Ty , TyCtxt } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: solve :: NextSolverError ;}
mkuse!{use crate :: traits :: query :: NoSolution ;}
mkuse!{use crate :: traits :: query :: normalize :: QueryNormalizeExt ;}
mkuse!{use crate :: traits :: { FromSolverError , Normalized , ObligationCause , ObligationCtxt } ;}

macro_rules! trivial_dropck_outlives_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trivial_dropck_outlives in module {}", module_path!());
    };
}

mkfn!{
    trivial_dropck_outlives_introspect!();
    #[doc = " This returns true if the type `ty` is \"trivial\" for"] #[doc = " dropck-outlives -- that is, if it doesn't require any types to"] #[doc = " outlive. This is similar but not *quite* the same as the"] #[doc = " `needs_drop` test in the compiler already -- that is, for every"] #[doc = " type T for which this function return true, needs-drop would"] #[doc = " return `false`. But the reverse does not hold: in particular,"] #[doc = " `needs_drop` returns false for `PhantomData`, but it is not"] #[doc = " trivial for dropck-outlives."] #[doc = ""] #[doc = " Note also that `needs_drop` requires a \"global\" type (i.e., one"] #[doc = " with erased regions), but this function does not."] #[doc = ""] pub fn trivial_dropck_outlives < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> bool { match ty . kind () { ty :: Infer (ty :: FreshIntTy (_)) | ty :: Infer (ty :: FreshFloatTy (_)) | ty :: Bool | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Never | ty :: FnDef (..) | ty :: FnPtr (..) | ty :: Char | ty :: CoroutineWitness (..) | ty :: RawPtr (_ , _) | ty :: Ref (..) | ty :: Str | ty :: Foreign (..) | ty :: Error (_) => true , ty :: Pat (ty , _) | ty :: Slice (ty) => trivial_dropck_outlives (tcx , * ty) , ty :: Array (ty , size) => { match size . try_to_target_usize (tcx) { Some (0) => true , _ => trivial_dropck_outlives (tcx , * ty) , } } ty :: Tuple (tys) => tys . iter () . all (| t | trivial_dropck_outlives (tcx , t)) , ty :: Closure (_ , args) => trivial_dropck_outlives (tcx , args . as_closure () . tupled_upvars_ty ()) , ty :: CoroutineClosure (_ , args) => { trivial_dropck_outlives (tcx , args . as_coroutine_closure () . tupled_upvars_ty ()) } ty :: Adt (def , _) => { if def . is_manually_drop () { true } else { false } } ty :: Dynamic (..) | ty :: Alias (..) | ty :: Param (_) | ty :: Placeholder (..) | ty :: Infer (_) | ty :: Bound (..) | ty :: Coroutine (..) | ty :: UnsafeBinder (_) => false , } }
}

macro_rules! compute_dropck_outlives_inner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_dropck_outlives_inner in module {}", module_path!());
    };
}

mkfn!{
    compute_dropck_outlives_inner_introspect!();
    pub fn compute_dropck_outlives_inner < 'tcx > (ocx : & ObligationCtxt < '_ , 'tcx > , goal : ParamEnvAnd < 'tcx , DropckOutlives < 'tcx > > , span : Span ,) -> Result < DropckOutlivesResult < 'tcx > , NoSolution > { match compute_dropck_outlives_with_errors (ocx , goal , span) { Ok (r) => Ok (r) , Err (_) => Err (NoSolution) , } }
}

macro_rules! compute_dropck_outlives_with_errors_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_dropck_outlives_with_errors in module {}", module_path!());
    };
}

mkfn!{
    compute_dropck_outlives_with_errors_introspect!();
    pub fn compute_dropck_outlives_with_errors < 'tcx , E > (ocx : & ObligationCtxt < '_ , 'tcx , E > , goal : ParamEnvAnd < 'tcx , DropckOutlives < 'tcx > > , span : Span ,) -> Result < DropckOutlivesResult < 'tcx > , Vec < E > > where E : FromSolverError < 'tcx , NextSolverError < 'tcx > > , { let tcx = ocx . infcx . tcx ; let ParamEnvAnd { param_env , value : DropckOutlives { dropped_ty } } = goal ; let mut result = DropckOutlivesResult { kinds : vec ! [] , overflows : vec ! [] } ; let mut ty_stack = vec ! [(dropped_ty , 0)] ; let mut ty_set = FxHashSet :: default () ; let cause = ObligationCause :: dummy_with_span (span) ; let mut constraints = DropckConstraint :: empty () ; while let Some ((ty , depth)) = ty_stack . pop () { debug ! ("{} kinds, {} overflows, {} ty_stack" , result . kinds . len () , result . overflows . len () , ty_stack . len ()) ; dtorck_constraint_for_ty_inner (tcx , ocx . infcx . typing_env (param_env) , span , depth , ty , & mut constraints ,) ; result . kinds . append (& mut constraints . outlives) ; result . overflows . append (& mut constraints . overflows) ; if ! result . overflows . is_empty () { break ; } for ty in constraints . dtorck_types . drain (..) { let ty = if let Ok (Normalized { value : ty , obligations }) = ocx . infcx . at (& cause , param_env) . query_normalize (ty) { ocx . register_obligations (obligations) ; debug ! ("dropck_outlives: ty from dtorck_types = {:?}" , ty) ; ty } else { let errors = ocx . select_all_or_error () ; if ! errors . is_empty () { return Err (errors) ; } match ocx . deeply_normalize (& cause , param_env , ty) { Ok (_) => { tcx . dcx () . span_delayed_bug (span , format ! ("query normalize succeeded of {ty}, \
                                but deep normalize failed" ,) ,) ; ty } Err (errors) => return Err (errors) , } } ; match ty . kind () { ty :: Param (..) => { } ty :: Alias (..) => { result . kinds . push (ty . into ()) ; } _ => { if ty_set . insert (ty) { ty_stack . push ((ty , depth + 1)) ; } } } } } debug ! ("dropck_outlives: result = {:#?}" , result) ; Ok (result) }
}

macro_rules! dtorck_constraint_for_ty_inner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dtorck_constraint_for_ty_inner in module {}", module_path!());
    };
}

mkfn!{
    dtorck_constraint_for_ty_inner_introspect!();
    #[doc = " Returns a set of constraints that needs to be satisfied in"] #[doc = " order for `ty` to be valid for destruction."] #[instrument (level = "debug" , skip (tcx , typing_env , span , constraints))] pub fn dtorck_constraint_for_ty_inner < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , span : Span , depth : usize , ty : Ty < 'tcx > , constraints : & mut DropckConstraint < 'tcx > ,) { if ! tcx . recursion_limit () . value_within_limit (depth) { constraints . overflows . push (ty) ; return ; } if trivial_dropck_outlives (tcx , ty) { return ; } match ty . kind () { ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Str | ty :: Never | ty :: Foreign (..) | ty :: RawPtr (..) | ty :: Ref (..) | ty :: FnDef (..) | ty :: FnPtr (..) | ty :: CoroutineWitness (..) => { } ty :: Pat (ety , _) | ty :: Array (ety , _) | ty :: Slice (ety) => { rustc_data_structures :: stack :: ensure_sufficient_stack (| | { dtorck_constraint_for_ty_inner (tcx , typing_env , span , depth + 1 , * ety , constraints) }) ; } ty :: Tuple (tys) => rustc_data_structures :: stack :: ensure_sufficient_stack (| | { for ty in tys . iter () { dtorck_constraint_for_ty_inner (tcx , typing_env , span , depth + 1 , ty , constraints) ; } }) , ty :: Closure (_ , args) => rustc_data_structures :: stack :: ensure_sufficient_stack (| | { for ty in args . as_closure () . upvar_tys () { dtorck_constraint_for_ty_inner (tcx , typing_env , span , depth + 1 , ty , constraints) ; } }) , ty :: CoroutineClosure (_ , args) => { rustc_data_structures :: stack :: ensure_sufficient_stack (| | { for ty in args . as_coroutine_closure () . upvar_tys () { dtorck_constraint_for_ty_inner (tcx , typing_env , span , depth + 1 , ty , constraints ,) ; } }) } ty :: Coroutine (def_id , args) => { let args = args . as_coroutine () ; let typing_env = tcx . erase_and_anonymize_regions (typing_env) ; let needs_drop = tcx . mir_coroutine_witnesses (def_id) . is_some_and (| witness | { witness . field_tys . iter () . any (| field | field . ty . needs_drop (tcx , typing_env)) }) ; if needs_drop { constraints . outlives . extend (args . upvar_tys () . iter () . map (ty :: GenericArg :: from)) ; constraints . outlives . push (args . resume_ty () . into ()) ; } else { for ty in args . upvar_tys () { dtorck_constraint_for_ty_inner (tcx , typing_env , span , depth + 1 , ty , constraints ,) ; } } } ty :: Adt (def , args) => { let DropckConstraint { dtorck_types , outlives , overflows } = tcx . at (span) . adt_dtorck_constraint (def . did ()) ; constraints . dtorck_types . extend (dtorck_types . iter () . map (| t | EarlyBinder :: bind (* t) . instantiate (tcx , args))) ; constraints . outlives . extend (outlives . iter () . map (| t | EarlyBinder :: bind (* t) . instantiate (tcx , args))) ; constraints . overflows . extend (overflows . iter () . map (| t | EarlyBinder :: bind (* t) . instantiate (tcx , args))) ; } ty :: Dynamic (..) => { constraints . outlives . push (ty . into ()) ; } ty :: Alias (..) | ty :: Param (..) => { constraints . dtorck_types . push (ty) ; } ty :: UnsafeBinder (_) => { constraints . dtorck_types . push (ty) ; } ty :: Placeholder (..) | ty :: Bound (..) | ty :: Infer (..) | ty :: Error (_) => { tcx . dcx () . span_delayed_bug (span , format ! ("Unresolved type in dropck: {:?}." , ty)) ; } } }
}