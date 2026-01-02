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
mkuse!{use rustc_hir :: def_id :: LOCAL_CRATE ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: ExpectedValues ;}
mkuse!{use rustc_span :: edit_distance :: find_best_match_for_name ;}
mkuse!{use rustc_span :: { ExpnKind , Ident , Span , Symbol , sym } ;}
mkuse!{use crate :: lints ;}
mkitem!{const MAX_CHECK_CFG_NAMES_OR_VALUES : usize = 35 ;}
mkitem!{mkenum!{enum FilterWellKnownNames { Yes , No , }}}

macro_rules! sort_and_truncate_possibilities_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sort_and_truncate_possibilities in module {}", module_path!());
    };
}

mkfn!{
    sort_and_truncate_possibilities_introspect!();
    fn sort_and_truncate_possibilities (sess : & Session , mut possibilities : Vec < Symbol > , filter_well_known_names : FilterWellKnownNames ,) -> (Vec < Symbol > , usize) { let possibilities_len = possibilities . len () ; let n_possibilities = if sess . opts . unstable_opts . check_cfg_all_expected { possibilities . len () } else { match filter_well_known_names { FilterWellKnownNames :: Yes => { possibilities . retain (| cfg_name | { ! sess . psess . check_config . well_known_names . contains (cfg_name) }) ; } FilterWellKnownNames :: No => { } } ; std :: cmp :: min (possibilities . len () , MAX_CHECK_CFG_NAMES_OR_VALUES) } ; possibilities . sort_by (| s1 , s2 | s1 . as_str () . cmp (s2 . as_str ())) ; let and_more = possibilities_len . saturating_sub (n_possibilities) ; possibilities . truncate (n_possibilities) ; (possibilities , and_more) }
}
mkitem!{mkenum!{enum EscapeQuotes { Yes , No , }}}

macro_rules! to_check_cfg_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_check_cfg_arg in module {}", module_path!());
    };
}

mkfn!{
    to_check_cfg_arg_introspect!();
    fn to_check_cfg_arg (name : Ident , value : Option < Symbol > , quotes : EscapeQuotes) -> String { if let Some (value) = value { let value = str :: escape_debug (value . as_str ()) . to_string () ; let values = match quotes { EscapeQuotes :: Yes => format ! ("\\\"{}\\\"" , value . replace ("\"" , "\\\\\\\\\"")) , EscapeQuotes :: No => format ! ("\"{value}\"") , } ; format ! ("cfg({name}, values({values}))") } else { format ! ("cfg({name})") } }
}

macro_rules! cargo_help_sub_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cargo_help_sub in module {}", module_path!());
    };
}

mkfn!{
    cargo_help_sub_introspect!();
    fn cargo_help_sub (sess : & Session , inst : & impl Fn (EscapeQuotes) -> String ,) -> lints :: UnexpectedCfgCargoHelp { let unescaped = & inst (EscapeQuotes :: No) ; if matches ! (& sess . opts . crate_name , Some (crate_name) if crate_name == "build_script_build") { lints :: UnexpectedCfgCargoHelp :: lint_cfg (unescaped) } else { lints :: UnexpectedCfgCargoHelp :: lint_cfg_and_build_rs (unescaped , & inst (EscapeQuotes :: Yes)) } }
}

macro_rules! rustc_macro_help_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rustc_macro_help in module {}", module_path!());
    };
}

mkfn!{
    rustc_macro_help_introspect!();
    fn rustc_macro_help (span : Span) -> Option < lints :: UnexpectedCfgRustcMacroHelp > { let oexpn = span . ctxt () . outer_expn_data () ; if let Some (def_id) = oexpn . macro_def_id && let ExpnKind :: Macro (macro_kind , macro_name) = oexpn . kind && def_id . krate != LOCAL_CRATE { Some (lints :: UnexpectedCfgRustcMacroHelp { macro_kind : macro_kind . descr () , macro_name }) } else { None } }
}

macro_rules! cargo_macro_help_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cargo_macro_help in module {}", module_path!());
    };
}

mkfn!{
    cargo_macro_help_introspect!();
    fn cargo_macro_help (tcx : Option < TyCtxt < '_ > > , span : Span ,) -> Option < lints :: UnexpectedCfgCargoMacroHelp > { let oexpn = span . ctxt () . outer_expn_data () ; if let Some (def_id) = oexpn . macro_def_id && let ExpnKind :: Macro (macro_kind , macro_name) = oexpn . kind && def_id . krate != LOCAL_CRATE && let Some (tcx) = tcx { Some (lints :: UnexpectedCfgCargoMacroHelp { macro_kind : macro_kind . descr () , macro_name , crate_name : tcx . crate_name (def_id . krate) , }) } else { None } }
}

macro_rules! unexpected_cfg_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unexpected_cfg_name in module {}", module_path!());
    };
}

