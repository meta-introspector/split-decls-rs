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
mkuse!{use std :: sync :: atomic :: Ordering :: Relaxed ;}
mkuse!{use either :: { Left , Right } ;}
mkuse!{use rustc_abi :: { self as abi , BackendRepr } ;}
mkuse!{use rustc_errors :: E0080 ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_middle :: mir :: interpret :: { AllocId , ErrorHandled , InterpErrorInfo , ReportedErrorInfo } ;}
mkuse!{use rustc_middle :: mir :: { self , ConstAlloc , ConstValue } ;}
mkuse!{use rustc_middle :: query :: TyCtxtAt ;}
mkuse!{use rustc_middle :: ty :: layout :: HasTypingEnv ;}
mkuse!{use rustc_middle :: ty :: print :: with_no_trimmed_paths ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_middle :: { bug , throw_inval } ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use tracing :: { debug , instrument , trace } ;}
mkuse!{use super :: { CanAccessMutGlobal , CompileTimeInterpCx , CompileTimeMachine } ;}
mkuse!{use crate :: const_eval :: CheckAlignment ;}
mkuse!{use crate :: interpret :: { CtfeValidationMode , GlobalId , Immediate , InternError , InternKind , InterpCx , InterpErrorKind , InterpResult , MPlaceTy , MemoryKind , OpTy , RefTracking , ReturnContinuation , create_static_alloc , intern_const_alloc_recursive , interp_ok , throw_exhaust , } ;}
mkuse!{use crate :: { CTRL_C_RECEIVED , errors } ;}

macro_rules! eval_body_using_ecx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eval_body_using_ecx in module {}", module_path!());
    };
}

mkfn!{
    eval_body_using_ecx_introspect!();
    #[instrument (level = "trace" , skip (ecx , body))] fn eval_body_using_ecx < 'tcx , R : InterpretationResult < 'tcx > > (ecx : & mut CompileTimeInterpCx < 'tcx > , cid : GlobalId < 'tcx > , body : & 'tcx mir :: Body < 'tcx > ,) -> InterpResult < 'tcx , R > { let tcx = * ecx . tcx ; assert ! (cid . promoted . is_some () || matches ! (ecx . tcx . def_kind (cid . instance . def_id ()) , DefKind :: Const | DefKind :: Static { .. } | DefKind :: ConstParam | DefKind :: AnonConst | DefKind :: InlineConst | DefKind :: AssocConst) , "Unexpected DefKind: {:?}" , ecx . tcx . def_kind (cid . instance . def_id ())) ; let layout = ecx . layout_of (body . bound_return_ty () . instantiate (tcx , cid . instance . args)) ? ; assert ! (layout . is_sized ()) ; let intern_kind = if cid . promoted . is_some () { InternKind :: Promoted } else { match tcx . static_mutability (cid . instance . def_id ()) { Some (m) => InternKind :: Static (m) , None => InternKind :: Constant , } } ; let ret = if let InternKind :: Static (_) = intern_kind { create_static_alloc (ecx , cid . instance . def_id () . expect_local () , layout) ? } else { ecx . allocate (layout , MemoryKind :: Stack) ? } ; trace ! ("eval_body_using_ecx: pushing stack frame for global: {}{}" , with_no_trimmed_paths ! (ecx . tcx . def_path_str (cid . instance . def_id ())) , cid . promoted . map_or_else (String :: new , | p | format ! ("::{p:?}"))) ; ecx . push_stack_frame_raw (cid . instance , body , & ret . clone () . into () , ReturnContinuation :: Stop { cleanup : false } ,) ? ; ecx . storage_live_for_always_live_locals () ? ; while ecx . step () ? { if CTRL_C_RECEIVED . load (Relaxed) { throw_exhaust ! (Interrupted) ; } } let intern_result = intern_const_alloc_recursive (ecx , intern_kind , & ret) ; const_validate_mplace (ecx , & ret , cid) ? ; match intern_result { Ok (()) => { } Err (InternError :: DanglingPointer) => { throw_inval ! (AlreadyReported (ReportedErrorInfo :: non_const_eval_error (ecx . tcx . dcx () . emit_err (errors :: DanglingPtrInFinal { span : ecx . tcx . span , kind : intern_kind }) ,))) ; } Err (InternError :: BadMutablePointer) => { throw_inval ! (AlreadyReported (ReportedErrorInfo :: non_const_eval_error (ecx . tcx . dcx () . emit_err (errors :: MutablePtrInFinal { span : ecx . tcx . span , kind : intern_kind }) ,))) ; } Err (InternError :: ConstAllocNotGlobal) => { throw_inval ! (AlreadyReported (ReportedErrorInfo :: non_const_eval_error (ecx . tcx . dcx () . emit_err (errors :: ConstHeapPtrInFinal { span : ecx . tcx . span }) ,))) ; } Err (InternError :: PartialPointer) => { throw_inval ! (AlreadyReported (ReportedErrorInfo :: non_const_eval_error (ecx . tcx . dcx () . emit_err (errors :: PartialPtrInFinal { span : ecx . tcx . span , kind : intern_kind }) ,))) ; } } interp_ok (R :: make_result (ret , ecx)) }
}

