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
mkuse!{use std :: cmp ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_data_structures :: sorted_map :: SortedMap ;}
mkuse!{use rustc_errors :: { Diag , MultiSpan } ;}
mkuse!{use rustc_hir :: { HirId , ItemLocalId } ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: lint :: builtin :: { self , FORBIDDEN_LINT_GROUPS } ;}
mkuse!{use rustc_session :: lint :: { FutureIncompatibilityReason , Level , Lint , LintExpectationId , LintId } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span , Symbol , kw } ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: ty :: TyCtxt ;}
mkitem!{mkenum!{# [doc = " How a lint level was set."] # [derive (Clone , Copy , PartialEq , Eq , Encodable , Decodable , HashStable , Debug)] pub enum LintLevelSource { # [doc = " Lint is at the default level as declared in rustc."] Default , # [doc = " Lint level was set by an attribute."] Node { name : Symbol , span : Span , # [doc = " RFC 2383 reason"] reason : Option < Symbol > , } , # [doc = " Lint level was set by a command-line flag."] # [doc = " The provided `Level` is the level specified on the command line."] # [doc = " (The actual level may be lower due to `--cap-lints`.)"] CommandLine (Symbol , Level) , }}}
mkitem!{mkimpl!{impl LintLevelSource { pub fn name (& self) -> Symbol { match * self { LintLevelSource :: Default => kw :: Default , LintLevelSource :: Node { name , .. } => name , LintLevelSource :: CommandLine (name , _) => name , } } pub fn span (& self) -> Span { match * self { LintLevelSource :: Default => DUMMY_SP , LintLevelSource :: Node { span , .. } => span , LintLevelSource :: CommandLine (_ , _) => DUMMY_SP , } } }}}
mkitem!{mkstruct!{# [doc = " Convenience helper for moving things around together that frequently are paired"] # [derive (Copy , Clone , Debug , HashStable , Encodable , Decodable)] pub struct LevelAndSource { pub level : Level , pub lint_id : Option < LintExpectationId > , pub src : LintLevelSource , }}}
mkitem!{mkstruct!{# [doc = " Return type for the `shallow_lint_levels_on` query."] # [doc = ""] # [doc = " This map represents the set of allowed lints and allowance levels given"] # [doc = " by the attributes for *a single HirId*."] # [derive (Default , Debug , HashStable)] pub struct ShallowLintLevelMap { pub expectations : Vec < (LintExpectationId , LintExpectation) > , pub specs : SortedMap < ItemLocalId , FxIndexMap < LintId , LevelAndSource > > , }}}

macro_rules! reveal_actual_level_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reveal_actual_level in module {}", module_path!());
    };
}

mkfn!{
    reveal_actual_level_introspect!();
    # [doc = " From an initial level and source, verify the effect of special annotations:"] # [doc = " `warnings` lint level and lint caps."] # [doc = ""] # [doc = " The return of this function is suitable for diagnostics."] pub fn reveal_actual_level (level : Option < (Level , Option < LintExpectationId >) > , src : & mut LintLevelSource , sess : & Session , lint : LintId , probe_for_lint_level : impl FnOnce (LintId ,) -> (Option < (Level , Option < LintExpectationId >) > , LintLevelSource) ,) -> (Level , Option < LintExpectationId >) { let (mut level , mut lint_id) = level . unwrap_or_else (| | (lint . lint . default_level (sess . edition ()) , None)) ; if level == Level :: Warn && lint != LintId :: of (FORBIDDEN_LINT_GROUPS) { let (warnings_level , warnings_src) = probe_for_lint_level (LintId :: of (builtin :: WARNINGS)) ; if let Some ((configured_warning_level , configured_lint_id)) = warnings_level { if configured_warning_level != Level :: Warn { level = configured_warning_level ; lint_id = configured_lint_id ; * src = warnings_src ; } } } level = if let LintLevelSource :: CommandLine (_ , Level :: ForceWarn) = src { level } else { cmp :: min (level , sess . opts . lint_cap . unwrap_or (Level :: Forbid)) } ; if let Some (driver_level) = sess . driver_lint_caps . get (& lint) { level = cmp :: min (* driver_level , level) ; } (level , lint_id) }
}
mkitem!{mkimpl!{impl ShallowLintLevelMap { # [doc = " Perform a deep probe in the HIR tree looking for the actual level for the lint."] # [doc = " This lint level is not usable for diagnostics, it needs to be corrected by"] # [doc = " `reveal_actual_level` beforehand."] # [instrument (level = "trace" , skip (self , tcx) , ret)] fn probe_for_lint_level (& self , tcx : TyCtxt < '_ > , id : LintId , start : HirId ,) -> (Option < (Level , Option < LintExpectationId >) > , LintLevelSource) { if let Some (map) = self . specs . get (& start . local_id) && let Some (& LevelAndSource { level , lint_id , src }) = map . get (& id) { return (Some ((level , lint_id)) , src) ; } let mut owner = start . owner ; let mut specs = & self . specs ; for parent in tcx . hir_parent_id_iter (start) { if parent . owner != owner { owner = parent . owner ; specs = & tcx . shallow_lint_levels_on (owner) . specs ; } if let Some (map) = specs . get (& parent . local_id) && let Some (& LevelAndSource { level , lint_id , src }) = map . get (& id) { return (Some ((level , lint_id)) , src) ; } } (None , LintLevelSource :: Default) } # [doc = " Fetch and return the user-visible lint level for the given lint at the given HirId."] # [instrument (level = "trace" , skip (self , tcx) , ret)] pub fn lint_level_id_at_node (& self , tcx : TyCtxt < '_ > , lint : LintId , cur : HirId ,) -> LevelAndSource { let (level , mut src) = self . probe_for_lint_level (tcx , lint , cur) ; let (level , lint_id) = reveal_actual_level (level , & mut src , tcx . sess , lint , | lint | { self . probe_for_lint_level (tcx , lint , cur) }) ; LevelAndSource { level , lint_id , src } } }}}
mkitem!{mkimpl!{impl TyCtxt < '_ > { # [doc = " Fetch and return the user-visible lint level for the given lint at the given HirId."] pub fn lint_level_at_node (self , lint : & 'static Lint , id : HirId) -> LevelAndSource { self . shallow_lint_levels_on (id . owner) . lint_level_id_at_node (self , LintId :: of (lint) , id) } }}}
mkitem!{mkstruct!{# [doc = " This struct represents a lint expectation and holds all required information"] # [doc = " to emit the `unfulfilled_lint_expectations` lint if it is unfulfilled after"] # [doc = " the `LateLintPass` has completed."] # [derive (Clone , Debug , Encodable , Decodable , HashStable)] pub struct LintExpectation { # [doc = " The reason for this expectation that can optionally be added as part of"] # [doc = " the attribute. It will be displayed as part of the lint message."] pub reason : Option < Symbol > , # [doc = " The [`Span`] of the attribute that this expectation originated from."] pub emission_span : Span , # [doc = " Lint messages for the `unfulfilled_lint_expectations` lint will be"] # [doc = " adjusted to include an additional note. Therefore, we have to track if"] # [doc = " the expectation is for the lint."] pub is_unfulfilled_lint_expectations : bool , # [doc = " This will hold the name of the tool that this lint belongs to. For"] # [doc = " the lint `clippy::some_lint` the tool would be `clippy`, the same"] # [doc = " goes for `rustdoc`. This will be `None` for rustc lints"] pub lint_tool : Option < Symbol > , }}}
mkitem!{mkimpl!{impl LintExpectation { pub fn new (reason : Option < Symbol > , emission_span : Span , is_unfulfilled_lint_expectations : bool , lint_tool : Option < Symbol > ,) -> Self { Self { reason , emission_span , is_unfulfilled_lint_expectations , lint_tool } } }}}

macro_rules! explain_lint_level_source_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function explain_lint_level_source in module {}", module_path!());
    };
}

mkfn!{
    explain_lint_level_source_introspect!();
    fn explain_lint_level_source (sess : & Session , lint : & 'static Lint , level : Level , src : LintLevelSource , err : & mut Diag < '_ , () > ,) { let lint_group_name = | lint | { let lint_groups_iter = sess . lint_groups_iter () ; let lint_id = LintId :: of (lint) ; lint_groups_iter . filter (| lint_group | ! lint_group . is_externally_loaded) . find (| lint_group | { lint_group . lints . iter () . find (| lint_group_lint | * * lint_group_lint == lint_id) . is_some () }) . map (| lint_group | lint_group . name) } ; let name = lint . name_lower () ; if let Level :: Allow = level { return ; } match src { LintLevelSource :: Default => { let level_str = level . as_str () ; match lint_group_name (lint) { Some (group_name) => { err . note_once (format ! ("`#[{level_str}({name})]` (part of `#[{level_str}({group_name})]`) on by default")) ; } None => { err . note_once (format ! ("`#[{level_str}({name})]` on by default")) ; } } } LintLevelSource :: CommandLine (lint_flag_val , orig_level) => { let flag = orig_level . to_cmd_flag () ; let hyphen_case_lint_name = name . replace ('_' , "-") ; if lint_flag_val . as_str () == name { err . note_once (format ! ("requested on the command line with `{flag} {hyphen_case_lint_name}`")) ; } else { let hyphen_case_flag_val = lint_flag_val . as_str () . replace ('_' , "-") ; err . note_once (format ! ("`{flag} {hyphen_case_lint_name}` implied by `{flag} {hyphen_case_flag_val}`")) ; if matches ! (orig_level , Level :: Warn | Level :: Deny) { err . help_once (format ! ("to override `{flag} {hyphen_case_flag_val}` add `#[allow({name})]`")) ; } } } LintLevelSource :: Node { name : lint_attr_name , span , reason , .. } => { if let Some (rationale) = reason { err . note (rationale . to_string ()) ; } err . span_note_once (span , "the lint level is defined here") ; if lint_attr_name . as_str () != name { let level_str = level . as_str () ; err . note_once (format ! ("`#[{level_str}({name})]` implied by `#[{level_str}({lint_attr_name})]`")) ; } } } }
}

macro_rules! lint_level_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lint_level in module {}", module_path!());
    };
}

mkfn!{
    lint_level_introspect!();
    # [doc = " The innermost function for emitting lints."] # [doc = ""] # [doc = " If you are looking to implement a lint, look for higher level functions,"] # [doc = " for example:"] # [doc = " - [`TyCtxt::emit_node_span_lint`]"] # [doc = " - [`TyCtxt::node_span_lint`]"] # [doc = " - [`TyCtxt::emit_node_lint`]"] # [doc = " - [`TyCtxt::node_lint`]"] # [doc = " - `LintContext::opt_span_lint`"] # [doc = ""] # [doc = " ## `decorate`"] # [doc = ""] # [doc = " It is not intended to call `emit`/`cancel` on the `Diag` passed in the `decorate` callback."] # [track_caller] pub fn lint_level (sess : & Session , lint : & 'static Lint , level : LevelAndSource , span : Option < MultiSpan > , decorate : impl for < 'a , 'b > FnOnce (& 'b mut Diag < 'a , () >) ,) { # [track_caller] fn lint_level_impl (sess : & Session , lint : & 'static Lint , level : LevelAndSource , span : Option < MultiSpan > , decorate : Box < dyn '_ + for < 'a , 'b > FnOnce (& 'b mut Diag < 'a , () >) > ,) { let LevelAndSource { level , lint_id , src } = level ; let future_incompatible = lint . future_incompatible ; let has_future_breakage = future_incompatible . map_or (sess . opts . unstable_opts . future_incompat_test && lint . default_level != Level :: Allow , | incompat | incompat . report_in_deps ,) ; let err_level = match level { Level :: Allow => { if has_future_breakage { rustc_errors :: Level :: Allow } else { return ; } } Level :: Expect => { rustc_errors :: Level :: Expect } Level :: ForceWarn => rustc_errors :: Level :: ForceWarning , Level :: Warn => rustc_errors :: Level :: Warning , Level :: Deny | Level :: Forbid => rustc_errors :: Level :: Error , } ; let mut err = Diag :: new (sess . dcx () , err_level , "") ; if let Some (span) = span { err . span (span) ; } if let Some (lint_id) = lint_id { err . lint_id (lint_id) ; } if err . span . primary_spans () . iter () . any (| s | s . in_external_macro (sess . source_map ())) { err . disable_suggestions () ; let incompatible = future_incompatible . is_some_and (| f | f . reason . edition () . is_none ()) ; if ! incompatible && ! lint . report_in_external_macro { err . cancel () ; return ; } } err . is_lint (lint . name_lower () , has_future_breakage) ; if let Level :: Expect = level { decorate (& mut err) ; err . emit () ; return ; } if let Some (future_incompatible) = future_incompatible { let explanation = match future_incompatible . reason { FutureIncompatibilityReason :: FutureReleaseError => { "this was previously accepted by the compiler but is being phased out; \
                         it will become a hard error in a future release!" . to_owned () } FutureIncompatibilityReason :: FutureReleaseSemanticsChange => { "this will change its meaning in a future release!" . to_owned () } FutureIncompatibilityReason :: EditionError (edition) => { let current_edition = sess . edition () ; format ! ("this is accepted in the current edition (Rust {current_edition}) but is a hard error in Rust {edition}!") } FutureIncompatibilityReason :: EditionSemanticsChange (edition) => { format ! ("this changes meaning in Rust {edition}") } FutureIncompatibilityReason :: EditionAndFutureReleaseError (edition) => { format ! ("this was previously accepted by the compiler but is being phased out; \
                         it will become a hard error in Rust {edition} and in a future release in all editions!") } FutureIncompatibilityReason :: EditionAndFutureReleaseSemanticsChange (edition) => { format ! ("this changes meaning in Rust {edition} and in a future release in all editions!") } FutureIncompatibilityReason :: Custom (reason) => reason . to_owned () , } ; if future_incompatible . explain_reason { err . warn (explanation) ; } if ! future_incompatible . reference . is_empty () { let citation = format ! ("for more information, see {}" , future_incompatible . reference) ; err . note (citation) ; } } let skip = err_level == rustc_errors :: Level :: Warning && ! sess . dcx () . can_emit_warnings () ; if ! skip { decorate (& mut err) ; } explain_lint_level_source (sess , lint , level , src , & mut err) ; err . emit () } lint_level_impl (sess , lint , level , span , Box :: new (decorate)) }
}