mkfn!{
    unexpected_cfg_name_introspect!();
    pub (super) fn unexpected_cfg_name (sess : & Session , tcx : Option < TyCtxt < '_ > > , (name , name_span) : (Symbol , Span) , value : Option < (Symbol , Span) > ,) -> lints :: UnexpectedCfgName { # [allow (rustc :: potential_query_instability)] let possibilities : Vec < Symbol > = sess . psess . check_config . expecteds . keys () . copied () . collect () ; let mut names_possibilities : Vec < _ > = if value . is_none () { # [allow (rustc :: potential_query_instability)] sess . psess . check_config . expecteds . iter () . filter_map (| (k , v) | match v { ExpectedValues :: Some (v) if v . contains (& Some (name)) => Some (k) , _ => None , }) . collect () } else { Vec :: new () } ; let is_from_cargo = rustc_session :: utils :: was_invoked_from_cargo () ; let is_from_external_macro = name_span . in_external_macro (sess . source_map ()) ; let mut is_feature_cfg = name == sym :: feature ; let code_sugg = if is_feature_cfg && is_from_cargo { lints :: unexpected_cfg_name :: CodeSuggestion :: DefineFeatures } else if let Some ((_value , value_span)) = value && name == sym :: version { lints :: unexpected_cfg_name :: CodeSuggestion :: VersionSyntax { between_name_and_value : name_span . between (value_span) , after_value : value_span . shrink_to_hi () , } } else if let Some (best_match) = find_best_match_for_name (& possibilities , name , None) { is_feature_cfg |= best_match == sym :: feature ; if let Some (ExpectedValues :: Some (best_match_values)) = sess . psess . check_config . expecteds . get (& best_match) { # [allow (rustc :: potential_query_instability)] let mut possibilities = best_match_values . iter () . flatten () . collect :: < Vec < _ > > () ; possibilities . sort_by_key (| s | s . as_str ()) ; let get_possibilities_sub = | | { if ! possibilities . is_empty () { let possibilities = possibilities . iter () . copied () . cloned () . collect :: < Vec < _ > > () . into () ; Some (lints :: unexpected_cfg_name :: ExpectedValues { best_match , possibilities }) } else { None } } ; let best_match = Ident :: new (best_match , name_span) ; if let Some ((value , value_span)) = value { if best_match_values . contains (& Some (value)) { lints :: unexpected_cfg_name :: CodeSuggestion :: SimilarNameAndValue { span : name_span , code : best_match . to_string () , } } else if best_match_values . contains (& None) { lints :: unexpected_cfg_name :: CodeSuggestion :: SimilarNameNoValue { span : name_span . to (value_span) , code : best_match . to_string () , } } else if let Some (first_value) = possibilities . first () { lints :: unexpected_cfg_name :: CodeSuggestion :: SimilarNameDifferentValues { span : name_span . to (value_span) , code : format ! ("{best_match} = \"{first_value}\"") , expected : get_possibilities_sub () , } } else { lints :: unexpected_cfg_name :: CodeSuggestion :: SimilarNameDifferentValues { span : name_span . to (value_span) , code : best_match . to_string () , expected : get_possibilities_sub () , } } } else { lints :: unexpected_cfg_name :: CodeSuggestion :: SimilarName { span : name_span , code : best_match . to_string () , expected : get_possibilities_sub () , } } } else { lints :: unexpected_cfg_name :: CodeSuggestion :: SimilarName { span : name_span , code : best_match . to_string () , expected : None , } } } else { let similar_values = if ! names_possibilities . is_empty () && names_possibilities . len () <= 3 { names_possibilities . sort () ; names_possibilities . iter () . map (| cfg_name | lints :: unexpected_cfg_name :: FoundWithSimilarValue { span : name_span , code : format ! ("{cfg_name} = \"{name}\"") , }) . collect () } else { vec ! [] } ; let (possibilities , and_more) = sort_and_truncate_possibilities (sess , possibilities , FilterWellKnownNames :: Yes) ; let expected_names = if ! possibilities . is_empty () { let possibilities : Vec < _ > = possibilities . into_iter () . map (| s | Ident :: new (s , name_span)) . collect () ; Some (lints :: unexpected_cfg_name :: ExpectedNames { possibilities : possibilities . into () , and_more , }) } else { None } ; lints :: unexpected_cfg_name :: CodeSuggestion :: SimilarValues { with_similar_values : similar_values , expected_names , } } ; let inst = | escape_quotes | { to_check_cfg_arg (Ident :: new (name , name_span) , value . map (| (v , _s) | v) , escape_quotes) } ; let invocation_help = if is_from_cargo { let help = if ! is_feature_cfg && ! is_from_external_macro { Some (cargo_help_sub (sess , & inst)) } else { None } ; lints :: unexpected_cfg_name :: InvocationHelp :: Cargo { help , macro_help : cargo_macro_help (tcx , name_span) , } } else { let help = lints :: UnexpectedCfgRustcHelp :: new (& inst (EscapeQuotes :: No)) ; lints :: unexpected_cfg_name :: InvocationHelp :: Rustc { help , macro_help : rustc_macro_help (name_span) , } } ; lints :: UnexpectedCfgName { code_sugg , invocation_help , name } }
}

macro_rules! unexpected_cfg_value_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unexpected_cfg_value in module {}", module_path!());
    };
}