macro_rules! mk_eval_cx_to_read_const_val_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mk_eval_cx_to_read_const_val in module {}", module_path!());
    };
}

mkfn!{
    mk_eval_cx_to_read_const_val_introspect!();
    #[doc = " The `InterpCx` is only meant to be used to do field and index projections into constants for"] #[doc = " `simd_shuffle` and const patterns in match arms."] #[doc = ""] #[doc = " This should *not* be used to do any actual interpretation. In particular, alignment checks are"] #[doc = " turned off!"] #[doc = ""] #[doc = " The function containing the `match` that is currently being analyzed may have generic bounds"] #[doc = " that inform us about the generic bounds of the constant. E.g., using an associated constant"] #[doc = " of a function's generic parameter will require knowledge about the bounds on the generic"] #[doc = " parameter. These bounds are passed to `mk_eval_cx` via the `ParamEnv` argument."] pub (crate) fn mk_eval_cx_to_read_const_val < 'tcx > (tcx : TyCtxt < 'tcx > , root_span : Span , typing_env : ty :: TypingEnv < 'tcx > , can_access_mut_global : CanAccessMutGlobal ,) -> CompileTimeInterpCx < 'tcx > { debug ! ("mk_eval_cx: {:?}" , typing_env) ; InterpCx :: new (tcx , root_span , typing_env , CompileTimeMachine :: new (can_access_mut_global , CheckAlignment :: No) ,) }
}

macro_rules! mk_eval_cx_for_const_val_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mk_eval_cx_for_const_val in module {}", module_path!());
    };
}

mkfn!{
    mk_eval_cx_for_const_val_introspect!();
    #[doc = " Create an interpreter context to inspect the given `ConstValue`."] #[doc = " Returns both the context and an `OpTy` that represents the constant."] pub fn mk_eval_cx_for_const_val < 'tcx > (tcx : TyCtxtAt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , val : mir :: ConstValue , ty : Ty < 'tcx > ,) -> Option < (CompileTimeInterpCx < 'tcx > , OpTy < 'tcx >) > { let ecx = mk_eval_cx_to_read_const_val (tcx . tcx , tcx . span , typing_env , CanAccessMutGlobal :: No) ; let op = ecx . const_val_to_op (val , ty , None) . discard_err () ? ; Some ((ecx , op)) }
}

macro_rules! op_to_const_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function op_to_const in module {}", module_path!());
    };
}

mkfn!{
    op_to_const_introspect!();
    #[doc = " This function converts an interpreter value into a MIR constant."] #[doc = ""] #[doc = " The `for_diagnostics` flag turns the usual rules for returning `ConstValue::Scalar` into a"] #[doc = " best-effort attempt. This is not okay for use in const-eval sine it breaks invariants rustc"] #[doc = " relies on, but it is okay for diagnostics which will just give up gracefully when they"] #[doc = " encounter an `Indirect` they cannot handle."] #[instrument (skip (ecx) , level = "debug")] pub (super) fn op_to_const < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , op : & OpTy < 'tcx > , for_diagnostics : bool ,) -> ConstValue { if op . layout . is_zst () { return ConstValue :: ZeroSized ; } let force_as_immediate = match op . layout . backend_repr { BackendRepr :: Scalar (abi :: Scalar :: Initialized { .. }) => true , _ => false , } ; let immediate = if force_as_immediate { match ecx . read_immediate (op) . report_err () { Ok (imm) => Right (imm) , Err (err) => { if for_diagnostics { op . as_mplace_or_imm () } else { panic ! ("normalization works on validated constants: {err:?}") } } } } else { op . as_mplace_or_imm () } ; debug ! (? immediate) ; match immediate { Left (ref mplace) => { let (prov , offset) = mplace . ptr () . into_pointer_or_addr () . unwrap () . prov_and_relative_offset () ; let alloc_id = prov . alloc_id () ; ConstValue :: Indirect { alloc_id , offset } } Right (imm) => match * imm { Immediate :: Scalar (x) => ConstValue :: Scalar (x) , Immediate :: ScalarPair (a , b) => { debug ! ("ScalarPair(a: {:?}, b: {:?})" , a , b) ; let pointee_ty = imm . layout . ty . builtin_deref (false) . unwrap () ; debug_assert ! (matches ! (ecx . tcx . struct_tail_for_codegen (pointee_ty , ecx . typing_env ()) . kind () , ty :: Str | ty :: Slice (..) ,) , "`ConstValue::Slice` is for slice-tailed types only, but got {}" , imm . layout . ty ,) ; let msg = "`op_to_const` on an immediate scalar pair must only be used on slice references to the beginning of an actual allocation" ; let ptr = a . to_pointer (ecx) . expect (msg) ; let (prov , offset) = ptr . into_pointer_or_addr () . expect (msg) . prov_and_relative_offset () ; let alloc_id = prov . alloc_id () ; assert ! (offset == abi :: Size :: ZERO , "{}" , msg) ; let meta = b . to_target_usize (ecx) . expect (msg) ; ConstValue :: Slice { alloc_id , meta } } Immediate :: Uninit => bug ! ("`Uninit` is not a valid value for {}" , op . layout . ty) , } , } }
}

