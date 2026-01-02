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
mkuse!{use std :: io :: Error ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Applicability , Diag , DiagCtxtHandle , DiagSymbolList , Diagnostic , EmissionGuarantee , Level , MultiSpan , Subdiagnostic , } ;}
mkuse!{use rustc_hir :: Target ;}
mkuse!{use rustc_hir :: attrs :: { MirDialect , MirPhase } ;}
mkuse!{use rustc_macros :: { Diagnostic , LintDiagnostic , Subdiagnostic } ;}
mkuse!{use rustc_middle :: ty :: { MainDefinition , Ty } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span , Symbol } ;}
mkuse!{use crate :: check_attr :: ProcMacroKind ;}
mkuse!{use crate :: fluent_generated as fluent ;}
mkuse!{use crate :: lang_items :: Duplicate ;}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_incorrect_do_not_recommend_location)] pub (crate) struct IncorrectDoNotRecommendLocation ;}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_incorrect_do_not_recommend_args)] pub (crate) struct DoNotRecommendDoesNotExpectArgs ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_autodiff_attr)] pub (crate) struct AutoDiffAttr { # [primary_span] # [label] pub attr_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_loop_match_attr)] pub (crate) struct LoopMatchAttr { # [primary_span] pub attr_span : Span , # [label] pub node_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_const_continue_attr)] pub (crate) struct ConstContinueAttr { # [primary_span] pub attr_span : Span , # [label] pub node_span : Span , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_mixed_export_name_and_no_mangle)] pub (crate) struct MixedExportNameAndNoMangle { # [label] # [suggestion (style = "verbose" , code = "" , applicability = "machine-applicable")] pub no_mangle_span : Span , # [note] pub export_name_span : Span , pub no_mangle_attr : & 'static str , pub export_name_attr : & 'static str , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_outer_crate_level_attr)] pub (crate) struct OuterCrateLevelAttr { # [subdiagnostic] pub suggestion : OuterCrateLevelAttrSuggestion , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (passes_outer_crate_level_attr_suggestion , style = "verbose")] pub (crate) struct OuterCrateLevelAttrSuggestion { # [suggestion_part (code = "!")] pub bang_position : Span , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_inner_crate_level_attr)] pub (crate) struct InnerCrateLevelAttr ;}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_ignored_attr_with_macro)] pub (crate) struct IgnoredAttrWithMacro < 'a > { pub sym : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_should_be_applied_to_fn)] pub (crate) struct AttrShouldBeAppliedToFn { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , pub on_crate : bool , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_non_exhaustive_with_default_field_values)] pub (crate) struct NonExhaustiveWithDefaultFieldValues { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_should_be_applied_to_trait)] pub (crate) struct AttrShouldBeAppliedToTrait { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_should_be_applied_to_static)] pub (crate) struct AttrShouldBeAppliedToStatic { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_expect_str)] pub (crate) struct DocExpectStr < 'a > { # [primary_span] pub attr_span : Span , pub attr_name : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_alias_empty)] pub (crate) struct DocAliasEmpty < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_alias_bad_char)] pub (crate) struct DocAliasBadChar < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , pub char_ : char , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_alias_start_end)] pub (crate) struct DocAliasStartEnd < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_alias_bad_location)] pub (crate) struct DocAliasBadLocation < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , pub location : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_alias_not_an_alias)] pub (crate) struct DocAliasNotAnAlias < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_alias_duplicated)] pub (crate) struct DocAliasDuplicated { # [label] pub first_defn : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_alias_not_string_literal)] pub (crate) struct DocAliasNotStringLiteral { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_alias_malformed)] pub (crate) struct DocAliasMalformed { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_keyword_attribute_empty_mod)] pub (crate) struct DocKeywordAttributeEmptyMod { # [primary_span] pub span : Span , pub attr_name : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_keyword_not_keyword)] # [help] pub (crate) struct DocKeywordNotKeyword { # [primary_span] pub span : Span , pub keyword : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_attribute_not_attribute)] # [help] pub (crate) struct DocAttributeNotAttribute { # [primary_span] pub span : Span , pub attribute : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_keyword_attribute_not_mod)] pub (crate) struct DocKeywordAttributeNotMod { # [primary_span] pub span : Span , pub attr_name : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_fake_variadic_not_valid)] pub (crate) struct DocFakeVariadicNotValid { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_keyword_only_impl)] pub (crate) struct DocKeywordOnlyImpl { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_search_unbox_invalid)] pub (crate) struct DocSearchUnboxInvalid { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_inline_conflict)] # [help] pub (crate) struct DocKeywordConflict { # [primary_span] pub spans : MultiSpan , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_inline_only_use)] # [note] pub (crate) struct DocInlineOnlyUse { # [label] pub attr_span : Span , # [label (passes_not_a_use_item_label)] pub item_span : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_masked_only_extern_crate)] # [note] pub (crate) struct DocMaskedOnlyExternCrate { # [label] pub attr_span : Span , # [label (passes_not_an_extern_crate_label)] pub item_span : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_masked_not_extern_crate_self)] pub (crate) struct DocMaskedNotExternCrateSelf { # [label] pub attr_span : Span , # [label (passes_extern_crate_self_label)] pub item_span : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_doc_attr_not_crate_level)] pub (crate) struct DocAttrNotCrateLevel < 'a > { # [primary_span] pub span : Span , pub attr_name : & 'a str , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown)] pub (crate) struct DocTestUnknown { pub path : String , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_test_literal)] pub (crate) struct DocTestLiteral ;}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_test_takes_list)] pub (crate) struct DocTestTakesList ;}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_cfg_hide_takes_list)] pub (crate) struct DocCfgHideTakesList ;}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_any)] pub (crate) struct DocTestUnknownAny { pub path : String , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_spotlight)] # [note] # [note (passes_no_op_note)] pub (crate) struct DocTestUnknownSpotlight { pub path : String , # [suggestion (style = "short" , applicability = "machine-applicable" , code = "notable_trait")] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_passes)] # [note] # [help] # [note (passes_no_op_note)] pub (crate) struct DocTestUnknownPasses { pub path : String , # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_plugins)] # [note] # [note (passes_no_op_note)] pub (crate) struct DocTestUnknownPlugins { pub path : String , # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_test_unknown_include)] pub (crate) struct DocTestUnknownInclude { pub path : String , pub value : String , pub inner : & 'static str , # [suggestion (code = "#{inner}[doc = include_str!(\"{value}\")]")] pub sugg : (Span , Applicability) , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_doc_invalid)] pub (crate) struct DocInvalid ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_has_incoherent_inherent_impl)] pub (crate) struct HasIncoherentInherentImpl { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_both_ffi_const_and_pure , code = E0757)] pub (crate) struct BothFfiConstAndPure { # [primary_span] pub attr_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_must_not_suspend)] pub (crate) struct MustNotSuspend { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_link)] # [warning] pub (crate) struct Link { # [label] pub span : Option < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_no_link)] pub (crate) struct NoLink { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_only)] pub (crate) struct RustcLegacyConstGenericsOnly { # [primary_span] pub attr_span : Span , # [label] pub param_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_index)] pub (crate) struct RustcLegacyConstGenericsIndex { # [primary_span] pub attr_span : Span , # [label] pub generics_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_index_exceed)] pub (crate) struct RustcLegacyConstGenericsIndexExceed { # [primary_span] # [label] pub span : Span , pub arg_count : usize , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_rustc_legacy_const_generics_index_negative)] pub (crate) struct RustcLegacyConstGenericsIndexNegative { # [primary_span] pub invalid_args : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_rustc_dirty_clean)] pub (crate) struct RustcDirtyClean { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_repr_conflicting , code = E0566)] pub (crate) struct ReprConflicting { # [primary_span] pub hint_spans : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_repr_align_greater_than_target_max , code = E0589)] # [note] pub (crate) struct InvalidReprAlignForTarget { # [primary_span] pub span : Span , pub size : u64 , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_repr_conflicting , code = E0566)] pub (crate) struct ReprConflictingLint ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_macro_only_attribute)] pub (crate) struct MacroOnlyAttribute { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_debug_visualizer_placement)] pub (crate) struct DebugVisualizerPlacement { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_debug_visualizer_invalid)] # [note (passes_note_1)] # [note (passes_note_2)] # [note (passes_note_3)] pub (crate) struct DebugVisualizerInvalid { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_debug_visualizer_unreadable)] pub (crate) struct DebugVisualizerUnreadable < 'a > { # [primary_span] pub span : Span , pub file : & 'a Path , pub error : Error , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_rustc_allow_const_fn_unstable)] pub (crate) struct RustcAllowConstFnUnstable { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_rustc_pub_transparent)] pub (crate) struct RustcPubTransparent { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_rustc_force_inline_coro)] pub (crate) struct RustcForceInlineCoro { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }}}
mkitem!{mkenum!{# [derive (LintDiagnostic)] pub (crate) enum MacroExport { # [diag (passes_macro_export)] Normal , # [diag (passes_macro_export_on_decl_macro)] # [note] OnDeclMacro , # [diag (passes_invalid_macro_export_arguments)] InvalidArgument , # [diag (passes_invalid_macro_export_arguments_too_many_items)] TooManyItems , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum UnusedNote { # [note (passes_unused_empty_lints_note)] EmptyList { name : Symbol } , # [note (passes_unused_no_lints_note)] NoLints { name : Symbol } , # [note (passes_unused_default_method_body_const_note)] DefaultMethodBodyConst , # [note (passes_unused_linker_messages_note)] LinkerMessagesBinaryCrateOnly , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unused)] pub (crate) struct Unused { # [suggestion (code = "" , applicability = "machine-applicable")] pub attr_span : Span , # [subdiagnostic] pub note : UnusedNote , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_non_exported_macro_invalid_attrs , code = E0518)] pub (crate) struct NonExportedMacroInvalidAttrs { # [primary_span] # [label] pub attr_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_may_dangle)] pub (crate) struct InvalidMayDangle { # [primary_span] pub attr_span : Span , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unused_duplicate)] pub (crate) struct UnusedDuplicate { # [suggestion (code = "" , applicability = "machine-applicable")] pub this : Span , # [note] pub other : Span , # [warning] pub warning : bool , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_unused_multiple)] pub (crate) struct UnusedMultiple { # [primary_span] # [suggestion (code = "" , applicability = "machine-applicable")] pub this : Span , # [note] pub other : Span , pub name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_rustc_lint_opt_ty)] pub (crate) struct RustcLintOptTy { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_rustc_lint_opt_deny_field_access)] pub (crate) struct RustcLintOptDenyFieldAccess { # [primary_span] pub attr_span : Span , # [label] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_collapse_debuginfo)] pub (crate) struct CollapseDebuginfo { # [primary_span] pub attr_span : Span , # [label] pub defn_span : Span , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_deprecated_annotation_has_no_effect)] pub (crate) struct DeprecatedAnnotationHasNoEffect { # [suggestion (applicability = "machine-applicable" , code = "")] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_unknown_external_lang_item , code = E0264)] pub (crate) struct UnknownExternLangItem { # [primary_span] pub span : Span , pub lang_item : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_missing_panic_handler)] pub (crate) struct MissingPanicHandler ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_panic_unwind_without_std)] # [help] # [note] pub (crate) struct PanicUnwindWithoutStd ;}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_missing_lang_item)] # [note] # [help] pub (crate) struct MissingLangItem { pub name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_lang_item_fn_with_track_caller)] pub (crate) struct LangItemWithTrackCaller { # [primary_span] pub attr_span : Span , pub name : Symbol , # [label] pub sig_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_lang_item_fn_with_target_feature)] pub (crate) struct LangItemWithTargetFeature { # [primary_span] pub attr_span : Span , pub name : Symbol , # [label] pub sig_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_lang_item_on_incorrect_target , code = E0718)] pub (crate) struct LangItemOnIncorrectTarget { # [primary_span] # [label] pub span : Span , pub name : Symbol , pub expected_target : Target , pub actual_target : Target , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_unknown_lang_item , code = E0522)] pub (crate) struct UnknownLangItem { # [primary_span] # [label] pub span : Span , pub name : Symbol , }}}
mkitem!{mkstruct!{pub (crate) struct InvalidAttrAtCrateLevel { pub span : Span , pub sugg_span : Option < Span > , pub name : Symbol , pub item : Option < ItemFollowingInnerAttr > , }}}
mkitem!{mkstruct!{# [derive (Clone , Copy)] pub (crate) struct ItemFollowingInnerAttr { pub span : Span , pub kind : & 'static str , }}}
mkitem!{mkimpl!{impl < G : EmissionGuarantee > Diagnostic < '_ , G > for InvalidAttrAtCrateLevel { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let mut diag = Diag :: new (dcx , level , fluent :: passes_invalid_attr_at_crate_level) ; diag . span (self . span) ; diag . arg ("name" , self . name) ; if let Some (span) = self . sugg_span { diag . span_suggestion_verbose (span , fluent :: passes_suggestion , String :: new () , Applicability :: MachineApplicable ,) ; } if let Some (item) = self . item { diag . arg ("kind" , item . kind) ; diag . span_label (item . span , fluent :: passes_invalid_attr_at_crate_level_item) ; } diag } }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_duplicate_diagnostic_item_in_crate)] pub (crate) struct DuplicateDiagnosticItemInCrate { # [primary_span] pub duplicate_span : Option < Span > , # [note (passes_diagnostic_item_first_defined)] pub orig_span : Option < Span > , # [note] pub different_crates : bool , pub crate_name : Symbol , pub orig_crate_name : Symbol , pub name : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_layout_abi)] pub (crate) struct LayoutAbi { # [primary_span] pub span : Span , pub abi : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_layout_align)] pub (crate) struct LayoutAlign { # [primary_span] pub span : Span , pub align : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_layout_size)] pub (crate) struct LayoutSize { # [primary_span] pub span : Span , pub size : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_layout_homogeneous_aggregate)] pub (crate) struct LayoutHomogeneousAggregate { # [primary_span] pub span : Span , pub homogeneous_aggregate : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_layout_of)] pub (crate) struct LayoutOf < 'tcx > { # [primary_span] pub span : Span , pub normalized_ty : Ty < 'tcx > , pub ty_layout : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_layout_invalid_attribute)] pub (crate) struct LayoutInvalidAttribute { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_abi_of)] pub (crate) struct AbiOf { # [primary_span] pub span : Span , pub fn_name : Symbol , pub fn_abi : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_abi_ne)] pub (crate) struct AbiNe { # [primary_span] pub span : Span , pub left : String , pub right : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_abi_invalid_attribute)] pub (crate) struct AbiInvalidAttribute { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_unrecognized_argument)] pub (crate) struct UnrecognizedArgument { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_feature_stable_twice , code = E0711)] pub (crate) struct FeatureStableTwice { # [primary_span] pub span : Span , pub feature : Symbol , pub since : Symbol , pub prev_since : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_feature_previously_declared , code = E0711)] pub (crate) struct FeaturePreviouslyDeclared < 'a > { # [primary_span] pub span : Span , pub feature : Symbol , pub declared : & 'a str , pub prev_declared : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_attr_only_in_functions)] pub (crate) struct AttrOnlyInFunctions { # [primary_span] pub span : Span , pub attr : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_multiple_rustc_main , code = E0137)] pub (crate) struct MultipleRustcMain { # [primary_span] pub span : Span , # [label (passes_first)] pub first : Span , # [label (passes_additional)] pub additional : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_extern_main)] pub (crate) struct ExternMain { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{pub (crate) struct NoMainErr { pub sp : Span , pub crate_name : Symbol , pub has_filename : bool , pub filename : PathBuf , pub file_empty : bool , pub non_main_fns : Vec < Span > , pub main_def_opt : Option < MainDefinition > , pub add_teach_note : bool , }}}
mkitem!{mkimpl!{impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for NoMainErr { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let mut diag = Diag :: new (dcx , level , fluent :: passes_no_main_function) ; diag . span (DUMMY_SP) ; diag . code (E0601) ; diag . arg ("crate_name" , self . crate_name) ; diag . arg ("filename" , self . filename) ; diag . arg ("has_filename" , self . has_filename) ; let note = if ! self . non_main_fns . is_empty () { for & span in & self . non_main_fns { diag . span_note (span , fluent :: passes_here_is_main) ; } diag . note (fluent :: passes_one_or_more_possible_main) ; diag . help (fluent :: passes_consider_moving_main) ; fluent :: passes_main_must_be_defined_at_crate } else if self . has_filename { fluent :: passes_consider_adding_main_to_file } else { fluent :: passes_consider_adding_main_at_crate } ; if self . file_empty { diag . note (note) ; } else { diag . span (self . sp . shrink_to_hi ()) ; diag . span_label (self . sp . shrink_to_hi () , note) ; } if let Some (main_def) = self . main_def_opt && main_def . opt_fn_def_id () . is_none () { diag . span_label (main_def . span , fluent :: passes_non_function_main) ; } if self . add_teach_note { diag . note (fluent :: passes_teach_note) ; } diag } }}}
mkitem!{mkstruct!{pub (crate) struct DuplicateLangItem { pub local_span : Option < Span > , pub lang_item_name : Symbol , pub crate_name : Symbol , pub dependency_of : Option < Symbol > , pub is_local : bool , pub path : String , pub first_defined_span : Option < Span > , pub orig_crate_name : Option < Symbol > , pub orig_dependency_of : Option < Symbol > , pub orig_is_local : bool , pub orig_path : String , pub (crate) duplicate : Duplicate , }}}
mkitem!{mkimpl!{impl < G : EmissionGuarantee > Diagnostic < '_ , G > for DuplicateLangItem { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let mut diag = Diag :: new (dcx , level , match self . duplicate { Duplicate :: Plain => fluent :: passes_duplicate_lang_item , Duplicate :: Crate => fluent :: passes_duplicate_lang_item_crate , Duplicate :: CrateDepends => fluent :: passes_duplicate_lang_item_crate_depends , } ,) ; diag . code (E0152) ; diag . arg ("lang_item_name" , self . lang_item_name) ; diag . arg ("crate_name" , self . crate_name) ; if let Some (dependency_of) = self . dependency_of { diag . arg ("dependency_of" , dependency_of) ; } diag . arg ("path" , self . path) ; if let Some (orig_crate_name) = self . orig_crate_name { diag . arg ("orig_crate_name" , orig_crate_name) ; } if let Some (orig_dependency_of) = self . orig_dependency_of { diag . arg ("orig_dependency_of" , orig_dependency_of) ; } diag . arg ("orig_path" , self . orig_path) ; if let Some (span) = self . local_span { diag . span (span) ; } if let Some (span) = self . first_defined_span { diag . span_note (span , fluent :: passes_first_defined_span) ; } else { if self . orig_dependency_of . is_none () { diag . note (fluent :: passes_first_defined_crate) ; } else { diag . note (fluent :: passes_first_defined_crate_depends) ; } if self . orig_is_local { diag . note (fluent :: passes_first_definition_local) ; } else { diag . note (fluent :: passes_first_definition_path) ; } if self . is_local { diag . note (fluent :: passes_second_definition_local) ; } else { diag . note (fluent :: passes_second_definition_path) ; } } diag } }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_incorrect_target , code = E0718)] pub (crate) struct IncorrectTarget < 'a > { # [primary_span] pub span : Span , # [label] pub generics_span : Span , pub name : & 'a str , pub kind : & 'static str , pub num : usize , pub actual_num : usize , pub at_least : bool , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_incorrect_crate_type)] pub (crate) struct IncorrectCrateType { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_useless_assignment)] pub (crate) struct UselessAssignment < 'a > { pub is_field_assign : bool , pub ty : Ty < 'a > , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_inline_ignored_for_exported)] # [help] pub (crate) struct InlineIgnoredForExported { }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_object_lifetime_err)] pub (crate) struct ObjectLifetimeErr { # [primary_span] pub span : Span , pub repr : String , }}}
mkitem!{mkenum!{# [derive (Diagnostic)] pub (crate) enum AttrApplication { # [diag (passes_attr_application_enum , code = E0517)] Enum { # [primary_span] hint_span : Span , # [label] span : Span , } , # [diag (passes_attr_application_struct , code = E0517)] Struct { # [primary_span] hint_span : Span , # [label] span : Span , } , # [diag (passes_attr_application_struct_union , code = E0517)] StructUnion { # [primary_span] hint_span : Span , # [label] span : Span , } , # [diag (passes_attr_application_struct_enum_union , code = E0517)] StructEnumUnion { # [primary_span] hint_span : Span , # [label] span : Span , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_transparent_incompatible , code = E0692)] pub (crate) struct TransparentIncompatible { # [primary_span] pub hint_spans : Vec < Span > , pub target : String , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_deprecated_attribute , code = E0549)] pub (crate) struct DeprecatedAttribute { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_useless_stability)] pub (crate) struct UselessStability { # [primary_span] # [label] pub span : Span , # [label (passes_item)] pub item_sp : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_cannot_stabilize_deprecated)] pub (crate) struct CannotStabilizeDeprecated { # [primary_span] # [label] pub span : Span , # [label (passes_item)] pub item_sp : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_unstable_attr_for_already_stable_feature)] pub (crate) struct UnstableAttrForAlreadyStableFeature { # [primary_span] # [label] # [help] pub attr_span : Span , # [label (passes_item)] pub item_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_missing_stability_attr)] pub (crate) struct MissingStabilityAttr < 'a > { # [primary_span] pub span : Span , pub descr : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_missing_const_stab_attr)] pub (crate) struct MissingConstStabAttr < 'a > { # [primary_span] pub span : Span , pub descr : & 'a str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_trait_impl_const_stable)] # [note] pub (crate) struct TraitImplConstStable { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_trait_impl_const_stability_mismatch)] pub (crate) struct TraitImplConstStabilityMismatch { # [primary_span] pub span : Span , # [subdiagnostic] pub impl_stability : ImplConstStability , # [subdiagnostic] pub trait_stability : TraitConstStability , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum TraitConstStability { # [note (passes_trait_impl_const_stability_mismatch_trait_stable)] Stable { # [primary_span] span : Span , } , # [note (passes_trait_impl_const_stability_mismatch_trait_unstable)] Unstable { # [primary_span] span : Span , } , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum ImplConstStability { # [note (passes_trait_impl_const_stability_mismatch_impl_stable)] Stable { # [primary_span] span : Span , } , # [note (passes_trait_impl_const_stability_mismatch_impl_unstable)] Unstable { # [primary_span] span : Span , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_unknown_feature , code = E0635)] pub (crate) struct UnknownFeature { # [primary_span] pub span : Span , pub feature : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_unknown_feature_alias , code = E0635)] pub (crate) struct RenamedFeature { # [primary_span] pub span : Span , pub feature : Symbol , pub alias : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_implied_feature_not_exist)] pub (crate) struct ImpliedFeatureNotExist { # [primary_span] pub span : Span , pub feature : Symbol , pub implied_by : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_duplicate_feature_err , code = E0636)] pub (crate) struct DuplicateFeatureErr { # [primary_span] pub span : Span , pub feature : Symbol , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_missing_const_err)] pub (crate) struct MissingConstErr { # [primary_span] # [help] pub fn_sig_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_const_stable_not_stable)] pub (crate) struct ConstStableNotStable { # [primary_span] pub fn_sig_span : Span , # [label] pub const_span : Span , }}}
mkitem!{mkenum!{# [derive (LintDiagnostic)] pub (crate) enum MultipleDeadCodes < 'tcx > { # [diag (passes_dead_codes)] DeadCodes { multiple : bool , num : usize , descr : & 'tcx str , participle : & 'tcx str , name_list : DiagSymbolList , # [subdiagnostic] enum_variants_with_same_name : Vec < EnumVariantSameName < 'tcx > > , # [subdiagnostic] parent_info : Option < ParentInfo < 'tcx > > , # [subdiagnostic] ignored_derived_impls : Option < IgnoredDerivedImpls > , } , # [diag (passes_dead_codes)] UnusedTupleStructFields { multiple : bool , num : usize , descr : & 'tcx str , participle : & 'tcx str , name_list : DiagSymbolList , # [subdiagnostic] change_fields_suggestion : ChangeFields , # [subdiagnostic] parent_info : Option < ParentInfo < 'tcx > > , # [subdiagnostic] ignored_derived_impls : Option < IgnoredDerivedImpls > , } , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (passes_enum_variant_same_name)] pub (crate) struct EnumVariantSameName < 'tcx > { # [primary_span] pub variant_span : Span , pub dead_name : Symbol , pub dead_descr : & 'tcx str , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [label (passes_parent_info)] pub (crate) struct ParentInfo < 'tcx > { pub num : usize , pub descr : & 'tcx str , pub parent_descr : & 'tcx str , # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [note (passes_ignored_derived_impls)] pub (crate) struct IgnoredDerivedImpls { pub name : Symbol , pub trait_list : DiagSymbolList , pub trait_list_len : usize , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum ChangeFields { # [multipart_suggestion (passes_change_fields_to_be_of_unit_type , applicability = "has-placeholders")] ChangeToUnitTypeOrRemove { num : usize , # [suggestion_part (code = "()")] spans : Vec < Span > , } , # [help (passes_remove_fields)] Remove { num : usize } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_proc_macro_bad_sig)] pub (crate) struct ProcMacroBadSig { # [primary_span] pub span : Span , pub kind : ProcMacroKind , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unreachable_due_to_uninhabited)] pub (crate) struct UnreachableDueToUninhabited < 'desc , 'tcx > { pub descr : & 'desc str , # [label] pub expr : Span , # [label (passes_label_orig)] # [note] pub orig : Span , pub ty : Ty < 'tcx > , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unused_var_maybe_capture_ref)] # [help] pub (crate) struct UnusedVarMaybeCaptureRef { pub name : String , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unused_capture_maybe_capture_ref)] # [help] pub (crate) struct UnusedCaptureMaybeCaptureRef { pub name : String , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unused_var_remove_field)] pub (crate) struct UnusedVarRemoveField { pub name : String , # [subdiagnostic] pub sugg : UnusedVarRemoveFieldSugg , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (passes_unused_var_remove_field_suggestion , applicability = "machine-applicable")] pub (crate) struct UnusedVarRemoveFieldSugg { # [suggestion_part (code = "")] pub spans : Vec < Span > , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unused_var_assigned_only)] # [note] pub (crate) struct UnusedVarAssignedOnly { pub name : String , # [subdiagnostic] pub typo : Option < PatternTypo > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (passes_unused_var_typo , style = "verbose" , applicability = "machine-applicable")] pub (crate) struct PatternTypo { # [suggestion_part (code = "{code}")] pub span : Span , pub code : String , pub item_name : String , pub kind : String , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unnecessary_stable_feature)] pub (crate) struct UnnecessaryStableFeature { pub feature : Symbol , pub since : Symbol , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unnecessary_partial_stable_feature)] pub (crate) struct UnnecessaryPartialStableFeature { # [suggestion (code = "{implies}" , applicability = "maybe-incorrect")] pub span : Span , # [suggestion (passes_suggestion_remove , code = "" , applicability = "maybe-incorrect")] pub line : Span , pub feature : Symbol , pub since : Symbol , pub implies : Symbol , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_ineffective_unstable_impl)] # [note] pub (crate) struct IneffectiveUnstableImpl ;}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unused_assign)] pub (crate) struct UnusedAssign { pub name : String , # [subdiagnostic] pub suggestion : Option < UnusedAssignSuggestion > , # [help] pub help : bool , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (passes_unused_assign_suggestion , applicability = "maybe-incorrect")] pub (crate) struct UnusedAssignSuggestion { pub pre : & 'static str , # [suggestion_part (code = "{pre}mut ")] pub ty_span : Option < Span > , # [suggestion_part (code = "")] pub ty_ref_span : Span , # [suggestion_part (code = "*")] pub ident_span : Span , # [suggestion_part (code = "")] pub expr_ref_span : Span , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unused_assign_passed)] # [help] pub (crate) struct UnusedAssignPassed { pub name : String , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unused_variable_try_prefix)] pub (crate) struct UnusedVariableTryPrefix { # [label] pub label : Option < Span > , # [subdiagnostic] pub string_interp : Vec < UnusedVariableStringInterp > , # [subdiagnostic] pub sugg : UnusedVariableSugg , pub name : String , # [subdiagnostic] pub typo : Option < PatternTypo > , }}}
mkitem!{mkenum!{# [derive (Subdiagnostic)] pub (crate) enum UnusedVariableSugg { # [multipart_suggestion (passes_suggestion , applicability = "maybe-incorrect")] TryPrefixSugg { # [suggestion_part (code = "_{name}")] spans : Vec < Span > , name : String , } , # [help (passes_unused_variable_args_in_macro)] NoSugg { # [primary_span] span : Span , name : String , } , }}}
mkitem!{mkstruct!{pub (crate) struct UnusedVariableStringInterp { pub lit : Span , pub lo : Span , pub hi : Span , }}}
mkitem!{mkimpl!{impl Subdiagnostic for UnusedVariableStringInterp { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . span_label (self . lit , crate :: fluent_generated :: passes_maybe_string_interpolation) ; diag . multipart_suggestion (crate :: fluent_generated :: passes_string_interpolation_only_works , vec ! [(self . lo , String :: from ("format!(")) , (self . hi , String :: from (")"))] , Applicability :: MachineApplicable ,) ; } }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_unused_variable_try_ignore)] pub (crate) struct UnusedVarTryIgnore { pub name : String , # [subdiagnostic] pub sugg : UnusedVarTryIgnoreSugg , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [multipart_suggestion (passes_suggestion , applicability = "maybe-incorrect")] pub (crate) struct UnusedVarTryIgnoreSugg { # [suggestion_part (code = "{name}: _")] pub shorthands : Vec < Span > , # [suggestion_part (code = "_")] pub non_shorthands : Vec < Span > , pub name : String , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (passes_attr_crate_level)] # [note] pub (crate) struct AttrCrateLevelOnly { # [subdiagnostic] pub sugg : Option < AttrCrateLevelOnlySugg > , }}}
mkitem!{mkstruct!{# [derive (Subdiagnostic)] # [suggestion (passes_suggestion , applicability = "maybe-incorrect" , code = "!" , style = "verbose")] pub (crate) struct AttrCrateLevelOnlySugg { # [primary_span] pub attr : Span , }}}
mkitem!{mkstruct!{# [doc = " \"sanitize attribute not allowed here\""] # [derive (Diagnostic)] # [diag (passes_sanitize_attribute_not_allowed)] pub (crate) struct SanitizeAttributeNotAllowed { # [primary_span] pub attr_span : Span , # [doc = " \"not a function, impl block, or module\""] # [label (passes_not_fn_impl_mod)] pub not_fn_impl_mod : Option < Span > , # [doc = " \"function has no body\""] # [label (passes_no_body)] pub no_body : Option < Span > , # [doc = " \"sanitize attribute can be applied to a function (with body), impl block, or module\""] # [help] pub help : () , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_rustc_const_stable_indirect_pairing)] pub (crate) struct RustcConstStableIndirectPairing { # [primary_span] pub span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_unsupported_attributes_in_where)] # [help] pub (crate) struct UnsupportedAttributesInWhere { # [primary_span] pub span : MultiSpan , }}}
mkitem!{mkenum!{# [derive (Diagnostic)] pub (crate) enum UnexportableItem < 'a > { # [diag (passes_unexportable_item)] Item { # [primary_span] span : Span , descr : & 'a str , } , # [diag (passes_unexportable_generic_fn)] GenericFn (# [primary_span] Span) , # [diag (passes_unexportable_fn_abi)] FnAbi (# [primary_span] Span) , # [diag (passes_unexportable_type_repr)] TypeRepr (# [primary_span] Span) , # [diag (passes_unexportable_type_in_interface)] TypeInInterface { # [primary_span] span : Span , desc : & 'a str , ty : & 'a str , # [label] ty_span : Span , } , # [diag (passes_unexportable_priv_item)] PrivItem { # [primary_span] span : Span , # [note] vis_note : Span , vis_descr : & 'a str , } , # [diag (passes_unexportable_adt_with_private_fields)] AdtWithPrivFields { # [primary_span] span : Span , # [note] vis_note : Span , field_name : & 'a str , } , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_repr_align_should_be_align)] pub (crate) struct ReprAlignShouldBeAlign { # [primary_span] # [help] pub span : Span , pub item : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_repr_align_should_be_align_static)] pub (crate) struct ReprAlignShouldBeAlignStatic { # [primary_span] # [help] pub span : Span , pub item : & 'static str , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_custom_mir_phase_requires_dialect)] pub (crate) struct CustomMirPhaseRequiresDialect { # [primary_span] pub attr_span : Span , # [label] pub phase_span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (passes_custom_mir_incompatible_dialect_and_phase)] pub (crate) struct CustomMirIncompatibleDialectAndPhase { pub dialect : MirDialect , pub phase : MirPhase , # [primary_span] pub attr_span : Span , # [label] pub dialect_span : Span , # [label] pub phase_span : Span , }}}