mkfn!{
    unexpected_cfg_value_introspect!();
    pub (super) fn unexpected_cfg_value (sess : & Session , tcx : Option < TyCtxt < '_ > > , (name , name_span) : (Symbol , Span) , value : Option < (Symbol , Span) > ,) -> lints :: UnexpectedCfgValue { let Some (ExpectedValues :: Some (values)) = & sess . psess . check_config . expecteds . get (& name) else { bug ! ("it shouldn't be possible to have a diagnostic on a value whose name is not in values") ; } ; let mut have_none_possibility = false ; # [allow (rustc :: potential_query_instability)] let possibilities : Vec < Symbol > = values . iter () . inspect (| a | have_none_possibility |= a . is_none ()) . copied () . flatten () . collect () ; let is_from_cargo = rustc_session :: utils :: was_invoked_from_cargo () ; let is_from_external_macro = name_span . in_external_macro (sess . source_map ()) ; let code_sugg = if ! possibilities . is_empty () { let expected_values = { let (possibilities , and_more) = sort_and_truncate_possibilities (sess , possibilities . clone () , FilterWellKnownNames :: No ,) ; lints :: unexpected_cfg_value :: ExpectedValues { name , have_none_possibility , possibilities : possibilities . into () , and_more , } } ; let suggestion = if let Some ((value , value_span)) = value { if let Some (best_match) = find_best_match_for_name (& possibilities , value , None) { Some (lints :: unexpected_cfg_value :: ChangeValueSuggestion :: SimilarName { span : value_span , best_match , }) } else { None } } else if let & [first_possibility] = & possibilities [..] { Some (lints :: unexpected_cfg_value :: ChangeValueSuggestion :: SpecifyValue { span : name_span . shrink_to_hi () , first_possibility , }) } else { None } ; lints :: unexpected_cfg_value :: CodeSuggestion :: ChangeValue { expected_values , suggestion } } else if have_none_possibility { let suggestion = value . map (| (_value , value_span) | lints :: unexpected_cfg_value :: RemoveValueSuggestion { span : name_span . shrink_to_hi () . to (value_span) , }) ; lints :: unexpected_cfg_value :: CodeSuggestion :: RemoveValue { suggestion , name } } else { let span = if let Some ((_value , value_span)) = value { name_span . to (value_span) } else { name_span } ; let suggestion = lints :: unexpected_cfg_value :: RemoveConditionSuggestion { span } ; lints :: unexpected_cfg_value :: CodeSuggestion :: RemoveCondition { suggestion , name } } ; let can_suggest_adding_value = ! sess . psess . check_config . well_known_names . contains (& name) || (matches ! (sess . psess . unstable_features , rustc_feature :: UnstableFeatures :: Cheat) && ! sess . opts . unstable_opts . ui_testing) ; let inst = | escape_quotes | { to_check_cfg_arg (Ident :: new (name , name_span) , value . map (| (v , _s) | v) , escape_quotes) } ; let invocation_help = if is_from_cargo { let help = if name == sym :: feature && ! is_from_external_macro { if let Some ((value , _value_span)) = value { Some (lints :: unexpected_cfg_value :: CargoHelp :: AddFeature { value }) } else { Some (lints :: unexpected_cfg_value :: CargoHelp :: DefineFeatures) } } else if can_suggest_adding_value && ! is_from_external_macro { Some (lints :: unexpected_cfg_value :: CargoHelp :: Other (cargo_help_sub (sess , & inst))) } else { None } ; lints :: unexpected_cfg_value :: InvocationHelp :: Cargo { help , macro_help : cargo_macro_help (tcx , name_span) , } } else { let help = if can_suggest_adding_value { Some (lints :: UnexpectedCfgRustcHelp :: new (& inst (EscapeQuotes :: No))) } else { None } ; lints :: unexpected_cfg_value :: InvocationHelp :: Rustc { help , macro_help : rustc_macro_help (name_span) , } } ; lints :: UnexpectedCfgValue { code_sugg , invocation_help , has_value : value . is_some () , value : value . map_or_else (String :: new , | (v , _span) | v . to_string ()) , } }
}