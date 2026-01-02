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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use rustc_ast :: ast ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_hir :: limit :: Limit ;}
mkuse!{use rustc_macros :: { Diagnostic , Subdiagnostic } ;}
mkuse!{use rustc_span :: { Ident , MacroRulesNormalizedIdent , Span , Symbol } ;}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_expr_repeat_no_syntax_vars)] pub (crate) struct NoSyntaxVarsExprRepeat { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_must_repeat_once)] pub (crate) struct MustRepeatOnce { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_count_repetition_misplaced)] pub (crate) struct CountRepetitionMisplaced { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_var_still_repeating)] pub (crate) struct VarStillRepeating { # [primary_span] pub span : Span , pub ident : MacroRulesNormalizedIdent , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_meta_var_dif_seq_matchers)] pub (crate) struct MetaVarsDifSeqMatchers { # [primary_span] pub span : Span , pub msg : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_resolve_relative_path)] pub (crate) struct ResolveRelativePath { # [primary_span] pub span : Span , pub path : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_collapse_debuginfo_illegal)] pub (crate) struct CollapseMacroDebuginfoIllegal { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_macro_const_stability)] pub (crate) struct MacroConstStability { # [primary_span] # [label] pub span : Span , # [label (expand_label2)] pub head_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_macro_body_stability)] pub (crate) struct MacroBodyStability { # [primary_span] # [label] pub span : Span , # [label (expand_label2)] pub head_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_feature_removed , code = E0557)] # [note] pub (crate) struct FeatureRemoved < 'a > { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub reason : Option < FeatureRemovedReason < 'a > > , pub removed_rustc_version : & 'a str , pub pull_note : String , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (expand_reason)] pub (crate) struct FeatureRemovedReason < 'a > { pub reason : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_feature_not_allowed , code = E0725)] pub (crate) struct FeatureNotAllowed { # [primary_span] pub span : Span , pub name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_recursion_limit_reached)] # [help] pub (crate) struct RecursionLimitReached { # [primary_span] pub span : Span , pub descr : String , pub suggested_limit : Limit , pub crate_name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_malformed_feature_attribute , code = E0556)] pub (crate) struct MalformedFeatureAttribute { # [primary_span] pub span : Span , # [subdiagnostic] pub help : MalformedFeatureAttributeHelp , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum MalformedFeatureAttributeHelp { # [label (expand_expected)] Label { # [primary_span] span : Span , } , # [suggestion (expand_expected , code = "{suggestion}" , applicability = "maybe-incorrect")] Suggestion { # [primary_span] span : Span , suggestion : Symbol , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_remove_expr_not_supported)] pub (crate) struct RemoveExprNotSupported { # [primary_span] pub span : Span , }}}
mkitem!{mkenum!{# [derive (Diagnostic)] pub (crate) enum InvalidCfg { # [diag (expand_invalid_cfg_no_parens)] NotFollowedByParens { # [primary_span] # [suggestion (expand_invalid_cfg_expected_syntax , code = "cfg(/* predicate */)" , applicability = "has-placeholders")] span : Span , } , # [diag (expand_invalid_cfg_no_predicate)] NoPredicate { # [primary_span] # [suggestion (expand_invalid_cfg_expected_syntax , code = "cfg(/* predicate */)" , applicability = "has-placeholders")] span : Span , } , # [diag (expand_invalid_cfg_multiple_predicates)] MultiplePredicates { # [primary_span] span : Span , } , # [diag (expand_invalid_cfg_predicate_literal)] PredicateLiteral { # [primary_span] span : Span , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_wrong_fragment_kind)] pub (crate) struct WrongFragmentKind < 'a > { # [primary_span] pub span : Span , pub kind : & 'a str , pub name : & 'a ast :: Path , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_unsupported_key_value)] pub (crate) struct UnsupportedKeyValue { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_incomplete_parse)] # [note] pub (crate) struct IncompleteParse < 'a > { # [primary_span] pub span : Span , pub descr : String , # [label] pub label_span : Span , pub macro_path : & 'a ast :: Path , pub kind_name : & 'a str , # [note (expand_macro_expands_to_match_arm)] pub expands_to_match_arm : bool , # [suggestion (expand_suggestion_add_semi , style = "verbose" , code = ";" , applicability = "maybe-incorrect")] pub add_semicolon : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_remove_node_not_supported)] pub (crate) struct RemoveNodeNotSupported { # [primary_span] pub span : Span , pub descr : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_module_circular)] pub (crate) struct ModuleCircular { # [primary_span] pub span : Span , pub modules : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_module_in_block)] pub (crate) struct ModuleInBlock { # [primary_span] pub span : Span , # [subdiagnostic] pub name : Option < ModuleInBlockName > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (expand_note)] pub (crate) struct ModuleInBlockName { # [primary_span] pub span : Span , pub name : Ident , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_module_file_not_found , code = E0583)] # [help] # [note] pub (crate) struct ModuleFileNotFound { # [primary_span] pub span : Span , pub name : Ident , pub default_path : String , pub secondary_path : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_module_multiple_candidates , code = E0761)] # [help] pub (crate) struct ModuleMultipleCandidates { # [primary_span] pub span : Span , pub name : Ident , pub default_path : String , pub secondary_path : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_trace_macro)] pub (crate) struct TraceMacro { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_proc_macro_panicked)] pub (crate) struct ProcMacroPanicked { # [primary_span] pub span : Span , # [subdiagnostic] pub message : Option < ProcMacroPanickedHelp > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [help (expand_help)] pub (crate) struct ProcMacroPanickedHelp { pub message : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_proc_macro_derive_panicked)] pub (crate) struct ProcMacroDerivePanicked { # [primary_span] pub span : Span , # [subdiagnostic] pub message : Option < ProcMacroDerivePanickedHelp > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [help (expand_help)] pub (crate) struct ProcMacroDerivePanickedHelp { pub message : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_custom_attribute_panicked)] pub (crate) struct CustomAttributePanicked { # [primary_span] pub span : Span , # [subdiagnostic] pub message : Option < CustomAttributePanickedHelp > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [help (expand_help)] pub (crate) struct CustomAttributePanickedHelp { pub message : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_proc_macro_derive_tokens)] pub (crate) struct ProcMacroDeriveTokens { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_duplicate_matcher_binding)] pub (crate) struct DuplicateMatcherBinding { # [primary_span] # [label] pub span : Span , # [label (expand_label2)] pub prev : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_missing_fragment_specifier)] # [note] # [help (expand_valid)] pub (crate) struct MissingFragmentSpecifier { # [primary_span] pub span : Span , # [suggestion (expand_suggestion_add_fragspec , style = "verbose" , code = ":spec" , applicability = "maybe-incorrect")] pub add_span : Span , pub valid : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_invalid_fragment_specifier)] # [help] pub (crate) struct InvalidFragmentSpecifier { # [primary_span] pub span : Span , pub fragment : Ident , pub help : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_expected_paren_or_brace)] pub (crate) struct ExpectedParenOrBrace < 'a > { # [primary_span] pub span : Span , pub token : Cow < 'a , str > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_empty_delegation_mac)] pub (crate) struct EmptyDelegationMac { # [primary_span] pub span : Span , pub kind : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_glob_delegation_outside_impls)] pub (crate) struct GlobDelegationOutsideImpls { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_crate_name_in_cfg_attr)] pub (crate) struct CrateNameInCfgAttr { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_crate_type_in_cfg_attr)] pub (crate) struct CrateTypeInCfgAttr { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_glob_delegation_traitless_qpath)] pub (crate) struct GlobDelegationTraitlessQpath { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_proc_macro_back_compat)] # [note] pub (crate) struct ProcMacroBackCompat { pub crate_name : String , pub fixed_version : String , }}}
mkuse!{pub (crate) use metavar_exprs :: * ;}
mkmod!{metavar_exprs, { 
                getname!(metavar_exprs);
                getsrc!(metavar_exprs);
                getpath!(metavar_exprs);
                get_deps!(metavar_exprs);
                get_crates!(metavar_exprs);
                mkinclude!(metavar_exprs);
                mkuse!{use super :: * ;}
mkitem!{mkstruct!{# [derive (Diagnostic , Default)] # [diag (expand_mve_extra_tokens)] pub (crate) struct MveExtraTokens { # [primary_span] # [suggestion (code = "" , applicability = "machine-applicable")] pub span : Span , # [label] pub ident_span : Span , pub extra_count : usize , # [note (expand_exact)] pub exact_args_note : Option < () > , # [note (expand_range)] pub range_args_note : Option < () > , pub min_or_exact_args : usize , pub max_args : usize , pub name : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [note] # [diag (expand_mve_missing_paren)] pub (crate) struct MveMissingParen { # [primary_span] # [label] pub ident_span : Span , # [label (expand_unexpected)] pub unexpected_span : Option < Span > , # [suggestion (code = "( /* ... */ )" , applicability = "has-placeholders")] pub insert_span : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [note] # [diag (expand_mve_unrecognized_expr)] pub (crate) struct MveUnrecognizedExpr { # [primary_span] # [label] pub span : Span , pub valid_expr_list : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_mve_unrecognized_var)] pub (crate) struct MveUnrecognizedVar { # [primary_span] pub span : Span , pub key : MacroRulesNormalizedIdent , }}} 
            }}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (expand_macro_args_bad_delim)] pub (crate) struct MacroArgsBadDelim { # [primary_span] pub span : Span , # [subdiagnostic] pub sugg : MacroArgsBadDelimSugg , pub rule_kw : Symbol , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (expand_macro_args_bad_delim_sugg , applicability = "machine-applicable")] pub (crate) struct MacroArgsBadDelimSugg { # [suggestion_part (code = "(")] pub open : Span , # [suggestion_part (code = ")")] pub close : Span , }}}