macro_rules! turn_into_const_value_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function turn_into_const_value in module {}", module_path!());
    };
}

mkfn!{
    turn_into_const_value_introspect!();
    #[instrument (skip (tcx) , level = "debug" , ret)] pub (crate) fn turn_into_const_value < 'tcx > (tcx : TyCtxt < 'tcx > , constant : ConstAlloc < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , GlobalId < 'tcx > > ,) -> ConstValue { let cid = key . value ; let def_id = cid . instance . def . def_id () ; let is_static = tcx . is_static (def_id) ; let ecx = mk_eval_cx_to_read_const_val (tcx , tcx . def_span (key . value . instance . def_id ()) , key . typing_env , CanAccessMutGlobal :: from (is_static) ,) ; let mplace = ecx . raw_const_to_mplace (constant) . expect ("can only fail if layout computation failed, \
        which should have given a good error before ever invoking this function" ,) ; assert ! (! is_static || cid . promoted . is_some () , "the `eval_to_const_value_raw` query should not be used for statics, use `eval_to_allocation` instead") ; op_to_const (& ecx , & mplace . into () , false) }
}

macro_rules! eval_to_const_value_raw_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eval_to_const_value_raw_provider in module {}", module_path!());
    };
}

mkfn!{
    eval_to_const_value_raw_provider_introspect!();
    #[instrument (skip (tcx) , level = "debug")] pub fn eval_to_const_value_raw_provider < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , GlobalId < 'tcx > > ,) -> :: rustc_middle :: mir :: interpret :: EvalToConstValueResult < 'tcx > { tcx . eval_to_allocation_raw (key) . map (| val | turn_into_const_value (tcx , val , key)) }
}

macro_rules! eval_static_initializer_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eval_static_initializer_provider in module {}", module_path!());
    };
}

mkfn!{
    eval_static_initializer_provider_introspect!();
    #[instrument (skip (tcx) , level = "debug")] pub fn eval_static_initializer_provider < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> :: rustc_middle :: mir :: interpret :: EvalStaticInitializerRawResult < 'tcx > { assert ! (tcx . is_static (def_id . to_def_id ())) ; let instance = ty :: Instance :: mono (tcx , def_id . to_def_id ()) ; let cid = rustc_middle :: mir :: interpret :: GlobalId { instance , promoted : None } ; eval_in_interpreter (tcx , cid , ty :: TypingEnv :: fully_monomorphized ()) }
}
mkitem!{mktrait!{pub trait InterpretationResult < 'tcx > { #[doc = " This function takes the place where the result of the evaluation is stored"] #[doc = " and prepares it for returning it in the appropriate format needed by the specific"] #[doc = " evaluation query."] fn make_result (mplace : MPlaceTy < 'tcx > , ecx : & mut InterpCx < 'tcx , CompileTimeMachine < 'tcx > > ,) -> Self ; }}}
mkitem!{mkimpl!{impl < 'tcx > InterpretationResult < 'tcx > for ConstAlloc < 'tcx > { fn make_result (mplace : MPlaceTy < 'tcx > , _ecx : & mut InterpCx < 'tcx , CompileTimeMachine < 'tcx > > ,) -> Self { ConstAlloc { alloc_id : mplace . ptr () . provenance . unwrap () . alloc_id () , ty : mplace . layout . ty } } }}}

macro_rules! eval_to_allocation_raw_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eval_to_allocation_raw_provider in module {}", module_path!());
    };
}

mkfn!{
    eval_to_allocation_raw_provider_introspect!();
    #[instrument (skip (tcx) , level = "debug")] pub fn eval_to_allocation_raw_provider < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , GlobalId < 'tcx > > ,) -> :: rustc_middle :: mir :: interpret :: EvalToAllocationRawResult < 'tcx > { assert ! (key . value . promoted . is_some () || ! tcx . is_static (key . value . instance . def_id ())) ; debug_assert_eq ! (key . typing_env . typing_mode , ty :: TypingMode :: PostAnalysis) ; if cfg ! (debug_assertions) { let instance = with_no_trimmed_paths ! (key . value . instance . to_string ()) ; trace ! ("const eval: {:?} ({})" , key , instance) ; } eval_in_interpreter (tcx , key . value , key . typing_env) }
}

