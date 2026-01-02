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
mkuse!{use rustc_ast :: attr :: AttributeExt ;}
mkuse!{use rustc_ast_pretty :: pprust ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashSet , FxIndexMap } ;}
mkuse!{use rustc_data_structures :: unord :: UnordSet ;}
mkuse!{use rustc_errors :: { Diag , LintDiagnostic , MultiSpan } ;}
mkuse!{use rustc_feature :: { Features , GateIssue } ;}
mkuse!{use rustc_hir :: HirId ;}
mkuse!{use rustc_hir :: intravisit :: { self , Visitor } ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: hir :: nested_filter ;}
mkuse!{use rustc_middle :: lint :: { LevelAndSource , LintExpectation , LintLevelSource , ShallowLintLevelMap , lint_level , reveal_actual_level , } ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { RegisteredTools , TyCtxt } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: lint :: builtin :: { self , FORBIDDEN_LINT_GROUPS , RENAMED_AND_REMOVED_LINTS , SINGLE_USE_LIFETIMES , UNFULFILLED_LINT_EXPECTATIONS , UNKNOWN_LINTS , UNUSED_ATTRIBUTES , } ;}
mkuse!{use rustc_session :: lint :: { Level , Lint , LintExpectationId , LintId } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span , Symbol , sym } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use { rustc_ast as ast , rustc_hir as hir } ;}
mkuse!{use crate :: builtin :: MISSING_DOCS ;}
mkuse!{use crate :: context :: { CheckLintNameResult , LintStore } ;}
mkuse!{use crate :: errors :: { CheckNameUnknownTool , MalformedAttribute , MalformedAttributeSub , OverruledAttribute , OverruledAttributeSub , RequestedLevel , UnknownToolInScopedLint , UnsupportedGroup , } ;}
mkuse!{use crate :: fluent_generated as fluent ;}
mkuse!{use crate :: late :: unerased_lint_store ;}
mkuse!{use crate :: lints :: { DeprecatedLintName , DeprecatedLintNameFromCommandLine , IgnoredUnlessCrateSpecified , OverruledAttributeLint , RemovedLint , RemovedLintFromCommandLine , RenamedLint , RenamedLintFromCommandLine , RenamedLintSuggestion , UnknownLint , UnknownLintFromCommandLine , UnknownLintSuggestion , } ;}
mkitem!{mkstruct!{# [doc = " Collection of lint levels for the whole crate."] # [doc = " This is used by AST-based lints, which do not"] # [doc = " wait until we have built HIR to be emitted."] # [derive (Debug)] struct LintLevelSets { # [doc = " Linked list of specifications."] list : IndexVec < LintStackIndex , LintSet > , }}}
mkitem!{rustc_index :: newtype_index ! { struct LintStackIndex { const COMMAND_LINE = 0 ; } }}
mkitem!{mkstruct!{# [doc = " Specifications found at this position in the stack. This map only represents the lints"] # [doc = " found for one set of attributes (like `shallow_lint_levels_on` does)."] # [doc = ""] # [doc = " We store the level specifications as a linked list."] # [doc = " Each `LintSet` represents a set of attributes on the same AST node."] # [doc = " The `parent` forms a linked list that matches the AST tree."] # [doc = " This way, walking the linked list is equivalent to walking the AST bottom-up"] # [doc = " to find the specifications for a given lint."] # [derive (Debug)] struct LintSet { specs : FxIndexMap < LintId , LevelAndSource > , parent : LintStackIndex , }}}
mkitem!{mkimpl!{impl LintLevelSets { fn new () -> Self { LintLevelSets { list : IndexVec :: new () } } fn get_lint_level (& self , lint : & 'static Lint , idx : LintStackIndex , aux : Option < & FxIndexMap < LintId , LevelAndSource > > , sess : & Session ,) -> LevelAndSource { let lint = LintId :: of (lint) ; let (level , mut src) = self . raw_lint_id_level (lint , idx , aux) ; let (level , lint_id) = reveal_actual_level (level , & mut src , sess , lint , | id | { self . raw_lint_id_level (id , idx , aux) }) ; LevelAndSource { level , lint_id , src } } fn raw_lint_id_level (& self , id : LintId , mut idx : LintStackIndex , aux : Option < & FxIndexMap < LintId , LevelAndSource > > ,) -> (Option < (Level , Option < LintExpectationId >) > , LintLevelSource) { if let Some (specs) = aux && let Some (& LevelAndSource { level , lint_id , src }) = specs . get (& id) { return (Some ((level , lint_id)) , src) ; } loop { let LintSet { ref specs , parent } = self . list [idx] ; if let Some (& LevelAndSource { level , lint_id , src }) = specs . get (& id) { return (Some ((level , lint_id)) , src) ; } if idx == COMMAND_LINE { return (None , LintLevelSource :: Default) ; } idx = parent ; } } }}}

macro_rules! lints_that_dont_need_to_run_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lints_that_dont_need_to_run in module {}", module_path!());
    };
}

mkfn!{
    lints_that_dont_need_to_run_introspect!();
    fn lints_that_dont_need_to_run (tcx : TyCtxt < '_ > , () : ()) -> UnordSet < LintId > { let store = unerased_lint_store (& tcx . sess) ; let root_map = tcx . shallow_lint_levels_on (hir :: CRATE_OWNER_ID) ; let mut dont_need_to_run : FxHashSet < LintId > = store . get_lints () . into_iter () . filter (| lint | { let has_future_breakage = lint . future_incompatible . is_some_and (| fut | fut . report_in_deps) ; ! has_future_breakage && ! lint . eval_always }) . filter (| lint | { let lint_level = root_map . lint_level_id_at_node (tcx , LintId :: of (lint) , hir :: CRATE_HIR_ID) ; matches ! (lint_level . level , Level :: Allow) || (matches ! (lint_level . src , LintLevelSource :: Default) && lint . default_level (tcx . sess . edition ()) == Level :: Allow) }) . map (| lint | LintId :: of (* lint)) . collect () ; for owner in tcx . hir_crate_items (()) . owners () { let map = tcx . shallow_lint_levels_on (owner) ; for (_ , specs) in map . specs . iter () { for (lint , level_and_source) in specs . iter () { if ! matches ! (level_and_source . level , Level :: Allow) { dont_need_to_run . remove (lint) ; } } } } dont_need_to_run . into () }
}

macro_rules! shallow_lint_levels_on_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function shallow_lint_levels_on in module {}", module_path!());
    };
}

mkfn!{
    shallow_lint_levels_on_introspect!();
    # [instrument (level = "trace" , skip (tcx) , ret)] fn shallow_lint_levels_on (tcx : TyCtxt < '_ > , owner : hir :: OwnerId) -> ShallowLintLevelMap { let store = unerased_lint_store (tcx . sess) ; let attrs = tcx . hir_attr_map (owner) ; let mut levels = LintLevelsBuilder { sess : tcx . sess , features : tcx . features () , provider : LintLevelQueryMap { tcx , cur : owner . into () , specs : ShallowLintLevelMap :: default () , empty : FxIndexMap :: default () , attrs , } , lint_added_lints : false , store , registered_tools : tcx . registered_tools (()) , } ; if owner == hir :: CRATE_OWNER_ID { levels . add_command_line () ; } match attrs . map . range (..) { [] => { } & [(local_id , _)] => levels . add_id (HirId { owner , local_id }) , _ => match tcx . hir_owner_node (owner) { hir :: OwnerNode :: Item (item) => levels . visit_item (item) , hir :: OwnerNode :: ForeignItem (item) => levels . visit_foreign_item (item) , hir :: OwnerNode :: TraitItem (item) => levels . visit_trait_item (item) , hir :: OwnerNode :: ImplItem (item) => levels . visit_impl_item (item) , hir :: OwnerNode :: Crate (mod_) => { levels . add_id (hir :: CRATE_HIR_ID) ; levels . visit_mod (mod_ , mod_ . spans . inner_span , hir :: CRATE_HIR_ID) } hir :: OwnerNode :: Synthetic => unreachable ! () , } , } let specs = levels . provider . specs ; # [cfg (debug_assertions)] for (_ , v) in specs . specs . iter () { debug_assert ! (! v . is_empty ()) ; } specs }
}
mkitem!{mkstruct!{pub struct TopDown { sets : LintLevelSets , cur : LintStackIndex , }}}
mkitem!{mktrait!{pub trait LintLevelsProvider { fn current_specs (& self) -> & FxIndexMap < LintId , LevelAndSource > ; fn insert (& mut self , id : LintId , lvl : LevelAndSource) ; fn get_lint_level (& self , lint : & 'static Lint , sess : & Session) -> LevelAndSource ; fn push_expectation (& mut self , id : LintExpectationId , expectation : LintExpectation) ; }}}
mkitem!{mkimpl!{impl LintLevelsProvider for TopDown { fn current_specs (& self) -> & FxIndexMap < LintId , LevelAndSource > { & self . sets . list [self . cur] . specs } fn insert (& mut self , id : LintId , lvl : LevelAndSource) { self . sets . list [self . cur] . specs . insert (id , lvl) ; } fn get_lint_level (& self , lint : & 'static Lint , sess : & Session) -> LevelAndSource { self . sets . get_lint_level (lint , self . cur , Some (self . current_specs ()) , sess) } fn push_expectation (& mut self , _ : LintExpectationId , _ : LintExpectation) { } }}}
mkitem!{mkstruct!{struct LintLevelQueryMap < 'tcx > { tcx : TyCtxt < 'tcx > , cur : HirId , specs : ShallowLintLevelMap , # [doc = " Empty hash map to simplify code."] empty : FxIndexMap < LintId , LevelAndSource > , attrs : & 'tcx hir :: AttributeMap < 'tcx > , }}}
mkitem!{mkimpl!{impl LintLevelsProvider for LintLevelQueryMap < '_ > { fn current_specs (& self) -> & FxIndexMap < LintId , LevelAndSource > { self . specs . specs . get (& self . cur . local_id) . unwrap_or (& self . empty) } fn insert (& mut self , id : LintId , lvl : LevelAndSource) { self . specs . specs . get_mut_or_insert_default (self . cur . local_id) . insert (id , lvl) ; } fn get_lint_level (& self , lint : & 'static Lint , _ : & Session) -> LevelAndSource { self . specs . lint_level_id_at_node (self . tcx , LintId :: of (lint) , self . cur) } fn push_expectation (& mut self , id : LintExpectationId , expectation : LintExpectation) { self . specs . expectations . push ((id , expectation)) } }}}
mkitem!{mkimpl!{impl < 'tcx > LintLevelsBuilder < '_ , LintLevelQueryMap < 'tcx > > { fn add_id (& mut self , hir_id : HirId) { self . provider . cur = hir_id ; self . add (self . provider . attrs . get (hir_id . local_id) , hir_id == hir :: CRATE_HIR_ID , Some (hir_id) ,) ; } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for LintLevelsBuilder < '_ , LintLevelQueryMap < 'tcx > > { type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . provider . tcx } fn visit_param (& mut self , param : & 'tcx hir :: Param < 'tcx >) { self . add_id (param . hir_id) ; intravisit :: walk_param (self , param) ; } fn visit_item (& mut self , it : & 'tcx hir :: Item < 'tcx >) { self . add_id (it . hir_id ()) ; intravisit :: walk_item (self , it) ; } fn visit_foreign_item (& mut self , it : & 'tcx hir :: ForeignItem < 'tcx >) { self . add_id (it . hir_id ()) ; intravisit :: walk_foreign_item (self , it) ; } fn visit_stmt (& mut self , s : & 'tcx hir :: Stmt < 'tcx >) { self . add_id (s . hir_id) ; intravisit :: walk_stmt (self , s) ; } fn visit_expr (& mut self , e : & 'tcx hir :: Expr < 'tcx >) { self . add_id (e . hir_id) ; intravisit :: walk_expr (self , e) ; } fn visit_pat_field (& mut self , f : & 'tcx hir :: PatField < 'tcx >) -> Self :: Result { self . add_id (f . hir_id) ; intravisit :: walk_pat_field (self , f) ; } fn visit_expr_field (& mut self , f : & 'tcx hir :: ExprField < 'tcx >) { self . add_id (f . hir_id) ; intravisit :: walk_expr_field (self , f) ; } fn visit_field_def (& mut self , s : & 'tcx hir :: FieldDef < 'tcx >) { self . add_id (s . hir_id) ; intravisit :: walk_field_def (self , s) ; } fn visit_variant (& mut self , v : & 'tcx hir :: Variant < 'tcx >) { self . add_id (v . hir_id) ; intravisit :: walk_variant (self , v) ; } fn visit_local (& mut self , l : & 'tcx hir :: LetStmt < 'tcx >) { self . add_id (l . hir_id) ; intravisit :: walk_local (self , l) ; } fn visit_arm (& mut self , a : & 'tcx hir :: Arm < 'tcx >) { self . add_id (a . hir_id) ; intravisit :: walk_arm (self , a) ; } fn visit_trait_item (& mut self , trait_item : & 'tcx hir :: TraitItem < 'tcx >) { self . add_id (trait_item . hir_id ()) ; intravisit :: walk_trait_item (self , trait_item) ; } fn visit_impl_item (& mut self , impl_item : & 'tcx hir :: ImplItem < 'tcx >) { self . add_id (impl_item . hir_id ()) ; intravisit :: walk_impl_item (self , impl_item) ; } }}}
mkitem!{mkstruct!{pub struct LintLevelsBuilder < 's , P > { sess : & 's Session , features : & 's Features , provider : P , lint_added_lints : bool , store : & 's LintStore , registered_tools : & 's RegisteredTools , }}}
mkitem!{mkstruct!{pub (crate) struct BuilderPush { prev : LintStackIndex , }}}
mkitem!{mkimpl!{impl < 's > LintLevelsBuilder < 's , TopDown > { pub (crate) fn new (sess : & 's Session , features : & 's Features , lint_added_lints : bool , store : & 's LintStore , registered_tools : & 's RegisteredTools ,) -> Self { let mut builder = LintLevelsBuilder { sess , features , provider : TopDown { sets : LintLevelSets :: new () , cur : COMMAND_LINE } , lint_added_lints , store , registered_tools , } ; builder . process_command_line () ; assert_eq ! (builder . provider . sets . list . len () , 1) ; builder } pub fn crate_root (sess : & 's Session , features : & 's Features , lint_added_lints : bool , store : & 's LintStore , registered_tools : & 's RegisteredTools , crate_attrs : & [ast :: Attribute] ,) -> Self { let mut builder = Self :: new (sess , features , lint_added_lints , store , registered_tools) ; builder . add (crate_attrs , true , None) ; builder } fn process_command_line (& mut self) { self . provider . cur = self . provider . sets . list . push (LintSet { specs : FxIndexMap :: default () , parent : COMMAND_LINE }) ; self . add_command_line () ; } # [doc = " Pushes a list of AST lint attributes onto this context."] # [doc = ""] # [doc = " This function will return a `BuilderPush` object which should be passed"] # [doc = " to `pop` when this scope for the attributes provided is exited."] # [doc = ""] # [doc = " This function will perform a number of tasks:"] # [doc = ""] # [doc = " * It'll validate all lint-related attributes in `attrs`"] # [doc = " * It'll mark all lint-related attributes as used"] # [doc = " * Lint levels will be updated based on the attributes provided"] # [doc = " * Lint attributes are validated, e.g., a `#[forbid]` can't be switched to"] # [doc = "   `#[allow]`"] # [doc = ""] # [doc = " Don't forget to call `pop`!"] pub (crate) fn push (& mut self , attrs : & [ast :: Attribute] , is_crate_node : bool , source_hir_id : Option < HirId > ,) -> BuilderPush { let prev = self . provider . cur ; self . provider . cur = self . provider . sets . list . push (LintSet { specs : FxIndexMap :: default () , parent : prev }) ; self . add (attrs , is_crate_node , source_hir_id) ; if self . provider . current_specs () . is_empty () { self . provider . sets . list . pop () ; self . provider . cur = prev ; } BuilderPush { prev } } # [doc = " Called after `push` when the scope of a set of attributes are exited."] pub (crate) fn pop (& mut self , push : BuilderPush) { self . provider . cur = push . prev ; std :: mem :: forget (push) ; } }}}
mkitem!{mkimpl!{# [cfg (debug_assertions)] impl Drop for BuilderPush { fn drop (& mut self) { panic ! ("Found a `push` without a `pop`.") ; } }}}
mkitem!{mkimpl!{impl < 's , P : LintLevelsProvider > LintLevelsBuilder < 's , P > { pub (crate) fn sess (& self) -> & Session { self . sess } pub (crate) fn features (& self) -> & Features { self . features } fn current_specs (& self) -> & FxIndexMap < LintId , LevelAndSource > { self . provider . current_specs () } fn insert (& mut self , id : LintId , lvl : LevelAndSource) { self . provider . insert (id , lvl) } fn add_command_line (& mut self) { for & (ref lint_name , level) in & self . sess . opts . lint_opts { let (tool_name , lint_name_only) = parse_lint_and_tool_name (lint_name) ; if lint_name_only == crate :: WARNINGS . name_lower () && matches ! (level , Level :: ForceWarn) { self . sess . dcx () . emit_err (UnsupportedGroup { lint_group : crate :: WARNINGS . name_lower () }) ; } match self . store . check_lint_name (lint_name_only , tool_name , self . registered_tools) { CheckLintNameResult :: Renamed (ref replace) => { let name = lint_name . as_str () ; let suggestion = RenamedLintSuggestion :: WithoutSpan { replace } ; let requested_level = RequestedLevel { level , lint_name } ; let lint = RenamedLintFromCommandLine { name , replace , suggestion , requested_level } ; self . emit_lint (RENAMED_AND_REMOVED_LINTS , lint) ; } CheckLintNameResult :: Removed (ref reason) => { let name = lint_name . as_str () ; let requested_level = RequestedLevel { level , lint_name } ; let lint = RemovedLintFromCommandLine { name , reason , requested_level } ; self . emit_lint (RENAMED_AND_REMOVED_LINTS , lint) ; } CheckLintNameResult :: NoLint (suggestion) => { let name = lint_name . clone () ; let suggestion = suggestion . map (| (replace , from_rustc) | { UnknownLintSuggestion :: WithoutSpan { replace , from_rustc } }) ; let requested_level = RequestedLevel { level , lint_name } ; let lint = UnknownLintFromCommandLine { name , suggestion , requested_level } ; self . emit_lint (UNKNOWN_LINTS , lint) ; } CheckLintNameResult :: Tool (_ , Some (ref replace)) => { let name = lint_name . clone () ; let requested_level = RequestedLevel { level , lint_name } ; let lint = DeprecatedLintNameFromCommandLine { name , replace , requested_level } ; self . emit_lint (RENAMED_AND_REMOVED_LINTS , lint) ; } CheckLintNameResult :: NoTool => { self . sess . dcx () . emit_err (CheckNameUnknownTool { tool_name : tool_name . unwrap () , sub : RequestedLevel { level , lint_name } , }) ; } _ => { } } ; let lint_flag_val = Symbol :: intern (lint_name) ; let Some (ids) = self . store . find_lints (lint_name) else { continue ; } ; for & id in ids { if let Some (LevelAndSource { level : Level :: ForceWarn | Level :: Forbid , .. }) = self . current_specs () . get (& id) { continue ; } if self . check_gated_lint (id , DUMMY_SP , true) { let src = LintLevelSource :: CommandLine (lint_flag_val , level) ; self . insert (id , LevelAndSource { level , lint_id : None , src }) ; } } } } # [doc = " Attempts to insert the `id` to `level_src` map entry. If unsuccessful"] # [doc = " (e.g. if a forbid was already inserted on the same scope), then emits a"] # [doc = " diagnostic with no change to `specs`."] fn insert_spec (& mut self , id : LintId , LevelAndSource { level , lint_id , src } : LevelAndSource) { let LevelAndSource { level : old_level , src : old_src , .. } = self . provider . get_lint_level (id . lint , self . sess) ; if self . lint_added_lints && level == Level :: Deny && old_level == Level :: Forbid { return ; } else if self . lint_added_lints && level != Level :: Forbid && old_level == Level :: Forbid { let id_name = id . lint . name_lower () ; let fcw_warning = match old_src { LintLevelSource :: Default => false , LintLevelSource :: Node { name , .. } => self . store . is_lint_group (name) , LintLevelSource :: CommandLine (symbol , _) => self . store . is_lint_group (symbol) , } ; debug ! ("fcw_warning={:?}, specs.get(&id) = {:?}, old_src={:?}, id_name={:?}" , fcw_warning , self . current_specs () , old_src , id_name) ; let sub = match old_src { LintLevelSource :: Default => { OverruledAttributeSub :: DefaultSource { id : id . to_string () } } LintLevelSource :: Node { span , reason , .. } => { OverruledAttributeSub :: NodeSource { span , reason } } LintLevelSource :: CommandLine (_ , _) => OverruledAttributeSub :: CommandLineSource , } ; if ! fcw_warning { self . sess . dcx () . emit_err (OverruledAttribute { span : src . span () , overruled : src . span () , lint_level : level . as_str () , lint_source : src . name () , sub , }) ; } else { self . emit_span_lint (FORBIDDEN_LINT_GROUPS , src . span () . into () , OverruledAttributeLint { overruled : src . span () , lint_level : level . as_str () , lint_source : src . name () , sub , } ,) ; } if ! fcw_warning { return ; } } if let Level :: Expect = level && id == LintId :: of (UNFULFILLED_LINT_EXPECTATIONS) { return ; } match (old_level , level) { (Level :: ForceWarn , Level :: Expect) => { self . insert (id , LevelAndSource { level : Level :: ForceWarn , lint_id , src : old_src }) } (Level :: ForceWarn , _) => self . insert (id , LevelAndSource { level : Level :: ForceWarn , lint_id : None , src : old_src } ,) , _ => self . insert (id , LevelAndSource { level , lint_id , src }) , } ; } fn add (& mut self , attrs : & [impl AttributeExt] , is_crate_node : bool , source_hir_id : Option < HirId > ,) { let sess = self . sess ; for (attr_index , attr) in attrs . iter () . enumerate () { if attr . is_automatically_derived_attr () { self . insert (LintId :: of (SINGLE_USE_LIFETIMES) , LevelAndSource { level : Level :: Allow , lint_id : None , src : LintLevelSource :: Default , } ,) ; continue ; } if attr . has_name (sym :: doc) && attr . meta_item_list () . is_some_and (| l | ast :: attr :: list_contains_name (& l , sym :: hidden)) { self . insert (LintId :: of (MISSING_DOCS) , LevelAndSource { level : Level :: Allow , lint_id : None , src : LintLevelSource :: Default , } ,) ; continue ; } let (level , lint_id) = match Level :: from_attr (attr) { None => continue , Some ((Level :: Expect , Some (unstable_id))) if let Some (hir_id) = source_hir_id => { let LintExpectationId :: Unstable { lint_index : None , attr_id : _ } = unstable_id else { bug ! ("stable id Level::from_attr") } ; let stable_id = LintExpectationId :: Stable { hir_id , attr_index : attr_index . try_into () . unwrap () , lint_index : None , } ; (Level :: Expect , Some (stable_id)) } Some ((lvl , id)) => (lvl , id) , } ; let Some (mut metas) = attr . meta_item_list () else { continue } ; let Some (tail_li) = metas . last () else { continue ; } ; let mut reason = None ; if let Some (item) = tail_li . meta_item () { match item . kind { ast :: MetaItemKind :: Word => { } ast :: MetaItemKind :: NameValue (ref name_value) => { if item . path == sym :: reason { if let ast :: LitKind :: Str (rationale , _) = name_value . kind { reason = Some (rationale) ; } else { sess . dcx () . emit_err (MalformedAttribute { span : name_value . span , sub : MalformedAttributeSub :: ReasonMustBeStringLiteral (name_value . span ,) , }) ; } metas . pop () . unwrap () ; } else { sess . dcx () . emit_err (MalformedAttribute { span : item . span , sub : MalformedAttributeSub :: BadAttributeArgument (item . span) , }) ; } } ast :: MetaItemKind :: List (_) => { sess . dcx () . emit_err (MalformedAttribute { span : item . span , sub : MalformedAttributeSub :: BadAttributeArgument (item . span) , }) ; } } } for (lint_index , li) in metas . iter_mut () . enumerate () { let mut lint_id = lint_id ; if let Some (id) = & mut lint_id { id . set_lint_index (Some (lint_index as u16)) ; } let sp = li . span () ; let meta_item = match li { ast :: MetaItemInner :: MetaItem (meta_item) if meta_item . is_word () => meta_item , _ => { let sub = if let Some (item) = li . meta_item () && let ast :: MetaItemKind :: NameValue (_) = item . kind && item . path == sym :: reason { MalformedAttributeSub :: ReasonMustComeLast (sp) } else { MalformedAttributeSub :: BadAttributeArgument (sp) } ; sess . dcx () . emit_err (MalformedAttribute { span : sp , sub }) ; continue ; } } ; let tool_ident = if meta_item . path . segments . len () > 1 { Some (meta_item . path . segments . remove (0) . ident) } else { None } ; let tool_name = tool_ident . map (| ident | ident . name) ; let name = pprust :: path_to_string (& meta_item . path) ; let lint_result = self . store . check_lint_name (& name , tool_name , self . registered_tools) ; let (ids , name) = match lint_result { CheckLintNameResult :: Ok (ids) => { let name = meta_item . path . segments . last () . expect ("empty lint name") . ident . name ; (ids , name) } CheckLintNameResult :: Tool (ids , new_lint_name) => { let name = match new_lint_name { None => { let complete_name = & format ! ("{}::{}" , tool_ident . unwrap () . name , name) ; Symbol :: intern (complete_name) } Some (new_lint_name) => { self . emit_span_lint (builtin :: RENAMED_AND_REMOVED_LINTS , sp . into () , DeprecatedLintName { name , suggestion : sp , replace : & new_lint_name , } ,) ; Symbol :: intern (& new_lint_name) } } ; (ids , name) } CheckLintNameResult :: MissingTool => { continue ; } CheckLintNameResult :: NoTool => { sess . dcx () . emit_err (UnknownToolInScopedLint { span : tool_ident . map (| ident | ident . span) , tool_name : tool_name . unwrap () , lint_name : pprust :: path_to_string (& meta_item . path) , is_nightly_build : sess . is_nightly_build () , }) ; continue ; } CheckLintNameResult :: Renamed (ref replace) => { if self . lint_added_lints { let suggestion = RenamedLintSuggestion :: WithSpan { suggestion : sp , replace } ; let name = tool_ident . map (| tool | format ! ("{tool}::{name}")) . unwrap_or (name) ; let lint = RenamedLint { name : name . as_str () , replace , suggestion } ; self . emit_span_lint (RENAMED_AND_REMOVED_LINTS , sp . into () , lint) ; } let CheckLintNameResult :: Ok (ids) = self . store . check_lint_name (replace , None , self . registered_tools) else { panic ! ("renamed lint does not exist: {replace}") ; } ; (ids , Symbol :: intern (& replace)) } CheckLintNameResult :: Removed (ref reason) => { if self . lint_added_lints { let name = tool_ident . map (| tool | format ! ("{tool}::{name}")) . unwrap_or (name) ; let lint = RemovedLint { name : name . as_str () , reason } ; self . emit_span_lint (RENAMED_AND_REMOVED_LINTS , sp . into () , lint) ; } continue ; } CheckLintNameResult :: NoLint (suggestion) => { if self . lint_added_lints { let name = tool_ident . map (| tool | format ! ("{tool}::{name}")) . unwrap_or (name) ; let suggestion = suggestion . map (| (replace , from_rustc) | { UnknownLintSuggestion :: WithSpan { suggestion : sp , replace , from_rustc , } }) ; let lint = UnknownLint { name , suggestion } ; self . emit_span_lint (UNKNOWN_LINTS , sp . into () , lint) ; } continue ; } } ; let src = LintLevelSource :: Node { name , span : sp , reason } ; for & id in ids { if self . check_gated_lint (id , sp , false) { self . insert_spec (id , LevelAndSource { level , lint_id , src }) ; } } if let (Level :: Expect , Some (expect_id)) = (level , lint_id) { let is_unfulfilled_lint_expectations = match ids { [lint] => * lint == LintId :: of (UNFULFILLED_LINT_EXPECTATIONS) , _ => false , } ; self . provider . push_expectation (expect_id , LintExpectation :: new (reason , sp , is_unfulfilled_lint_expectations , tool_name ,) ,) ; } } } if self . lint_added_lints && ! is_crate_node { for (id , & LevelAndSource { level , ref src , .. }) in self . current_specs () . iter () { if ! id . lint . crate_level_only { continue ; } let LintLevelSource :: Node { name : lint_attr_name , span : lint_attr_span , .. } = * src else { continue ; } ; self . emit_span_lint (UNUSED_ATTRIBUTES , lint_attr_span . into () , IgnoredUnlessCrateSpecified { level : level . as_str () , name : lint_attr_name } ,) ; break ; } } } # [doc = " Checks if the lint is gated on a feature that is not enabled."] # [doc = ""] # [doc = " Returns `true` if the lint's feature is enabled."] # [track_caller] fn check_gated_lint (& self , lint_id : LintId , span : Span , lint_from_cli : bool) -> bool { let feature = if let Some (feature) = lint_id . lint . feature_gate && ! self . features . enabled (feature) && ! span . allows_unstable (feature) { feature } else { return true ; } ; if self . lint_added_lints { let lint = builtin :: UNKNOWN_LINTS ; let level = self . lint_level (builtin :: UNKNOWN_LINTS) ; # [allow (rustc :: diagnostic_outside_of_impl)] lint_level (self . sess , lint , level , Some (span . into ()) , | lint | { lint . primary_message (fluent :: lint_unknown_gated_lint) ; lint . arg ("name" , lint_id . lint . name_lower ()) ; lint . note (fluent :: lint_note) ; rustc_session :: parse :: add_feature_diagnostics_for_issue (lint , & self . sess , feature , GateIssue :: Language , lint_from_cli , None ,) ; }) ; } false } # [doc = " Find the lint level for a lint."] pub fn lint_level (& self , lint : & 'static Lint) -> LevelAndSource { self . provider . get_lint_level (lint , self . sess) } # [doc = " Used to emit a lint-related diagnostic based on the current state of"] # [doc = " this lint context."] # [doc = ""] # [doc = " [`lint_level`]: rustc_middle::lint::lint_level#decorate-signature"] # [rustc_lint_diagnostics] # [track_caller] pub (crate) fn opt_span_lint (& self , lint : & 'static Lint , span : Option < MultiSpan > , decorate : impl for < 'a , 'b > FnOnce (& 'b mut Diag < 'a , () >) ,) { let level = self . lint_level (lint) ; lint_level (self . sess , lint , level , span , decorate) } # [track_caller] pub fn emit_span_lint (& self , lint : & 'static Lint , span : MultiSpan , decorate : impl for < 'a > LintDiagnostic < 'a , () > ,) { let level = self . lint_level (lint) ; lint_level (self . sess , lint , level , Some (span) , | lint | { decorate . decorate_lint (lint) ; }) ; } # [track_caller] pub fn emit_lint (& self , lint : & 'static Lint , decorate : impl for < 'a > LintDiagnostic < 'a , () >) { let level = self . lint_level (lint) ; lint_level (self . sess , lint , level , None , | lint | { decorate . decorate_lint (lint) ; }) ; } }}}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { shallow_lint_levels_on , lints_that_dont_need_to_run , .. * providers } ; }
}

macro_rules! parse_lint_and_tool_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_lint_and_tool_name in module {}", module_path!());
    };
}

mkfn!{
    parse_lint_and_tool_name_introspect!();
    pub (crate) fn parse_lint_and_tool_name (lint_name : & str) -> (Option < Symbol > , & str) { match lint_name . split_once ("::") { Some ((tool_name , lint_name)) => { let tool_name = Symbol :: intern (tool_name) ; (Some (tool_name) , lint_name) } None => (None , lint_name) , } }
}