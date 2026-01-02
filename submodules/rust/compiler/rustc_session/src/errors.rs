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
mkuse!{use std :: num :: { NonZero , ParseIntError } ;}
mkuse!{use rustc_ast :: token ;}
mkuse!{use rustc_ast :: util :: literal :: LitError ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Diag , DiagCtxtHandle , DiagMessage , Diagnostic , EmissionGuarantee , ErrorGuaranteed , Level , MultiSpan , } ;}
mkuse!{use rustc_macros :: { Diagnostic , LintDiagnostic , Subdiagnostic } ;}
mkuse!{use rustc_span :: { Span , Symbol } ;}
mkuse!{use rustc_target :: spec :: { SplitDebuginfo , StackProtector , TargetTuple } ;}
mkuse!{use crate :: config :: CrateType ;}
mkuse!{use crate :: parse :: ParseSess ;}
mkitem!{mkenum!{# [derive (Diagnostic)] pub (crate) enum AppleDeploymentTarget { # [diag (session_apple_deployment_target_invalid)] Invalid { env_var : & 'static str , error : ParseIntError } , # [diag (session_apple_deployment_target_too_low)] TooLow { env_var : & 'static str , version : String , os_min : String } , }}}
mkitem!{mkstruct!{pub (crate) struct FeatureGateError { pub (crate) span : MultiSpan , pub (crate) explain : DiagMessage , }}}
mkitem!{mkimpl!{impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for FeatureGateError { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { Diag :: new (dcx , level , self . explain) . with_span (self . span) . with_code (E0658) } }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (session_feature_diagnostic_for_issue)] pub (crate) struct FeatureDiagnosticForIssue { pub (crate) n : NonZero < u32 > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (session_feature_suggest_upgrade_compiler)] pub (crate) struct SuggestUpgradeCompiler { date : & 'static str , }}}
mkitem!{mkimpl!{impl SuggestUpgradeCompiler { pub (crate) fn ui_testing () -> Self { Self { date : "YYYY-MM-DD" } } pub (crate) fn new () -> Option < Self > { let date = option_env ! ("CFG_VER_DATE") ? ; Some (Self { date }) } }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [help (session_feature_diagnostic_help)] pub (crate) struct FeatureDiagnosticHelp { pub (crate) feature : Symbol , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (session_feature_diagnostic_suggestion , applicability = "maybe-incorrect" , code = "#![feature({feature})]\n")] pub struct FeatureDiagnosticSuggestion { pub feature : Symbol , # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [help (session_cli_feature_diagnostic_help)] pub (crate) struct CliFeatureDiagnosticHelp { pub (crate) feature : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_not_circumvent_feature)] pub (crate) struct NotCircumventFeature ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_linker_plugin_lto_windows_not_supported)] pub (crate) struct LinkerPluginToWindowsNotSupported ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_profile_use_file_does_not_exist)] pub (crate) struct ProfileUseFileDoesNotExist < 'a > { pub (crate) path : & 'a std :: path :: Path , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_profile_sample_use_file_does_not_exist)] pub (crate) struct ProfileSampleUseFileDoesNotExist < 'a > { pub (crate) path : & 'a std :: path :: Path , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_target_requires_unwind_tables)] pub (crate) struct TargetRequiresUnwindTables ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_instrumentation_not_supported)] pub (crate) struct InstrumentationNotSupported { pub (crate) us : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_sanitizer_not_supported)] pub (crate) struct SanitizerNotSupported { pub (crate) us : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_sanitizers_not_supported)] pub (crate) struct SanitizersNotSupported { pub (crate) us : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_cannot_mix_and_match_sanitizers)] pub (crate) struct CannotMixAndMatchSanitizers { pub (crate) first : String , pub (crate) second : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_cannot_enable_crt_static_linux)] pub (crate) struct CannotEnableCrtStaticLinux ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_sanitizer_cfi_requires_lto)] pub (crate) struct SanitizerCfiRequiresLto ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_sanitizer_cfi_requires_single_codegen_unit)] pub (crate) struct SanitizerCfiRequiresSingleCodegenUnit ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_sanitizer_cfi_canonical_jump_tables_requires_cfi)] pub (crate) struct SanitizerCfiCanonicalJumpTablesRequiresCfi ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_sanitizer_cfi_generalize_pointers_requires_cfi)] pub (crate) struct SanitizerCfiGeneralizePointersRequiresCfi ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_sanitizer_cfi_normalize_integers_requires_cfi)] pub (crate) struct SanitizerCfiNormalizeIntegersRequiresCfi ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_sanitizer_kcfi_arity_requires_kcfi)] pub (crate) struct SanitizerKcfiArityRequiresKcfi ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_sanitizer_kcfi_requires_panic_abort)] pub (crate) struct SanitizerKcfiRequiresPanicAbort ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_split_lto_unit_requires_lto)] pub (crate) struct SplitLtoUnitRequiresLto ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_unstable_virtual_function_elimination)] pub (crate) struct UnstableVirtualFunctionElimination ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_unsupported_dwarf_version)] # [help (session_unsupported_dwarf_version_help)] pub (crate) struct UnsupportedDwarfVersion { pub (crate) dwarf_version : u32 , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_embed_source_insufficient_dwarf_version)] pub (crate) struct EmbedSourceInsufficientDwarfVersion { pub (crate) dwarf_version : u32 , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_embed_source_requires_debug_info)] pub (crate) struct EmbedSourceRequiresDebugInfo ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_target_stack_protector_not_supported)] pub (crate) struct StackProtectorNotSupportedForTarget < 'a > { pub (crate) stack_protector : StackProtector , pub (crate) target_triple : & 'a TargetTuple , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_target_small_data_threshold_not_supported)] pub (crate) struct SmallDataThresholdNotSupportedForTarget < 'a > { pub (crate) target_triple : & 'a TargetTuple , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_branch_protection_requires_aarch64)] pub (crate) struct BranchProtectionRequiresAArch64 ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_split_debuginfo_unstable_platform)] pub (crate) struct SplitDebugInfoUnstablePlatform { pub (crate) debuginfo : SplitDebuginfo , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_file_is_not_writeable)] pub (crate) struct FileIsNotWriteable < 'a > { pub (crate) file : & 'a std :: path :: Path , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_file_write_fail)] pub (crate) struct FileWriteFail < 'a > { pub (crate) path : & 'a std :: path :: Path , pub (crate) err : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_crate_name_empty)] pub (crate) struct CrateNameEmpty { # [primary_span] pub (crate) span : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_invalid_character_in_crate_name)] pub (crate) struct InvalidCharacterInCrateName { # [primary_span] pub (crate) span : Option < Span > , pub (crate) character : char , pub (crate) crate_name : Symbol , # [help] pub (crate) help : Option < () > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (session_expr_parentheses_needed , applicability = "machine-applicable")] pub struct ExprParenthesesNeeded { # [suggestion_part (code = "(")] left : Span , # [suggestion_part (code = ")")] right : Span , }}}
mkitem!{mkimpl!{impl ExprParenthesesNeeded { pub fn surrounding (s : Span) -> Self { ExprParenthesesNeeded { left : s . shrink_to_lo () , right : s . shrink_to_hi () } } }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_skipping_const_checks)] pub (crate) struct SkippingConstChecks { # [subdiagnostic] pub (crate) unleashed_features : Vec < UnleashedFeatureHelp > , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum UnleashedFeatureHelp { # [help (session_unleashed_feature_help_named)] Named { # [primary_span] span : Span , gate : Symbol , } , # [help (session_unleashed_feature_help_unnamed)] Unnamed { # [primary_span] span : Span , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_invalid_literal_suffix)] struct InvalidLiteralSuffix < 'a > { # [primary_span] # [label] span : Span , kind : & 'a str , suffix : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_invalid_int_literal_width)] # [help] struct InvalidIntLiteralWidth { # [primary_span] span : Span , width : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_invalid_num_literal_base_prefix)] # [note] struct InvalidNumLiteralBasePrefix { # [primary_span] # [suggestion (applicability = "maybe-incorrect" , code = "{fixed}")] span : Span , fixed : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_invalid_num_literal_suffix)] # [help] struct InvalidNumLiteralSuffix { # [primary_span] # [label] span : Span , suffix : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_invalid_float_literal_width)] # [help] struct InvalidFloatLiteralWidth { # [primary_span] span : Span , width : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_invalid_float_literal_suffix)] # [help] struct InvalidFloatLiteralSuffix { # [primary_span] # [label] span : Span , suffix : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_int_literal_too_large)] # [note] struct IntLiteralTooLarge { # [primary_span] span : Span , limit : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_hexadecimal_float_literal_not_supported)] struct HexadecimalFloatLiteralNotSupported { # [primary_span] # [label (session_not_supported)] span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_octal_float_literal_not_supported)] struct OctalFloatLiteralNotSupported { # [primary_span] # [label (session_not_supported)] span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_binary_float_literal_not_supported)] struct BinaryFloatLiteralNotSupported { # [primary_span] # [label (session_not_supported)] span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_unsupported_crate_type_for_target)] pub (crate) struct UnsupportedCrateTypeForTarget < 'a > { pub (crate) crate_type : CrateType , pub (crate) target_triple : & 'a TargetTuple , }}}

macro_rules! report_lit_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_lit_error in module {}", module_path!());
    };
}

mkfn!{
    report_lit_error_introspect!();
    pub fn report_lit_error (psess : & ParseSess , err : LitError , lit : token :: Lit , span : Span ,) -> ErrorGuaranteed { create_lit_error (psess , err , lit , span) . emit () }
}

macro_rules! create_lit_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_lit_error in module {}", module_path!());
    };
}

mkfn!{
    create_lit_error_introspect!();
    pub fn create_lit_error (psess : & ParseSess , err : LitError , lit : token :: Lit , span : Span) -> Diag < '_ > { fn looks_like_width_suffix (first_chars : & [char] , s : & str) -> bool { s . len () > 1 && s . starts_with (first_chars) && s [1 ..] . chars () . all (| c | c . is_ascii_digit ()) } fn fix_base_capitalisation (prefix : & str , suffix : & str) -> Option < String > { let mut chars = suffix . chars () ; let base_char = chars . next () . unwrap () ; let base = match base_char { 'B' => 2 , 'O' => 8 , 'X' => 16 , _ => return None , } ; let valid = prefix == "0" && chars . filter (| c | * c != '_') . take_while (| c | * c != 'i' && * c != 'u') . all (| c | c . to_digit (base) . is_some ()) ; valid . then (| | format ! ("0{}{}" , base_char . to_ascii_lowercase () , & suffix [1 ..])) } let dcx = psess . dcx () ; match err { LitError :: InvalidSuffix (suffix) => { dcx . create_err (InvalidLiteralSuffix { span , kind : lit . kind . descr () , suffix }) } LitError :: InvalidIntSuffix (suffix) => { let suf = suffix . as_str () ; if looks_like_width_suffix (& ['i' , 'u'] , suf) { dcx . create_err (InvalidIntLiteralWidth { span , width : suf [1 ..] . into () }) } else if let Some (fixed) = fix_base_capitalisation (lit . symbol . as_str () , suf) { dcx . create_err (InvalidNumLiteralBasePrefix { span , fixed }) } else { dcx . create_err (InvalidNumLiteralSuffix { span , suffix : suf . to_string () }) } } LitError :: InvalidFloatSuffix (suffix) => { let suf = suffix . as_str () ; if looks_like_width_suffix (& ['f'] , suf) { dcx . create_err (InvalidFloatLiteralWidth { span , width : suf [1 ..] . to_string () }) } else { dcx . create_err (InvalidFloatLiteralSuffix { span , suffix : suf . to_string () }) } } LitError :: NonDecimalFloat (base) => match base { 16 => dcx . create_err (HexadecimalFloatLiteralNotSupported { span }) , 8 => dcx . create_err (OctalFloatLiteralNotSupported { span }) , 2 => dcx . create_err (BinaryFloatLiteralNotSupported { span }) , _ => unreachable ! () , } , LitError :: IntTooLarge (base) => { let max = u128 :: MAX ; let limit = match base { 2 => format ! ("{max:#b}") , 8 => format ! ("{max:#o}") , 16 => format ! ("{max:#x}") , _ => format ! ("{max}") , } ; dcx . create_err (IntLiteralTooLarge { span , limit }) } } }
}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_incompatible_linker_flavor)] # [note] pub (crate) struct IncompatibleLinkerFlavor { pub (crate) flavor : & 'static str , pub (crate) compatible_list : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_function_return_requires_x86_or_x86_64)] pub (crate) struct FunctionReturnRequiresX86OrX8664 ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_function_return_thunk_extern_requires_non_large_code_model)] pub (crate) struct FunctionReturnThunkExternRequiresNonLargeCodeModel ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_indirect_branch_cs_prefix_requires_x86_or_x86_64)] pub (crate) struct IndirectBranchCsPrefixRequiresX86OrX8664 ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_unsupported_regparm)] pub (crate) struct UnsupportedRegparm { pub (crate) regparm : u32 , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_unsupported_regparm_arch)] pub (crate) struct UnsupportedRegparmArch ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_unsupported_reg_struct_return_arch)] pub (crate) struct UnsupportedRegStructReturnArch ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_failed_to_create_profiler)] pub (crate) struct FailedToCreateProfiler { pub (crate) err : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_soft_float_ignored)] # [note] pub (crate) struct SoftFloatIgnored ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (session_soft_float_deprecated)] # [note] # [note (session_soft_float_deprecated_issue)] pub (crate) struct SoftFloatDeprecated ;}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (session_unexpected_builtin_cfg)] # [note (session_controlled_by)] # [note (session_incoherent)] pub (crate) struct UnexpectedBuiltinCfg { pub (crate) cfg : String , pub (crate) cfg_name : Symbol , pub (crate) controlled_by : & 'static str , }}}