macro_rules! eval_in_interpreter_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eval_in_interpreter in module {}", module_path!());
    };
}

mkfn!{
    eval_in_interpreter_introspect!();
    fn eval_in_interpreter < 'tcx , R : InterpretationResult < 'tcx > > (tcx : TyCtxt < 'tcx > , cid : GlobalId < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> Result < R , ErrorHandled > { let def = cid . instance . def . def_id () ; let is_static = tcx . is_static (def) ; let mut ecx = InterpCx :: new (tcx , tcx . def_span (def) , typing_env , CompileTimeMachine :: new (CanAccessMutGlobal :: from (is_static) , CheckAlignment :: Error) ,) ; let res = ecx . load_mir (cid . instance . def , cid . promoted) ; res . and_then (| body | eval_body_using_ecx (& mut ecx , cid , body)) . report_err () . map_err (| error | report_eval_error (& ecx , cid , error)) }
}

macro_rules! const_validate_mplace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function const_validate_mplace in module {}", module_path!());
    };
}

mkfn!{
    const_validate_mplace_introspect!();
    #[inline (always)] fn const_validate_mplace < 'tcx > (ecx : & mut InterpCx < 'tcx , CompileTimeMachine < 'tcx > > , mplace : & MPlaceTy < 'tcx > , cid : GlobalId < 'tcx > ,) -> Result < () , ErrorHandled > { let alloc_id = mplace . ptr () . provenance . unwrap () . alloc_id () ; let mut ref_tracking = RefTracking :: new (mplace . clone ()) ; let mut inner = false ; while let Some ((mplace , path)) = ref_tracking . next () { let mode = match ecx . tcx . static_mutability (cid . instance . def_id ()) { _ if cid . promoted . is_some () => CtfeValidationMode :: Promoted , Some (mutbl) => CtfeValidationMode :: Static { mutbl } , None => { CtfeValidationMode :: Const { allow_immutable_unsafe_cell : ! inner } } } ; ecx . const_validate_operand (& mplace . into () , path , & mut ref_tracking , mode) . report_err () . map_err (| error | report_validation_error (& ecx , cid , error , alloc_id)) ? ; inner = true ; } Ok (()) }
}

macro_rules! report_eval_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_eval_error in module {}", module_path!());
    };
}

mkfn!{
    report_eval_error_introspect!();
    #[inline (never)] fn report_eval_error < 'tcx > (ecx : & InterpCx < 'tcx , CompileTimeMachine < 'tcx > > , cid : GlobalId < 'tcx > , error : InterpErrorInfo < 'tcx > ,) -> ErrorHandled { let (error , backtrace) = error . into_parts () ; backtrace . print_backtrace () ; let instance = with_no_trimmed_paths ! (cid . instance . to_string ()) ; super :: report (ecx , error , DUMMY_SP , | | super :: get_span_and_frames (ecx . tcx , ecx . stack ()) , | diag , span , frames | { let num_frames = frames . len () ; diag . code (E0080) ; diag . span_label (span , crate :: fluent_generated :: const_eval_error) ; for frame in frames { diag . subdiagnostic (frame) ; } diag . arg ("instance" , instance) ; diag . arg ("num_frames" , num_frames) ; } ,) }
}

macro_rules! report_validation_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_validation_error in module {}", module_path!());
    };
}

mkfn!{
    report_validation_error_introspect!();
    #[inline (never)] fn report_validation_error < 'tcx > (ecx : & InterpCx < 'tcx , CompileTimeMachine < 'tcx > > , cid : GlobalId < 'tcx > , error : InterpErrorInfo < 'tcx > , alloc_id : AllocId ,) -> ErrorHandled { if ! matches ! (error . kind () , InterpErrorKind :: UndefinedBehavior (_)) { return report_eval_error (ecx , cid , error) ; } let (error , backtrace) = error . into_parts () ; backtrace . print_backtrace () ; let bytes = ecx . print_alloc_bytes_for_diagnostics (alloc_id) ; let info = ecx . get_alloc_info (alloc_id) ; let raw_bytes = errors :: RawBytesNote { size : info . size . bytes () , align : info . align . bytes () , bytes } ; crate :: const_eval :: report (ecx , error , DUMMY_SP , | | crate :: const_eval :: get_span_and_frames (ecx . tcx , ecx . stack ()) , move | diag , span , frames | { diag . code (E0080) ; diag . span_label (span , crate :: fluent_generated :: const_eval_validation_failure) ; diag . note (crate :: fluent_generated :: const_eval_validation_failure_note) ; for frame in frames { diag . subdiagnostic (frame) ; } diag . subdiagnostic (raw_bytes) ; } ,) }
}