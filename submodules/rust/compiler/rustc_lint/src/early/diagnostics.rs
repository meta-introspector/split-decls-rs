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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use rustc_ast :: util :: unicode :: TEXT_FLOW_CONTROL_CHARS ;}
mkuse!{use rustc_errors :: { Applicability , Diag , DiagArgValue , LintDiagnostic , elided_lifetime_in_path_suggestion , } ;}
mkuse!{use rustc_middle :: middle :: stability ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: lint :: BuiltinLintDiag ;}
mkuse!{use rustc_span :: BytePos ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: lints ;}
mkmod!{check_cfg, { 
                getname!(check_cfg);
                getsrc!(check_cfg);
                getpath!(check_cfg);
                get_deps!(check_cfg);
                get_crates!(check_cfg);
                mkinclude!(check_cfg);
                 
            }}

macro_rules! decorate_builtin_lint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decorate_builtin_lint in module {}", module_path!());
    };
}

mkfn!{
    decorate_builtin_lint_introspect!();
    pub fn decorate_builtin_lint (sess : & Session , tcx : Option < TyCtxt < '_ > > , diagnostic : BuiltinLintDiag , diag : & mut Diag < '_ , () > ,) { match diagnostic { BuiltinLintDiag :: UnicodeTextFlow (comment_span , content) => { let spans : Vec < _ > = content . char_indices () . filter_map (| (i , c) | { TEXT_FLOW_CONTROL_CHARS . contains (& c) . then (| | { let lo = comment_span . lo () + BytePos (2 + i as u32) ; (c , comment_span . with_lo (lo) . with_hi (lo + BytePos (c . len_utf8 () as u32))) }) }) . collect () ; let characters = spans . iter () . map (| & (c , span) | lints :: UnicodeCharNoteSub { span , c_debug : format ! ("{c:?}") }) . collect () ; let suggestions = (! spans . is_empty ()) . then_some (lints :: UnicodeTextFlowSuggestion { spans : spans . iter () . map (| (_c , span) | * span) . collect () , }) ; lints :: UnicodeTextFlow { comment_span , characters , suggestions , num_codepoints : spans . len () , } . decorate_lint (diag) ; } BuiltinLintDiag :: AbsPathWithModule (mod_span) => { let (replacement , applicability) = match sess . source_map () . span_to_snippet (mod_span) { Ok (ref s) => { let opt_colon = if s . trim_start () . starts_with ("::") { "" } else { "::" } ; (format ! ("crate{opt_colon}{s}") , Applicability :: MachineApplicable) } Err (_) => ("crate::<path>" . to_string () , Applicability :: HasPlaceholders) , } ; lints :: AbsPathWithModule { sugg : lints :: AbsPathWithModuleSugg { span : mod_span , applicability , replacement } , } . decorate_lint (diag) ; } BuiltinLintDiag :: ProcMacroDeriveResolutionFallback { span : macro_span , ns_descr , ident , } => lints :: ProcMacroDeriveResolutionFallback { span : macro_span , ns_descr , ident } . decorate_lint (diag) , BuiltinLintDiag :: MacroExpandedMacroExportsAccessedByAbsolutePaths (span_def) => { lints :: MacroExpandedMacroExportsAccessedByAbsolutePaths { definition : span_def } . decorate_lint (diag) } BuiltinLintDiag :: ElidedLifetimesInPaths (n , path_span , incl_angl_brckt , insertion_span) => { lints :: ElidedLifetimesInPaths { subdiag : elided_lifetime_in_path_suggestion (sess . source_map () , n , path_span , incl_angl_brckt , insertion_span ,) , } . decorate_lint (diag) ; } BuiltinLintDiag :: UnknownCrateTypes { span , candidate } => { let sugg = candidate . map (| candidate | lints :: UnknownCrateTypesSub { span , candidate }) ; lints :: UnknownCrateTypes { sugg } . decorate_lint (diag) ; } BuiltinLintDiag :: UnusedImports { remove_whole_use , num_to_remove , remove_spans , test_module_span , span_snippets , } => { let sugg = if remove_whole_use { lints :: UnusedImportsSugg :: RemoveWholeUse { span : remove_spans [0] } } else { lints :: UnusedImportsSugg :: RemoveImports { remove_spans , num_to_remove } } ; let test_module_span = test_module_span . map (| span | sess . source_map () . guess_head_span (span)) ; lints :: UnusedImports { sugg , test_module_span , num_snippets : span_snippets . len () , span_snippets : DiagArgValue :: StrListSepByAnd (span_snippets . into_iter () . map (Cow :: Owned) . collect () ,) , } . decorate_lint (diag) ; } BuiltinLintDiag :: RedundantImport (spans , ident) => { let subs = spans . into_iter () . map (| (span , is_imported) | { (match (span . is_dummy () , is_imported) { (false , true) => lints :: RedundantImportSub :: ImportedHere , (false , false) => lints :: RedundantImportSub :: DefinedHere , (true , true) => lints :: RedundantImportSub :: ImportedPrelude , (true , false) => lints :: RedundantImportSub :: DefinedPrelude , }) (span) }) . collect () ; lints :: RedundantImport { subs , ident } . decorate_lint (diag) ; } BuiltinLintDiag :: DeprecatedMacro { suggestion , suggestion_span , note , path , since_kind , } => { let sub = suggestion . map (| suggestion | stability :: DeprecationSuggestion { span : suggestion_span , kind : "macro" . to_owned () , suggestion , }) ; stability :: Deprecated { sub , kind : "macro" . to_owned () , path , note , since_kind } . decorate_lint (diag) ; } BuiltinLintDiag :: UnusedDocComment (attr_span) => { lints :: UnusedDocComment { span : attr_span } . decorate_lint (diag) ; } BuiltinLintDiag :: PatternsInFnsWithoutBody { span : remove_span , ident , is_foreign } => { let sub = lints :: PatternsInFnsWithoutBodySub { ident , span : remove_span } ; if is_foreign { lints :: PatternsInFnsWithoutBody :: Foreign { sub } } else { lints :: PatternsInFnsWithoutBody :: Bodiless { sub } } . decorate_lint (diag) ; } BuiltinLintDiag :: LegacyDeriveHelpers (label_span) => { lints :: LegacyDeriveHelpers { span : label_span } . decorate_lint (diag) ; } BuiltinLintDiag :: OrPatternsBackCompat (suggestion_span , suggestion) => { lints :: OrPatternsBackCompat { span : suggestion_span , suggestion } . decorate_lint (diag) ; } BuiltinLintDiag :: ReservedPrefix (label_span , prefix) => { lints :: ReservedPrefix { label : label_span , suggestion : label_span . shrink_to_hi () , prefix , } . decorate_lint (diag) ; } BuiltinLintDiag :: RawPrefix (label_span) => { lints :: RawPrefix { label : label_span , suggestion : label_span . shrink_to_hi () } . decorate_lint (diag) ; } BuiltinLintDiag :: ReservedString { is_string , suggestion } => { if is_string { lints :: ReservedString { suggestion } . decorate_lint (diag) ; } else { lints :: ReservedMultihash { suggestion } . decorate_lint (diag) ; } } BuiltinLintDiag :: UnusedBuiltinAttribute { attr_name , macro_name , invoc_span , attr_span , } => { lints :: UnusedBuiltinAttribute { invoc_span , attr_name , macro_name , attr_span } . decorate_lint (diag) ; } BuiltinLintDiag :: TrailingMacro (is_trailing , name) => { lints :: TrailingMacro { is_trailing , name } . decorate_lint (diag) ; } BuiltinLintDiag :: BreakWithLabelAndLoop (sugg_span) => { lints :: BreakWithLabelAndLoop { sub : lints :: BreakWithLabelAndLoopSub { left : sugg_span . shrink_to_lo () , right : sugg_span . shrink_to_hi () , } , } . decorate_lint (diag) ; } BuiltinLintDiag :: UnexpectedCfgName (name , value) => { check_cfg :: unexpected_cfg_name (sess , tcx , name , value) . decorate_lint (diag) ; } BuiltinLintDiag :: UnexpectedCfgValue (name , value) => { check_cfg :: unexpected_cfg_value (sess , tcx , name , value) . decorate_lint (diag) ; } BuiltinLintDiag :: DeprecatedWhereclauseLocation (left_sp , sugg) => { let suggestion = match sugg { Some ((right_sp , sugg)) => lints :: DeprecatedWhereClauseLocationSugg :: MoveToEnd { left : left_sp , right : right_sp , sugg , } , None => lints :: DeprecatedWhereClauseLocationSugg :: RemoveWhere { span : left_sp } , } ; lints :: DeprecatedWhereClauseLocation { suggestion } . decorate_lint (diag) ; } BuiltinLintDiag :: MissingUnsafeOnExtern { suggestion } => { lints :: MissingUnsafeOnExtern { suggestion } . decorate_lint (diag) ; } BuiltinLintDiag :: SingleUseLifetime { param_span , use_span : Some ((use_span , elide)) , deletion_span , ident , } => { debug ! (? param_span , ? use_span , ? deletion_span) ; let suggestion = if let Some (deletion_span) = deletion_span { let (use_span , replace_lt) = if elide { let use_span = sess . source_map () . span_extend_while_whitespace (use_span) ; (use_span , String :: new ()) } else { (use_span , "'_" . to_owned ()) } ; debug ! (? deletion_span , ? use_span) ; let deletion_span = if deletion_span . is_empty () { None } else { Some (deletion_span) } ; Some (lints :: SingleUseLifetimeSugg { deletion_span , use_span , replace_lt }) } else { None } ; lints :: SingleUseLifetime { suggestion , param_span , use_span , ident } . decorate_lint (diag) ; } BuiltinLintDiag :: SingleUseLifetime { use_span : None , deletion_span , ident , .. } => { debug ! (? deletion_span) ; lints :: UnusedLifetime { deletion_span , ident } . decorate_lint (diag) ; } BuiltinLintDiag :: NamedArgumentUsedPositionally { position_sp_to_replace , position_sp_for_msg , named_arg_sp , named_arg_name , is_formatting_arg , } => { let (suggestion , name) = if let Some (positional_arg_to_replace) = position_sp_to_replace { let mut name = named_arg_name . clone () ; if is_formatting_arg { name . push ('$') } ; let span_to_replace = if let Ok (positional_arg_content) = sess . source_map () . span_to_snippet (positional_arg_to_replace) && positional_arg_content . starts_with (':') { positional_arg_to_replace . shrink_to_lo () } else { positional_arg_to_replace } ; (Some (span_to_replace) , name) } else { (None , String :: new ()) } ; lints :: NamedArgumentUsedPositionally { named_arg_sp , position_label_sp : position_sp_for_msg , suggestion , name , named_arg_name , } . decorate_lint (diag) ; } BuiltinLintDiag :: ByteSliceInPackedStructWithDerive { ty } => { lints :: ByteSliceInPackedStructWithDerive { ty } . decorate_lint (diag) ; } BuiltinLintDiag :: UnusedExternCrate { span , removal_span } => { lints :: UnusedExternCrate { span , removal_span } . decorate_lint (diag) ; } BuiltinLintDiag :: ExternCrateNotIdiomatic { vis_span , ident_span } => { let suggestion_span = vis_span . between (ident_span) ; let code = if vis_span . is_empty () { "use " } else { " use " } ; lints :: ExternCrateNotIdiomatic { span : suggestion_span , code } . decorate_lint (diag) ; } BuiltinLintDiag :: AmbiguousGlobImports { diag : ambiguity } => { lints :: AmbiguousGlobImports { ambiguity } . decorate_lint (diag) ; } BuiltinLintDiag :: AmbiguousGlobReexports { name , namespace , first_reexport_span , duplicate_reexport_span , } => { lints :: AmbiguousGlobReexports { first_reexport : first_reexport_span , duplicate_reexport : duplicate_reexport_span , name , namespace , } . decorate_lint (diag) ; } BuiltinLintDiag :: HiddenGlobReexports { name , namespace , glob_reexport_span , private_item_span , } => { lints :: HiddenGlobReexports { glob_reexport : glob_reexport_span , private_item : private_item_span , name , namespace , } . decorate_lint (diag) ; } BuiltinLintDiag :: ReexportPrivateDependency { name , kind , krate } => { lints :: ReexportPrivateDependency { name , kind , krate } . decorate_lint (diag) ; } BuiltinLintDiag :: UnusedQualifications { removal_span } => { lints :: UnusedQualifications { removal_span } . decorate_lint (diag) ; } BuiltinLintDiag :: UnsafeAttrOutsideUnsafe { attribute_name_span , sugg_spans : (left , right) , } => { lints :: UnsafeAttrOutsideUnsafe { span : attribute_name_span , suggestion : lints :: UnsafeAttrOutsideUnsafeSuggestion { left , right } , } . decorate_lint (diag) ; } BuiltinLintDiag :: AssociatedConstElidedLifetime { elided , span : lt_span , lifetimes_in_scope , } => { let lt_span = if elided { lt_span . shrink_to_hi () } else { lt_span } ; let code = if elided { "'static " } else { "'static" } ; lints :: AssociatedConstElidedLifetime { span : lt_span , code , elided , lifetimes_in_scope , } . decorate_lint (diag) ; } BuiltinLintDiag :: RedundantImportVisibility { max_vis , span : vis_span , import_vis } => { lints :: RedundantImportVisibility { span : vis_span , help : () , max_vis , import_vis } . decorate_lint (diag) ; } BuiltinLintDiag :: UnknownDiagnosticAttribute { span : typo_span , typo_name } => { let typo = typo_name . map (| typo_name | lints :: UnknownDiagnosticAttributeTypoSugg { span : typo_span , typo_name , }) ; lints :: UnknownDiagnosticAttribute { typo } . decorate_lint (diag) ; } BuiltinLintDiag :: MacroUseDeprecated => { lints :: MacroUseDeprecated . decorate_lint (diag) ; } BuiltinLintDiag :: UnusedMacroUse => lints :: UnusedMacroUse . decorate_lint (diag) , BuiltinLintDiag :: PrivateExternCrateReexport { source : ident , extern_crate_span } => { lints :: PrivateExternCrateReexport { ident , sugg : extern_crate_span . shrink_to_lo () } . decorate_lint (diag) ; } BuiltinLintDiag :: UnusedLabel => lints :: UnusedLabel . decorate_lint (diag) , BuiltinLintDiag :: MacroIsPrivate (ident) => { lints :: MacroIsPrivate { ident } . decorate_lint (diag) ; } BuiltinLintDiag :: UnusedMacroDefinition (name) => { lints :: UnusedMacroDefinition { name } . decorate_lint (diag) ; } BuiltinLintDiag :: MacroRuleNeverUsed (n , name) => { lints :: MacroRuleNeverUsed { n : n + 1 , name } . decorate_lint (diag) ; } BuiltinLintDiag :: UnstableFeature (msg) => { lints :: UnstableFeature { msg } . decorate_lint (diag) ; } BuiltinLintDiag :: AvoidUsingIntelSyntax => { lints :: AvoidIntelSyntax . decorate_lint (diag) ; } BuiltinLintDiag :: AvoidUsingAttSyntax => { lints :: AvoidAttSyntax . decorate_lint (diag) ; } BuiltinLintDiag :: IncompleteInclude => { lints :: IncompleteInclude . decorate_lint (diag) ; } BuiltinLintDiag :: UnnameableTestItems => { lints :: UnnameableTestItems . decorate_lint (diag) ; } BuiltinLintDiag :: DuplicateMacroAttribute => { lints :: DuplicateMacroAttribute . decorate_lint (diag) ; } BuiltinLintDiag :: CfgAttrNoAttributes => { lints :: CfgAttrNoAttributes . decorate_lint (diag) ; } BuiltinLintDiag :: MetaVariableStillRepeating (name) => { lints :: MetaVariableStillRepeating { name } . decorate_lint (diag) ; } BuiltinLintDiag :: MetaVariableWrongOperator => { lints :: MetaVariableWrongOperator . decorate_lint (diag) ; } BuiltinLintDiag :: DuplicateMatcherBinding => { lints :: DuplicateMatcherBinding . decorate_lint (diag) ; } BuiltinLintDiag :: UnknownMacroVariable (name) => { lints :: UnknownMacroVariable { name } . decorate_lint (diag) ; } BuiltinLintDiag :: UnusedCrateDependency { extern_crate , local_crate } => { lints :: UnusedCrateDependency { extern_crate , local_crate } . decorate_lint (diag) } BuiltinLintDiag :: IllFormedAttributeInput { suggestions , docs } => { lints :: IllFormedAttributeInput { num_suggestions : suggestions . len () , suggestions : DiagArgValue :: StrListSepByAnd (suggestions . into_iter () . map (| s | format ! ("`{s}`") . into ()) . collect () ,) , has_docs : docs . is_some () , docs : docs . unwrap_or ("") , } . decorate_lint (diag) } BuiltinLintDiag :: OutOfScopeMacroCalls { span , path , location } => { lints :: OutOfScopeMacroCalls { span , path , location } . decorate_lint (diag) } } }
}