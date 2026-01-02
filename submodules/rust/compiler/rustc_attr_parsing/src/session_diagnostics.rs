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
mkuse!{use std :: num :: IntErrorKind ;}
mkuse!{use rustc_ast :: { self as ast , AttrStyle , Path } ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Applicability , Diag , DiagArgValue , DiagCtxtHandle , Diagnostic , EmissionGuarantee , Level , } ;}
mkuse!{use rustc_feature :: AttributeTemplate ;}
mkuse!{use rustc_hir :: { AttrPath , Target } ;}
mkuse!{use rustc_macros :: { Diagnostic , LintDiagnostic , Subdiagnostic } ;}
mkuse!{use rustc_span :: { Span , Symbol } ;}
mkuse!{use crate :: fluent_generated as fluent ;}
mkitem!{mkenum!{pub (crate) enum UnsupportedLiteralReason { Generic , CfgString , CfgBoolean , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_expected_one_cfg_pattern , code = E0536)] pub (crate) struct ExpectedOneCfgPattern { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_invalid_predicate , code = E0537)] pub (crate) struct InvalidPredicate { #[primary_span] pub span : Span , pub predicate : String , }}}
mkitem!{mkstruct!{#[doc = " Error code: E0541"] pub (crate) struct UnknownMetaItem < 'a > { pub span : Span , pub item : String , pub expected : & 'a [& 'a str] , }}}
mkitem!{mkimpl!{impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for UnknownMetaItem < '_ > { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let expected = self . expected . iter () . map (| name | format ! ("`{name}`")) . collect :: < Vec < _ > > () ; Diag :: new (dcx , level , fluent :: attr_parsing_unknown_meta_item) . with_span (self . span) . with_code (E0541) . with_arg ("item" , self . item) . with_arg ("expected" , expected . join (", ")) . with_span_label (self . span , fluent :: attr_parsing_label) } }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_missing_since , code = E0542)] pub (crate) struct MissingSince { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_missing_note , code = E0543)] pub (crate) struct MissingNote { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_multiple_stability_levels , code = E0544)] pub (crate) struct MultipleStabilityLevels { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_invalid_issue_string , code = E0545)] pub (crate) struct InvalidIssueString { #[primary_span] pub span : Span , #[subdiagnostic] pub cause : Option < InvalidIssueStringCause > , }}}
mkitem!{mkenum!{#[derive (Subdiagnostic)] pub (crate) enum InvalidIssueStringCause { #[label (attr_parsing_must_not_be_zero)] MustNotBeZero { #[primary_span] span : Span , } , #[label (attr_parsing_empty)] Empty { #[primary_span] span : Span , } , #[label (attr_parsing_invalid_digit)] InvalidDigit { #[primary_span] span : Span , } , #[label (attr_parsing_pos_overflow)] PosOverflow { #[primary_span] span : Span , } , #[label (attr_parsing_neg_overflow)] NegOverflow { #[primary_span] span : Span , } , }}}
mkitem!{mkimpl!{impl InvalidIssueStringCause { pub (crate) fn from_int_error_kind (span : Span , kind : & IntErrorKind) -> Option < Self > { match kind { IntErrorKind :: Empty => Some (Self :: Empty { span }) , IntErrorKind :: InvalidDigit => Some (Self :: InvalidDigit { span }) , IntErrorKind :: PosOverflow => Some (Self :: PosOverflow { span }) , IntErrorKind :: NegOverflow => Some (Self :: NegOverflow { span }) , IntErrorKind :: Zero => Some (Self :: MustNotBeZero { span }) , _ => None , } } }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_missing_feature , code = E0546)] pub (crate) struct MissingFeature { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_non_ident_feature , code = E0546)] pub (crate) struct NonIdentFeature { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_missing_issue , code = E0547)] pub (crate) struct MissingIssue { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_incorrect_repr_format_packed_one_or_zero_arg , code = E0552)] pub (crate) struct IncorrectReprFormatPackedOneOrZeroArg { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_incorrect_repr_format_packed_expect_integer , code = E0552)] pub (crate) struct IncorrectReprFormatPackedExpectInteger { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_invalid_repr_hint_no_paren , code = E0552)] pub (crate) struct InvalidReprHintNoParen { #[primary_span] pub span : Span , pub name : Symbol , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_invalid_repr_hint_no_value , code = E0552)] pub (crate) struct InvalidReprHintNoValue { #[primary_span] pub span : Span , pub name : Symbol , }}}
mkitem!{mkstruct!{#[doc = " Error code: E0565"] pub (crate) struct UnsupportedLiteral { pub span : Span , pub reason : UnsupportedLiteralReason , pub is_bytestr : bool , pub start_point_span : Span , }}}
mkitem!{mkimpl!{impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for UnsupportedLiteral { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let mut diag = Diag :: new (dcx , level , match self . reason { UnsupportedLiteralReason :: Generic => { fluent :: attr_parsing_unsupported_literal_generic } UnsupportedLiteralReason :: CfgString => { fluent :: attr_parsing_unsupported_literal_cfg_string } UnsupportedLiteralReason :: CfgBoolean => { fluent :: attr_parsing_unsupported_literal_cfg_boolean } } ,) ; diag . span (self . span) ; diag . code (E0565) ; if self . is_bytestr { diag . span_suggestion (self . start_point_span , fluent :: attr_parsing_unsupported_literal_suggestion , "" , Applicability :: MaybeIncorrect ,) ; } diag } }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_invalid_repr_align_need_arg , code = E0589)] pub (crate) struct InvalidReprAlignNeedArg { #[primary_span] #[suggestion (code = "align(...)" , applicability = "has-placeholders")] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_invalid_repr_generic , code = E0589)] pub (crate) struct InvalidReprGeneric < 'a > { #[primary_span] pub span : Span , pub repr_arg : String , pub error_part : & 'a str , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_incorrect_repr_format_align_one_arg , code = E0693)] pub (crate) struct IncorrectReprFormatAlignOneArg { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_incorrect_repr_format_expect_literal_integer , code = E0693)] pub (crate) struct IncorrectReprFormatExpectInteger { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_incorrect_repr_format_generic , code = E0693)] pub (crate) struct IncorrectReprFormatGeneric { #[primary_span] pub span : Span , pub repr_arg : Symbol , #[subdiagnostic] pub cause : Option < IncorrectReprFormatGenericCause > , }}}
mkitem!{mkenum!{#[derive (Subdiagnostic)] pub (crate) enum IncorrectReprFormatGenericCause { #[suggestion (attr_parsing_suggestion , code = "{name}({value})" , applicability = "machine-applicable")] Int { #[primary_span] span : Span , #[skip_arg] name : Symbol , #[skip_arg] value : u128 , } , #[suggestion (attr_parsing_suggestion , code = "{name}({value})" , applicability = "machine-applicable")] Symbol { #[primary_span] span : Span , #[skip_arg] name : Symbol , #[skip_arg] value : Symbol , } , }}}
mkitem!{mkimpl!{impl IncorrectReprFormatGenericCause { pub (crate) fn from_lit_kind (span : Span , kind : & ast :: LitKind , name : Symbol) -> Option < Self > { match * kind { ast :: LitKind :: Int (value , ast :: LitIntType :: Unsuffixed) => { Some (Self :: Int { span , name , value : value . get () }) } ast :: LitKind :: Str (value , _) => Some (Self :: Symbol { span , name , value }) , _ => None , } } }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_rustc_promotable_pairing , code = E0717)] pub (crate) struct RustcPromotablePairing { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_rustc_allowed_unstable_pairing , code = E0789)] pub (crate) struct RustcAllowedUnstablePairing { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_cfg_predicate_identifier)] pub (crate) struct CfgPredicateIdentifier { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_deprecated_item_suggestion)] pub (crate) struct DeprecatedItemSuggestion { #[primary_span] pub span : Span , #[help] pub is_nightly : bool , #[note] pub details : () , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_expected_single_version_literal)] pub (crate) struct ExpectedSingleVersionLiteral { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_expected_version_literal)] pub (crate) struct ExpectedVersionLiteral { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_expects_feature_list)] pub (crate) struct ExpectsFeatureList { #[primary_span] pub span : Span , pub name : String , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_expects_features)] pub (crate) struct ExpectsFeatures { #[primary_span] pub span : Span , pub name : String , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_invalid_since)] pub (crate) struct InvalidSince { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_soft_no_args)] pub (crate) struct SoftNoArgs { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_unknown_version_literal)] pub (crate) struct UnknownVersionLiteral { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_unused_multiple)] pub (crate) struct UnusedMultiple { #[primary_span] #[suggestion (code = "" , applicability = "machine-applicable")] pub this : Span , #[note] pub other : Span , pub name : Symbol , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (attr_parsing_unused_duplicate)] pub (crate) struct UnusedDuplicate { #[suggestion (code = "" , applicability = "machine-applicable")] pub this : Span , #[note] pub other : Span , #[warning] pub warning : bool , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (attr_parsing_ill_formed_attribute_input)] pub (crate) struct IllFormedAttributeInput { pub num_suggestions : usize , pub suggestions : DiagArgValue , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_ill_formed_attribute_input)] pub (crate) struct IllFormedAttributeInputLint { #[primary_span] pub span : Span , pub num_suggestions : usize , pub suggestions : DiagArgValue , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_null_on_export , code = E0648)] pub (crate) struct NullOnExport { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_null_on_link_section , code = E0648)] pub (crate) struct NullOnLinkSection { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_stability_outside_std , code = E0734)] pub (crate) struct StabilityOutsideStd { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_empty_confusables)] pub (crate) struct EmptyConfusables { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (attr_parsing_empty_attribute)] pub (crate) struct EmptyAttributeList { #[suggestion (code = "" , applicability = "machine-applicable")] pub attr_span : Span , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (attr_parsing_invalid_target_lint)] #[warning] #[help] pub (crate) struct InvalidTargetLint { pub name : AttrPath , pub target : & 'static str , pub applied : DiagArgValue , pub only : & 'static str , #[suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub attr_span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[help] #[diag (attr_parsing_invalid_target)] pub (crate) struct InvalidTarget { #[primary_span] #[suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub span : Span , pub name : AttrPath , pub target : & 'static str , pub applied : DiagArgValue , pub only : & 'static str , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_invalid_alignment_value , code = E0589)] pub (crate) struct InvalidAlignmentValue { #[primary_span] pub span : Span , pub error_part : & 'static str , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_repr_ident , code = E0565)] pub (crate) struct ReprIdent { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_unrecognized_repr_hint , code = E0552)] #[help] #[note] pub (crate) struct UnrecognizedReprHint { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_unstable_feature_bound_incompatible_stability)] #[help] pub (crate) struct UnstableFeatureBoundIncompatibleStability { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_naked_functions_incompatible_attribute , code = E0736)] pub (crate) struct NakedFunctionIncompatibleAttribute { #[primary_span] #[label] pub span : Span , #[label (attr_parsing_naked_attribute)] pub naked_span : Span , pub attr : String , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_link_ordinal_out_of_range)] #[note] pub (crate) struct LinkOrdinalOutOfRange { #[primary_span] pub span : Span , pub ordinal : u128 , }}}
mkitem!{mkenum!{pub (crate) enum AttributeParseErrorReason < 'a > { ExpectedNoArgs , ExpectedStringLiteral { byte_string : Option < Span > , } , ExpectedIntegerLiteral , ExpectedAtLeastOneArgument , ExpectedSingleArgument , ExpectedList , UnexpectedLiteral , ExpectedNameValue (Option < Symbol >) , DuplicateKey (Symbol) , ExpectedSpecificArgument { possibilities : & 'a [Symbol] , strings : bool , #[doc = " Should we tell the user to write a list when they didn't?"] list : bool , } , ExpectedIdentifier , }}}
mkitem!{mkstruct!{pub (crate) struct AttributeParseError < 'a > { pub (crate) span : Span , pub (crate) attr_span : Span , pub (crate) attr_style : AttrStyle , pub (crate) template : AttributeTemplate , pub (crate) attribute : AttrPath , pub (crate) reason : AttributeParseErrorReason < 'a > , }}}
mkitem!{mkimpl!{impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for AttributeParseError < '_ > { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let name = self . attribute . to_string () ; let mut diag = Diag :: new (dcx , level , format ! ("malformed `{name}` attribute input")) ; diag . span (self . attr_span) ; diag . code (E0539) ; match self . reason { AttributeParseErrorReason :: ExpectedStringLiteral { byte_string } => { if let Some (start_point_span) = byte_string { diag . span_suggestion (start_point_span , fluent :: attr_parsing_unsupported_literal_suggestion , "" , Applicability :: MaybeIncorrect ,) ; diag . note ("expected a normal string literal, not a byte string literal") ; return diag ; } else { diag . span_label (self . span , "expected a string literal here") ; } } AttributeParseErrorReason :: ExpectedIntegerLiteral => { diag . span_label (self . span , "expected an integer literal here") ; } AttributeParseErrorReason :: ExpectedSingleArgument => { diag . span_label (self . span , "expected a single argument here") ; diag . code (E0805) ; } AttributeParseErrorReason :: ExpectedAtLeastOneArgument => { diag . span_label (self . span , "expected at least 1 argument here") ; } AttributeParseErrorReason :: ExpectedList => { diag . span_label (self . span , "expected this to be a list") ; } AttributeParseErrorReason :: DuplicateKey (key) => { diag . span_label (self . span , format ! ("found `{key}` used as a key more than once")) ; diag . code (E0538) ; } AttributeParseErrorReason :: UnexpectedLiteral => { diag . span_label (self . span , "didn't expect a literal here") ; diag . code (E0565) ; } AttributeParseErrorReason :: ExpectedNoArgs => { diag . span_label (self . span , "didn't expect any arguments here") ; diag . code (E0565) ; } AttributeParseErrorReason :: ExpectedNameValue (None) => { if self . span != self . attr_span { diag . span_label (self . span , format ! ("expected this to be of the form `... = \"...\"`") ,) ; } } AttributeParseErrorReason :: ExpectedNameValue (Some (name)) => { diag . span_label (self . span , format ! ("expected this to be of the form `{name} = \"...\"`") ,) ; } AttributeParseErrorReason :: ExpectedSpecificArgument { possibilities , strings , list : false , } => { let quote = if strings { '"' } else { '`' } ; match possibilities { & [] => { } & [x] => { diag . span_label (self . span , format ! ("the only valid argument here is {quote}{x}{quote}") ,) ; } [first , second] => { diag . span_label (self . span , format ! ("valid arguments are {quote}{first}{quote} or {quote}{second}{quote}")) ; } [first @ .. , second_to_last , last] => { let mut res = String :: new () ; for i in first { res . push_str (& format ! ("{quote}{i}{quote}, ")) ; } res . push_str (& format ! ("{quote}{second_to_last}{quote} or {quote}{last}{quote}")) ; diag . span_label (self . span , format ! ("valid arguments are {res}")) ; } } } AttributeParseErrorReason :: ExpectedSpecificArgument { possibilities , strings , list : true , } => { let quote = if strings { '"' } else { '`' } ; match possibilities { & [] => { } & [x] => { diag . span_label (self . span , format ! ("this attribute is only valid with {quote}{x}{quote} as an argument") ,) ; } [first , second] => { diag . span_label (self . span , format ! ("this attribute is only valid with either {quote}{first}{quote} or {quote}{second}{quote} as an argument")) ; } [first @ .. , second_to_last , last] => { let mut res = String :: new () ; for i in first { res . push_str (& format ! ("{quote}{i}{quote}, ")) ; } res . push_str (& format ! ("{quote}{second_to_last}{quote} or {quote}{last}{quote}")) ; diag . span_label (self . span , format ! ("this attribute is only valid with one of the following arguments: {res}")) ; } } } AttributeParseErrorReason :: ExpectedIdentifier => { diag . span_label (self . span , "expected a valid identifier here") ; } } if let Some (link) = self . template . docs { diag . note (format ! ("for more information, visit <{link}>")) ; } let suggestions = self . template . suggestions (self . attr_style , & name) ; diag . span_suggestions (self . attr_span , if suggestions . len () == 1 { "must be of the form" } else { "try changing it to one of the following valid forms of the attribute" } , suggestions , Applicability :: HasPlaceholders ,) ; diag } }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_invalid_attr_unsafe)] #[note] pub (crate) struct InvalidAttrUnsafe { #[primary_span] #[label] pub span : Span , pub name : Path , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_unsafe_attr_outside_unsafe)] pub (crate) struct UnsafeAttrOutsideUnsafe { #[primary_span] #[label] pub span : Span , #[subdiagnostic] pub suggestion : UnsafeAttrOutsideUnsafeSuggestion , }}}
mkitem!{mkstruct!{#[derive (Subdiagnostic)] #[multipart_suggestion (attr_parsing_unsafe_attr_outside_unsafe_suggestion , applicability = "machine-applicable")] pub (crate) struct UnsafeAttrOutsideUnsafeSuggestion { #[suggestion_part (code = "unsafe(")] pub left : Span , #[suggestion_part (code = ")")] pub right : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_meta_bad_delim)] pub (crate) struct MetaBadDelim { #[primary_span] pub span : Span , #[subdiagnostic] pub sugg : MetaBadDelimSugg , }}}
mkitem!{mkstruct!{#[derive (Subdiagnostic)] #[multipart_suggestion (attr_parsing_meta_bad_delim_suggestion , applicability = "machine-applicable")] pub (crate) struct MetaBadDelimSugg { #[suggestion_part (code = "(")] pub open : Span , #[suggestion_part (code = ")")] pub close : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_invalid_meta_item)] pub (crate) struct InvalidMetaItem { #[primary_span] pub span : Span , pub descr : String , #[subdiagnostic] pub quote_ident_sugg : Option < InvalidMetaItemQuoteIdentSugg > , #[subdiagnostic] pub remove_neg_sugg : Option < InvalidMetaItemRemoveNegSugg > , }}}
mkitem!{mkstruct!{#[derive (Subdiagnostic)] #[multipart_suggestion (attr_parsing_quote_ident_sugg , applicability = "machine-applicable")] pub (crate) struct InvalidMetaItemQuoteIdentSugg { #[suggestion_part (code = "\"")] pub before : Span , #[suggestion_part (code = "\"")] pub after : Span , }}}
mkitem!{mkstruct!{#[derive (Subdiagnostic)] #[multipart_suggestion (attr_parsing_remove_neg_sugg , applicability = "machine-applicable")] pub (crate) struct InvalidMetaItemRemoveNegSugg { #[suggestion_part (code = "")] pub negative_sign : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_suffixed_literal_in_attribute)] #[help] pub (crate) struct SuffixedLiteralInAttribute { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (attr_parsing_invalid_style)] pub (crate) struct InvalidAttrStyle { pub name : AttrPath , pub is_used_as_inner : bool , #[note] pub target_span : Option < Span > , pub target : Target , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_empty_link_name , code = E0454)] pub (crate) struct EmptyLinkName { #[primary_span] #[label] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_link_framework_apple , code = E0455)] pub (crate) struct LinkFrameworkApple { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_incompatible_wasm_link)] pub (crate) struct IncompatibleWasmLink { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_link_requires_name , code = E0459)] pub (crate) struct LinkRequiresName { #[primary_span] #[label] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_raw_dylib_no_nul)] pub (crate) struct RawDylibNoNul { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_raw_dylib_only_windows , code = E0455)] pub (crate) struct RawDylibOnlyWindows { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_invalid_link_modifier)] pub (crate) struct InvalidLinkModifier { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_multiple_modifiers)] pub (crate) struct MultipleModifiers { #[primary_span] pub span : Span , pub modifier : Symbol , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_import_name_type_x86)] pub (crate) struct ImportNameTypeX86 { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_bundle_needs_static)] pub (crate) struct BundleNeedsStatic { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_whole_archive_needs_static)] pub (crate) struct WholeArchiveNeedsStatic { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_as_needed_compatibility)] pub (crate) struct AsNeededCompatibility { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_import_name_type_raw)] pub (crate) struct ImportNameTypeRaw { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (attr_parsing_limit_invalid)] pub (crate) struct LimitInvalid < 'a > { #[primary_span] pub span : Span , #[label] pub value_span : Span , pub error_str : & 'a str , }}}