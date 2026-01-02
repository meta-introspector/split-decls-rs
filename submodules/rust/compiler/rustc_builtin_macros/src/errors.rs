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
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Diag , DiagCtxtHandle , Diagnostic , EmissionGuarantee , Level , MultiSpan , SingleLabelManySpans , Subdiagnostic , } ;}
mkuse!{use rustc_macros :: { Diagnostic , Subdiagnostic } ;}
mkuse!{use rustc_span :: { Ident , Span , Symbol } ;}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_requires_cfg_pattern)] pub (crate) struct RequiresCfgPattern { # [primary_span] # [label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_expected_one_cfg_pattern)] pub (crate) struct OneCfgPattern { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_alloc_error_must_be_fn)] pub (crate) struct AllocErrorMustBeFn { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_assert_requires_boolean)] pub (crate) struct AssertRequiresBoolean { # [primary_span] # [label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_assert_requires_expression)] pub (crate) struct AssertRequiresExpression { # [primary_span] pub (crate) span : Span , # [suggestion (code = "" , applicability = "maybe-incorrect")] pub (crate) token : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_assert_missing_comma)] pub (crate) struct AssertMissingComma { # [primary_span] pub (crate) span : Span , # [suggestion (code = ", " , applicability = "maybe-incorrect" , style = "short")] pub (crate) comma : Span , }}}
mkitem!{mkenum!{# [derive (Diagnostic)] pub (crate) enum CfgAccessibleInvalid { # [diag (builtin_macros_cfg_accessible_unspecified_path)] UnspecifiedPath (# [primary_span] Span) , # [diag (builtin_macros_cfg_accessible_multiple_paths)] MultiplePaths (# [primary_span] Span) , # [diag (builtin_macros_cfg_accessible_literal_path)] LiteralPath (# [primary_span] Span) , # [diag (builtin_macros_cfg_accessible_has_args)] HasArguments (# [primary_span] Span) , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_cfg_accessible_indeterminate)] pub (crate) struct CfgAccessibleIndeterminate { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_concat_missing_literal)] # [note] pub (crate) struct ConcatMissingLiteral { # [primary_span] pub (crate) spans : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_concat_bytestr)] pub (crate) struct ConcatBytestr { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_concat_c_str_lit)] pub (crate) struct ConcatCStrLit { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_export_macro_rules)] pub (crate) struct ExportMacroRules { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_proc_macro)] pub (crate) struct ProcMacro { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_trace_macros)] pub (crate) struct TraceMacros { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_bench_sig)] pub (crate) struct BenchSig { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_alloc_must_statics)] pub (crate) struct AllocMustStatics { # [primary_span] pub (crate) span : Span , }}}
mkuse!{pub (crate) use autodiff :: * ;}
mkmod!{autodiff, { 
                getname!(autodiff);
                getsrc!(autodiff);
                getpath!(autodiff);
                get_deps!(autodiff);
                get_crates!(autodiff);
                mkinclude!(autodiff);
                mkuse!{use super :: * ;}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_autodiff_missing_config)] pub (crate) struct AutoDiffMissingConfig { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_autodiff_unknown_activity)] pub (crate) struct AutoDiffUnknownActivity { # [primary_span] pub (crate) span : Span , pub (crate) act : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_autodiff_ty_activity)] pub (crate) struct AutoDiffInvalidTypeForActivity { # [primary_span] pub (crate) span : Span , pub (crate) act : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_autodiff_number_activities)] pub (crate) struct AutoDiffInvalidNumberActivities { # [primary_span] pub (crate) span : Span , pub (crate) expected : usize , pub (crate) found : usize , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_autodiff_mode_activity)] pub (crate) struct AutoDiffInvalidApplicationModeAct { # [primary_span] pub (crate) span : Span , pub (crate) mode : String , pub (crate) act : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_autodiff_ret_activity)] pub (crate) struct AutoDiffInvalidRetAct { # [primary_span] pub (crate) span : Span , pub (crate) mode : String , pub (crate) act : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_autodiff_width)] pub (crate) struct AutoDiffInvalidWidth { # [primary_span] pub (crate) span : Span , pub (crate) width : u128 , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_autodiff)] pub (crate) struct AutoDiffInvalidApplication { # [primary_span] pub (crate) span : Span , }}} 
            }}
mkuse!{pub (crate) use ad_fallback :: * ;}
mkmod!{ad_fallback, { 
                getname!(ad_fallback);
                getsrc!(ad_fallback);
                getpath!(ad_fallback);
                get_deps!(ad_fallback);
                get_crates!(ad_fallback);
                mkinclude!(ad_fallback);
                mkuse!{use super :: * ;}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_autodiff_not_build)] pub (crate) struct AutoDiffSupportNotBuild { # [primary_span] pub (crate) span : Span , }}} 
            }}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_invalid)] pub (crate) struct ConcatBytesInvalid { # [primary_span] pub (crate) span : Span , pub (crate) lit_kind : & 'static str , # [subdiagnostic] pub (crate) sugg : Option < ConcatBytesInvalidSuggestion > , # [note (builtin_macros_c_str_note)] pub (crate) cs_note : Option < () > , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum ConcatBytesInvalidSuggestion { # [suggestion (builtin_macros_byte_char , code = "b{snippet}" , applicability = "machine-applicable")] CharLit { # [primary_span] span : Span , snippet : String , } , # [suggestion (builtin_macros_byte_str , code = "b{snippet}" , applicability = "machine-applicable")] StrLit { # [primary_span] span : Span , snippet : String , } , # [note (builtin_macros_c_str_note)] # [suggestion (builtin_macros_c_str , code = "{as_bstr}" , applicability = "machine-applicable")] CStrLit { # [primary_span] span : Span , as_bstr : String , } , # [suggestion (builtin_macros_number_array , code = "[{snippet}]" , applicability = "machine-applicable")] IntLit { # [primary_span] span : Span , snippet : String , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_oob)] pub (crate) struct ConcatBytesOob { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_non_u8)] pub (crate) struct ConcatBytesNonU8 { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_missing_literal)] # [note] pub (crate) struct ConcatBytesMissingLiteral { # [primary_span] pub (crate) spans : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_array)] pub (crate) struct ConcatBytesArray { # [primary_span] pub (crate) span : Span , # [note] # [help] pub (crate) bytestr : bool , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_bad_repeat)] pub (crate) struct ConcatBytesBadRepeat { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_bad_derive_target , code = E0774)] pub (crate) struct BadDeriveTarget { # [primary_span] # [label] pub (crate) span : Span , # [label (builtin_macros_label2)] pub (crate) item : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_tests_not_support)] pub (crate) struct TestsNotSupport { }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_unexpected_lit , code = E0777)] pub (crate) struct BadDeriveLit { # [primary_span] # [label] pub (crate) span : Span , # [subdiagnostic] pub help : BadDeriveLitHelp , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum BadDeriveLitHelp { # [help (builtin_macros_str_lit)] StrLit { sym : Symbol } , # [help (builtin_macros_other)] Other , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_derive_path_args_list)] pub (crate) struct DerivePathArgsList { # [suggestion (code = "" , applicability = "machine-applicable")] # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_derive_path_args_value)] pub (crate) struct DerivePathArgsValue { # [suggestion (code = "" , applicability = "machine-applicable")] # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_no_default_variant , code = E0665)] pub (crate) struct NoDefaultVariant { # [primary_span] pub (crate) span : Span , # [label] pub (crate) item_span : Span , # [subdiagnostic] pub (crate) suggs : Vec < NoDefaultVariantSugg > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (builtin_macros_suggestion , code = "#[default] " , applicability = "maybe-incorrect")] pub (crate) struct NoDefaultVariantSugg { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_multiple_defaults)] # [note] pub (crate) struct MultipleDefaults { # [primary_span] pub (crate) span : Span , # [label] pub (crate) first : Span , # [label (builtin_macros_additional)] pub additional : Vec < Span > , # [subdiagnostic] pub suggs : Vec < MultipleDefaultsSugg > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (builtin_macros_suggestion , applicability = "maybe-incorrect" , style = "tool-only")] pub (crate) struct MultipleDefaultsSugg { # [suggestion_part (code = "")] pub (crate) spans : Vec < Span > , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_non_unit_default)] # [help] pub (crate) struct NonUnitDefault { # [primary_span] pub (crate) span : Span , pub (crate) post : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_non_exhaustive_default)] # [help] pub (crate) struct NonExhaustiveDefault { # [primary_span] pub (crate) span : Span , # [label] pub (crate) non_exhaustive : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_multiple_default_attrs)] # [note] pub (crate) struct MultipleDefaultAttrs { # [primary_span] pub (crate) span : Span , # [label] pub (crate) first : Span , # [label (builtin_macros_label_again)] pub (crate) first_rest : Span , # [help] pub (crate) rest : MultiSpan , pub (crate) only_one : bool , # [subdiagnostic] pub (crate) sugg : MultipleDefaultAttrsSugg , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (builtin_macros_help , applicability = "machine-applicable" , style = "tool-only")] pub (crate) struct MultipleDefaultAttrsSugg { # [suggestion_part (code = "")] pub (crate) spans : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_default_arg)] pub (crate) struct DefaultHasArg { # [primary_span] # [suggestion (code = "#[default]" , style = "hidden" , applicability = "maybe-incorrect")] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_derive_from_wrong_target)] # [note (builtin_macros_derive_from_usage_note)] pub (crate) struct DeriveFromWrongTarget < 'a > { # [primary_span] pub (crate) span : MultiSpan , pub (crate) kind : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_derive_from_wrong_field_count)] # [note (builtin_macros_derive_from_usage_note)] pub (crate) struct DeriveFromWrongFieldCount { # [primary_span] pub (crate) span : MultiSpan , pub (crate) multiple_fields : bool , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_derive_macro_call)] pub (crate) struct DeriveMacroCall { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_cannot_derive_union)] pub (crate) struct DeriveUnion { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_env_takes_args)] pub (crate) struct EnvTakesArgs { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{pub (crate) struct EnvNotDefinedWithUserMessage { pub (crate) span : Span , pub (crate) msg_from_user : Symbol , }}}
mkitem!{mkimpl!{impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for EnvNotDefinedWithUserMessage { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { # [expect (rustc :: untranslatable_diagnostic , reason = "cannot translate user-provided messages")] let mut diag = Diag :: new (dcx , level , self . msg_from_user . to_string ()) ; diag . span (self . span) ; diag } }}}
mkitem!{mkenum!{# [derive (Diagnostic)] pub (crate) enum EnvNotDefined < 'a > { # [diag (builtin_macros_env_not_defined)] # [help (builtin_macros_cargo)] CargoEnvVar { # [primary_span] span : Span , var : Symbol , var_expr : & 'a rustc_ast :: Expr , } , # [diag (builtin_macros_env_not_defined)] # [help (builtin_macros_custom)] CustomEnvVar { # [primary_span] span : Span , var : Symbol , var_expr : & 'a rustc_ast :: Expr , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_env_not_unicode)] pub (crate) struct EnvNotUnicode { # [primary_span] pub (crate) span : Span , pub (crate) var : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_format_requires_string)] pub (crate) struct FormatRequiresString { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_format_duplicate_arg)] pub (crate) struct FormatDuplicateArg { # [primary_span] pub (crate) span : Span , # [label (builtin_macros_label1)] pub (crate) prev : Span , # [label (builtin_macros_label2)] pub (crate) duplicate : Span , pub (crate) ident : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_format_positional_after_named)] pub (crate) struct PositionalAfterNamed { # [primary_span] # [label] pub (crate) span : Span , # [label (builtin_macros_named_args)] pub (crate) args : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_format_string_invalid)] pub (crate) struct InvalidFormatString { # [primary_span] # [label] pub (crate) span : Span , pub (crate) desc : String , pub (crate) label1 : String , # [subdiagnostic] pub (crate) note_ : Option < InvalidFormatStringNote > , # [subdiagnostic] pub (crate) label_ : Option < InvalidFormatStringLabel > , # [subdiagnostic] pub (crate) sugg_ : Option < InvalidFormatStringSuggestion > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (builtin_macros_note)] pub (crate) struct InvalidFormatStringNote { pub (crate) note : String , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (builtin_macros_second_label)] pub (crate) struct InvalidFormatStringLabel { # [primary_span] pub (crate) span : Span , pub (crate) label : String , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum InvalidFormatStringSuggestion { # [multipart_suggestion (builtin_macros_format_use_positional , style = "verbose" , applicability = "machine-applicable")] UsePositional { # [suggestion_part (code = "{len}")] captured : Span , len : String , # [suggestion_part (code = ", {arg}")] span : Span , arg : String , } , # [suggestion (builtin_macros_format_remove_raw_ident , code = "" , applicability = "machine-applicable")] RemoveRawIdent { # [primary_span] span : Span , } , # [suggestion (builtin_macros_format_reorder_format_parameter , code = "{replacement}" , style = "verbose" , applicability = "machine-applicable")] ReorderFormatParameter { # [primary_span] span : Span , replacement : String , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_format_no_arg_named)] # [note] # [note (builtin_macros_note2)] pub (crate) struct FormatNoArgNamed { # [primary_span] pub (crate) span : Span , pub (crate) name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_format_unknown_trait)] # [note] pub (crate) struct FormatUnknownTrait < 'a > { # [primary_span] pub (crate) span : Span , pub (crate) ty : & 'a str , # [subdiagnostic] pub (crate) suggs : Vec < FormatUnknownTraitSugg > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (builtin_macros_suggestion , code = "{fmt}" , style = "tool-only" , applicability = "maybe-incorrect")] pub (crate) struct FormatUnknownTraitSugg { # [primary_span] pub span : Span , pub fmt : & 'static str , pub trait_name : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_format_unused_arg)] pub (crate) struct FormatUnusedArg { # [primary_span] # [label (builtin_macros_format_unused_arg)] pub (crate) span : Span , pub (crate) named : bool , }}}
mkitem!{mkimpl!{impl Subdiagnostic for FormatUnusedArg { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("named" , self . named) ; let msg = diag . eagerly_translate (crate :: fluent_generated :: builtin_macros_format_unused_arg) ; diag . remove_arg ("named") ; diag . span_label (self . span , msg) ; } }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_format_unused_args)] pub (crate) struct FormatUnusedArgs { # [primary_span] pub (crate) unused : Vec < Span > , # [label] pub (crate) fmt : Span , # [subdiagnostic] pub (crate) unused_labels : Vec < FormatUnusedArg > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_format_pos_mismatch)] pub (crate) struct FormatPositionalMismatch { # [primary_span] pub (crate) span : MultiSpan , pub (crate) n : usize , pub (crate) desc : String , # [subdiagnostic] pub (crate) highlight : SingleLabelManySpans , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_format_redundant_args)] pub (crate) struct FormatRedundantArgs { # [primary_span] pub (crate) span : MultiSpan , pub (crate) n : usize , # [note] pub (crate) note : MultiSpan , # [subdiagnostic] pub (crate) sugg : Option < FormatRedundantArgsSugg > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (builtin_macros_suggestion , applicability = "machine-applicable")] pub (crate) struct FormatRedundantArgsSugg { # [suggestion_part (code = "")] pub (crate) spans : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_test_case_non_item)] pub (crate) struct TestCaseNonItem { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_test_bad_fn)] pub (crate) struct TestBadFn { # [primary_span] pub (crate) span : Span , # [label] pub (crate) cause : Span , pub (crate) kind : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_explicit_register_name)] pub (crate) struct AsmExplicitRegisterName { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_mutually_exclusive)] pub (crate) struct AsmMutuallyExclusive { # [primary_span] pub (crate) spans : Vec < Span > , pub (crate) opt1 : & 'static str , pub (crate) opt2 : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_pure_combine)] pub (crate) struct AsmPureCombine { # [primary_span] pub (crate) spans : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_pure_no_output)] pub (crate) struct AsmPureNoOutput { # [primary_span] pub (crate) spans : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_modifier_invalid)] pub (crate) struct AsmModifierInvalid { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_attribute_not_supported)] pub (crate) struct AsmAttributeNotSupported { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_duplicate_arg)] pub (crate) struct AsmDuplicateArg { # [primary_span] # [label (builtin_macros_arg)] pub (crate) span : Span , # [label] pub (crate) prev : Span , pub (crate) name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_pos_after)] pub (crate) struct AsmPositionalAfter { # [primary_span] # [label (builtin_macros_pos)] pub (crate) span : Span , # [label (builtin_macros_named)] pub (crate) named : Vec < Span > , # [label (builtin_macros_explicit)] pub (crate) explicit : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_noreturn)] pub (crate) struct AsmNoReturn { # [primary_span] pub (crate) outputs_sp : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_no_matched_argument_name)] pub (crate) struct AsmNoMatchedArgumentName { pub (crate) name : String , # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_mayunwind)] pub (crate) struct AsmMayUnwind { # [primary_span] pub (crate) labels_sp : Vec < Span > , }}}
mkitem!{mkstruct!{pub (crate) struct AsmClobberNoReg { pub (crate) spans : Vec < Span > , pub (crate) clobbers : Vec < Span > , }}}
mkitem!{mkimpl!{impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for AsmClobberNoReg { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let lbl1 = dcx . eagerly_translate_to_string (crate :: fluent_generated :: builtin_macros_asm_clobber_abi , [] . into_iter () ,) ; let lbl2 = dcx . eagerly_translate_to_string (crate :: fluent_generated :: builtin_macros_asm_clobber_outputs , [] . into_iter () ,) ; Diag :: new (dcx , level , crate :: fluent_generated :: builtin_macros_asm_clobber_no_reg) . with_span (self . spans . clone ()) . with_span_labels (self . clobbers , & lbl1) . with_span_labels (self . spans , & lbl2) } }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_opt_already_provided)] pub (crate) struct AsmOptAlreadyprovided { # [primary_span] # [label] pub (crate) span : Span , pub (crate) symbol : Symbol , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub (crate) span_with_comma : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_unsupported_option)] pub (crate) struct AsmUnsupportedOption { # [primary_span] # [label] pub (crate) span : Span , pub (crate) symbol : Symbol , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub (crate) span_with_comma : Span , pub (crate) macro_name : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_asm_unsupported_clobber_abi)] pub (crate) struct AsmUnsupportedClobberAbi { # [primary_span] pub (crate) spans : Vec < Span > , pub (crate) macro_name : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_test_runner_invalid)] pub (crate) struct TestRunnerInvalid { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_test_runner_nargs)] pub (crate) struct TestRunnerNargs { # [primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_expected_comma_in_list)] pub (crate) struct ExpectedCommaInList { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_only_one_argument)] pub (crate) struct OnlyOneArgument < 'a > { # [primary_span] pub span : Span , pub name : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_takes_no_arguments)] pub (crate) struct TakesNoArguments < 'a > { # [primary_span] pub span : Span , pub name : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_proc_macro_attribute_only_usable_with_crate_type)] pub (crate) struct AttributeOnlyUsableWithCrateType < 'a > { # [primary_span] pub span : Span , pub path : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_source_uitls_expected_item)] pub (crate) struct ExpectedItem < 'a > { # [primary_span] pub span : Span , pub token : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_naked_functions_testing_attribute , code = E0736)] pub (crate) struct NakedFunctionTestingAttribute { # [primary_span] # [label (builtin_macros_naked_attribute)] pub naked_span : Span , # [label] pub testing_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_non_generic_pointee)] pub (crate) struct NonGenericPointee { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_expected_other)] pub (crate) struct AsmExpectedOther { # [primary_span] # [label (builtin_macros_expected_other)] pub (crate) span : Span , pub (crate) is_inline_asm : bool , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_cfg_select_no_matches)] pub (crate) struct CfgSelectNoMatches { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_cfg_select_unreachable)] pub (crate) struct CfgSelectUnreachable { # [primary_span] # [label (builtin_macros_label2)] pub span : Span , # [label] pub wildcard_span : Span , }}}