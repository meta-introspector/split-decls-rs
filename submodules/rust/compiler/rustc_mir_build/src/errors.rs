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
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: { Applicability , Diag , DiagArgValue , DiagCtxtHandle , Diagnostic , EmissionGuarantee , Level , MultiSpan , Subdiagnostic , pluralize , } ;}
mkuse!{use rustc_macros :: { Diagnostic , LintDiagnostic , Subdiagnostic } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty } ;}
mkuse!{use rustc_pattern_analysis :: errors :: Uncovered ;}
mkuse!{use rustc_pattern_analysis :: rustc :: RustcPatCtxt ;}
mkuse!{use rustc_span :: { Ident , Span , Symbol } ;}
mkuse!{use crate :: fluent_generated as fluent ;}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_call_to_deprecated_safe_fn_requires_unsafe)] pub (crate) struct CallToDeprecatedSafeFnRequiresUnsafe { #[label] pub (crate) span : Span , pub (crate) function : String , pub (crate) guarantee : String , #[subdiagnostic] pub (crate) sub : CallToDeprecatedSafeFnRequiresUnsafeSub , }}}
mkitem!{mkstruct!{#[derive (Subdiagnostic)] #[multipart_suggestion (mir_build_suggestion , applicability = "machine-applicable")] pub (crate) struct CallToDeprecatedSafeFnRequiresUnsafeSub { pub (crate) start_of_line_suggestion : String , #[suggestion_part (code = "{start_of_line_suggestion}")] pub (crate) start_of_line : Span , #[suggestion_part (code = "unsafe {{ ")] pub (crate) left : Span , #[suggestion_part (code = " }}")] pub (crate) right : Span , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_call_to_unsafe_fn_requires_unsafe , code = E0133)] #[note] pub (crate) struct UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafe { #[label] pub (crate) span : Span , pub (crate) function : String , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_call_to_unsafe_fn_requires_unsafe_nameless , code = E0133)] #[note] pub (crate) struct UnsafeOpInUnsafeFnCallToUnsafeFunctionRequiresUnsafeNameless { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_inline_assembly_requires_unsafe , code = E0133)] #[note] pub (crate) struct UnsafeOpInUnsafeFnUseOfInlineAssemblyRequiresUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_initializing_type_with_requires_unsafe , code = E0133)] #[note] pub (crate) struct UnsafeOpInUnsafeFnInitializingTypeWithRequiresUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_initializing_type_with_unsafe_field_requires_unsafe , code = E0133)] #[note] pub (crate) struct UnsafeOpInUnsafeFnInitializingTypeWithUnsafeFieldRequiresUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_mutable_static_requires_unsafe , code = E0133)] #[note] pub (crate) struct UnsafeOpInUnsafeFnUseOfMutableStaticRequiresUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_extern_static_requires_unsafe , code = E0133)] #[note] pub (crate) struct UnsafeOpInUnsafeFnUseOfExternStaticRequiresUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_unsafe_field_requires_unsafe , code = E0133)] #[note] pub (crate) struct UnsafeOpInUnsafeFnUseOfUnsafeFieldRequiresUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_deref_raw_pointer_requires_unsafe , code = E0133)] #[note] pub (crate) struct UnsafeOpInUnsafeFnDerefOfRawPointerRequiresUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_union_field_requires_unsafe , code = E0133)] #[note] pub (crate) struct UnsafeOpInUnsafeFnAccessToUnionFieldRequiresUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_mutation_of_layout_constrained_field_requires_unsafe , code = E0133)] #[note] pub (crate) struct UnsafeOpInUnsafeFnMutationOfLayoutConstrainedFieldRequiresUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_borrow_of_layout_constrained_field_requires_unsafe , code = E0133 ,)] pub (crate) struct UnsafeOpInUnsafeFnBorrowOfLayoutConstrainedFieldRequiresUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_binder_cast_requires_unsafe , code = E0133 ,)] pub (crate) struct UnsafeOpInUnsafeFnUnsafeBinderCastRequiresUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unsafe_op_in_unsafe_fn_call_to_fn_with_requires_unsafe , code = E0133)] #[help] pub (crate) struct UnsafeOpInUnsafeFnCallToFunctionWithRequiresUnsafe { #[label] pub (crate) span : Span , pub (crate) function : String , pub (crate) missing_target_features : DiagArgValue , pub (crate) missing_target_features_count : usize , #[note] pub (crate) note : bool , pub (crate) build_target_features : DiagArgValue , pub (crate) build_target_features_count : usize , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedLintNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_call_to_unsafe_fn_requires_unsafe , code = E0133)] #[note] pub (crate) struct CallToUnsafeFunctionRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , pub (crate) function : String , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_call_to_unsafe_fn_requires_unsafe_nameless , code = E0133)] #[note] pub (crate) struct CallToUnsafeFunctionRequiresUnsafeNameless { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_call_to_unsafe_fn_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct CallToUnsafeFunctionRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , pub (crate) function : String , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_call_to_unsafe_fn_requires_unsafe_nameless_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct CallToUnsafeFunctionRequiresUnsafeNamelessUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_inline_assembly_requires_unsafe , code = E0133)] #[note] pub (crate) struct UseOfInlineAssemblyRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_inline_assembly_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct UseOfInlineAssemblyRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_initializing_type_with_requires_unsafe , code = E0133)] #[note] pub (crate) struct InitializingTypeWithRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_initializing_type_with_unsafe_field_requires_unsafe , code = E0133)] #[note] pub (crate) struct InitializingTypeWithUnsafeFieldRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_initializing_type_with_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct InitializingTypeWithRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_initializing_type_with_unsafe_field_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct InitializingTypeWithUnsafeFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_mutable_static_requires_unsafe , code = E0133)] #[note] pub (crate) struct UseOfMutableStaticRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_mutable_static_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct UseOfMutableStaticRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_extern_static_requires_unsafe , code = E0133)] #[note] pub (crate) struct UseOfExternStaticRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_extern_static_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct UseOfExternStaticRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_unsafe_field_requires_unsafe , code = E0133)] #[note] pub (crate) struct UseOfUnsafeFieldRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_unsafe_field_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct UseOfUnsafeFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_deref_raw_pointer_requires_unsafe , code = E0133)] #[note] pub (crate) struct DerefOfRawPointerRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_deref_raw_pointer_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct DerefOfRawPointerRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_union_field_requires_unsafe , code = E0133)] #[note] pub (crate) struct AccessToUnionFieldRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_union_field_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct AccessToUnionFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_mutation_of_layout_constrained_field_requires_unsafe , code = E0133)] #[note] pub (crate) struct MutationOfLayoutConstrainedFieldRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_mutation_of_layout_constrained_field_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct MutationOfLayoutConstrainedFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_borrow_of_layout_constrained_field_requires_unsafe , code = E0133)] #[note] pub (crate) struct BorrowOfLayoutConstrainedFieldRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_borrow_of_layout_constrained_field_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[note] pub (crate) struct BorrowOfLayoutConstrainedFieldRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_call_to_fn_with_requires_unsafe , code = E0133)] #[help] pub (crate) struct CallToFunctionWithRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , pub (crate) function : String , pub (crate) missing_target_features : DiagArgValue , pub (crate) missing_target_features_count : usize , #[note] pub (crate) note : bool , pub (crate) build_target_features : DiagArgValue , pub (crate) build_target_features_count : usize , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_call_to_fn_with_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133)] #[help] pub (crate) struct CallToFunctionWithRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , pub (crate) function : String , pub (crate) missing_target_features : DiagArgValue , pub (crate) missing_target_features_count : usize , #[note] pub (crate) note : bool , pub (crate) build_target_features : DiagArgValue , pub (crate) build_target_features_count : usize , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_unsafe_binder_cast_requires_unsafe , code = E0133 ,)] pub (crate) struct UnsafeBinderCastRequiresUnsafe { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_unsafe_binder_cast_requires_unsafe_unsafe_op_in_unsafe_fn_allowed , code = E0133 ,)] pub (crate) struct UnsafeBinderCastRequiresUnsafeUnsafeOpInUnsafeFnAllowed { #[primary_span] #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) unsafe_not_inherited_note : Option < UnsafeNotInheritedNote > , }}}
mkitem!{mkstruct!{#[derive (Subdiagnostic)] #[label (mir_build_unsafe_not_inherited)] pub (crate) struct UnsafeNotInheritedNote { #[primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{pub (crate) struct UnsafeNotInheritedLintNote { pub (crate) signature_span : Span , pub (crate) body_span : Span , }}}
mkitem!{mkimpl!{impl Subdiagnostic for UnsafeNotInheritedLintNote { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . span_note (self . signature_span , fluent :: mir_build_unsafe_fn_safe_body) ; let body_start = self . body_span . shrink_to_lo () ; let body_end = self . body_span . shrink_to_hi () ; diag . tool_only_multipart_suggestion (fluent :: mir_build_wrap_suggestion , vec ! [(body_start , "{ unsafe " . into ()) , (body_end , "}" . into ())] , Applicability :: MachineApplicable ,) ; } }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unused_unsafe)] pub (crate) struct UnusedUnsafe { #[label] pub (crate) span : Span , #[subdiagnostic] pub (crate) enclosing : Option < UnusedUnsafeEnclosing > , }}}
mkitem!{mkenum!{#[derive (Subdiagnostic)] pub (crate) enum UnusedUnsafeEnclosing { #[label (mir_build_unused_unsafe_enclosing_block_label)] Block { #[primary_span] span : Span , } , }}}
mkitem!{mkstruct!{pub (crate) struct NonExhaustivePatternsTypeNotEmpty < 'p , 'tcx , 'm > { pub (crate) cx : & 'm RustcPatCtxt < 'p , 'tcx > , pub (crate) scrut_span : Span , pub (crate) braces_span : Option < Span > , pub (crate) ty : Ty < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for NonExhaustivePatternsTypeNotEmpty < '_ , '_ , '_ > { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let mut diag = Diag :: new (dcx , level , fluent :: mir_build_non_exhaustive_patterns_type_not_empty) ; diag . span (self . scrut_span) ; diag . code (E0004) ; let peeled_ty = self . ty . peel_refs () ; diag . arg ("ty" , self . ty) ; diag . arg ("peeled_ty" , peeled_ty) ; if let ty :: Adt (def , _) = peeled_ty . kind () { let def_span = self . cx . tcx . hir_get_if_local (def . did ()) . and_then (| node | node . ident ()) . map (| ident | ident . span) . unwrap_or_else (| | self . cx . tcx . def_span (def . did ())) ; let mut span : MultiSpan = def_span . into () ; span . push_span_label (def_span , "") ; diag . span_note (span , fluent :: mir_build_def_note) ; } let is_non_exhaustive = matches ! (self . ty . kind () , ty :: Adt (def , _) if def . variant_list_has_applicable_non_exhaustive ()) ; if is_non_exhaustive { diag . note (fluent :: mir_build_non_exhaustive_type_note) ; } else { diag . note (fluent :: mir_build_type_note) ; } if let ty :: Ref (_ , sub_ty , _) = self . ty . kind () { if ! sub_ty . is_inhabited_from (self . cx . tcx , self . cx . module , self . cx . typing_env) { diag . note (fluent :: mir_build_reference_note) ; } } let sm = self . cx . tcx . sess . source_map () ; if let Some (braces_span) = self . braces_span { let (indentation , more) = if let Some (snippet) = sm . indentation_before (self . scrut_span) { (format ! ("\n{snippet}") , "    ") } else { (" " . to_string () , "") } ; diag . span_suggestion_verbose (braces_span , fluent :: mir_build_suggestion , format ! (" {{{indentation}{more}_ => todo!(),{indentation}}}") , Applicability :: HasPlaceholders ,) ; } else { diag . help (fluent :: mir_build_help) ; } diag } }}}
mkitem!{mkstruct!{#[derive (Subdiagnostic)] #[note (mir_build_non_exhaustive_match_all_arms_guarded)] pub (crate) struct NonExhaustiveMatchAllArmsGuarded ;}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_static_in_pattern , code = E0158)] pub (crate) struct StaticInPattern { #[primary_span] #[label] pub (crate) span : Span , #[label (mir_build_static_in_pattern_def)] pub (crate) static_span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_const_param_in_pattern , code = E0158)] pub (crate) struct ConstParamInPattern { #[primary_span] #[label] pub (crate) span : Span , #[label (mir_build_const_param_in_pattern_def)] pub (crate) const_span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_non_const_path , code = E0080)] pub (crate) struct NonConstPath { #[primary_span] #[label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_unreachable_pattern)] pub (crate) struct UnreachablePattern < 'tcx > { #[label] pub (crate) span : Option < Span > , #[label (mir_build_unreachable_matches_no_values)] pub (crate) matches_no_values : Option < Span > , pub (crate) matches_no_values_ty : Ty < 'tcx > , #[note (mir_build_unreachable_uninhabited_note)] pub (crate) uninhabited_note : Option < () > , #[label (mir_build_unreachable_covered_by_catchall)] pub (crate) covered_by_catchall : Option < Span > , #[subdiagnostic] pub (crate) wanted_constant : Option < WantedConstant > , #[note (mir_build_unreachable_pattern_const_reexport_accessible)] pub (crate) accessible_constant : Option < Span > , #[note (mir_build_unreachable_pattern_const_inaccessible)] pub (crate) inaccessible_constant : Option < Span > , #[note (mir_build_unreachable_pattern_let_binding)] pub (crate) pattern_let_binding : Option < Span > , #[label (mir_build_unreachable_covered_by_one)] pub (crate) covered_by_one : Option < Span > , #[note (mir_build_unreachable_covered_by_many)] pub (crate) covered_by_many : Option < MultiSpan > , pub (crate) covered_by_many_n_more_count : usize , #[suggestion (code = "" , applicability = "machine-applicable")] pub (crate) suggest_remove : Option < Span > , }}}
mkitem!{mkstruct!{#[derive (Subdiagnostic)] #[suggestion (mir_build_unreachable_pattern_wanted_const , code = "{const_path}" , applicability = "machine-applicable")] pub (crate) struct WantedConstant { #[primary_span] pub (crate) span : Span , pub (crate) is_typo : bool , pub (crate) const_name : String , pub (crate) const_path : String , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_const_pattern_depends_on_generic_parameter , code = E0158)] pub (crate) struct ConstPatternDependsOnGenericParameter { #[primary_span] #[label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_could_not_eval_const_pattern)] pub (crate) struct CouldNotEvalConstPattern { #[primary_span] #[label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_lower_range_bound_must_be_less_than_or_equal_to_upper , code = E0030)] pub (crate) struct LowerRangeBoundMustBeLessThanOrEqualToUpper { #[primary_span] #[label] pub (crate) span : Span , #[note (mir_build_teach_note)] pub (crate) teach : bool , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_literal_in_range_out_of_bounds)] pub (crate) struct LiteralOutOfRange < 'tcx > { #[primary_span] #[label] pub (crate) span : Span , pub (crate) ty : Ty < 'tcx > , pub (crate) min : i128 , pub (crate) max : u128 , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_lower_range_bound_must_be_less_than_upper , code = E0579)] pub (crate) struct LowerRangeBoundMustBeLessThanUpper { #[primary_span] pub (crate) span : Span , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_leading_irrefutable_let_patterns)] #[note] #[help] pub (crate) struct LeadingIrrefutableLetPatterns { pub (crate) count : usize , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_trailing_irrefutable_let_patterns)] #[note] #[help] pub (crate) struct TrailingIrrefutableLetPatterns { pub (crate) count : usize , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_bindings_with_variant_name , code = E0170)] pub (crate) struct BindingsWithVariantName { #[suggestion (code = "{ty_path}::{name}" , applicability = "machine-applicable")] pub (crate) suggestion : Option < Span > , pub (crate) ty_path : String , pub (crate) name : Ident , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_irrefutable_let_patterns_if_let)] #[note] #[help] pub (crate) struct IrrefutableLetPatternsIfLet { pub (crate) count : usize , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_irrefutable_let_patterns_if_let_guard)] #[note] #[help] pub (crate) struct IrrefutableLetPatternsIfLetGuard { pub (crate) count : usize , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_irrefutable_let_patterns_let_else)] #[note] #[help] pub (crate) struct IrrefutableLetPatternsLetElse { pub (crate) count : usize , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_irrefutable_let_patterns_while_let)] #[note] #[help] pub (crate) struct IrrefutableLetPatternsWhileLet { pub (crate) count : usize , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_borrow_of_moved_value)] pub (crate) struct BorrowOfMovedValue < 'tcx > { #[primary_span] #[label] #[label (mir_build_occurs_because_label)] pub (crate) binding_span : Span , #[label (mir_build_value_borrowed_label)] pub (crate) conflicts_ref : Vec < Span > , pub (crate) name : Ident , pub (crate) ty : Ty < 'tcx > , #[suggestion (code = "ref " , applicability = "machine-applicable")] pub (crate) suggest_borrowing : Option < Span > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_multiple_mut_borrows)] pub (crate) struct MultipleMutBorrows { #[primary_span] pub (crate) span : Span , #[subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_already_borrowed)] pub (crate) struct AlreadyBorrowed { #[primary_span] pub (crate) span : Span , #[subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_already_mut_borrowed)] pub (crate) struct AlreadyMutBorrowed { #[primary_span] pub (crate) span : Span , #[subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_moved_while_borrowed)] pub (crate) struct MovedWhileBorrowed { #[primary_span] pub (crate) span : Span , #[subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }}}
mkitem!{mkenum!{#[derive (Subdiagnostic)] pub (crate) enum Conflict { #[label (mir_build_mutable_borrow)] Mut { #[primary_span] span : Span , name : Symbol , } , #[label (mir_build_borrow)] Ref { #[primary_span] span : Span , name : Symbol , } , #[label (mir_build_moved)] Moved { #[primary_span] span : Span , name : Symbol , } , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_union_pattern)] pub (crate) struct UnionPattern { #[primary_span] #[label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_type_not_structural)] pub (crate) struct TypeNotStructural < 'tcx > { #[primary_span] #[label] pub (crate) span : Span , #[label (mir_build_type_not_structural_def)] pub (crate) ty_def_span : Span , pub (crate) ty : Ty < 'tcx > , #[note (mir_build_type_not_structural_tip)] pub (crate) manual_partialeq_impl_span : Option < Span > , #[note (mir_build_type_not_structural_more_info)] pub (crate) manual_partialeq_impl_note : bool , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_non_partial_eq_match)] #[note (mir_build_type_not_structural_more_info)] pub (crate) struct TypeNotPartialEq < 'tcx > { #[primary_span] #[label] pub (crate) span : Span , pub (crate) ty : Ty < 'tcx > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_invalid_pattern)] pub (crate) struct InvalidPattern < 'tcx > { #[primary_span] #[label] pub (crate) span : Span , pub (crate) non_sm_ty : Ty < 'tcx > , pub (crate) prefix : String , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_unsized_pattern)] pub (crate) struct UnsizedPattern < 'tcx > { #[primary_span] pub (crate) span : Span , pub (crate) non_sm_ty : Ty < 'tcx > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_nan_pattern)] #[note] #[help] pub (crate) struct NaNPattern { #[primary_span] #[label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_pointer_pattern)] #[note] pub (crate) struct PointerPattern { #[primary_span] #[label] pub (crate) span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_non_empty_never_pattern)] #[note] pub (crate) struct NonEmptyNeverPattern < 'tcx > { #[primary_span] #[label] pub (crate) span : Span , pub (crate) ty : Ty < 'tcx > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_pattern_not_covered , code = E0005)] pub (crate) struct PatternNotCovered < 's , 'tcx > { #[primary_span] pub (crate) span : Span , pub (crate) origin : & 's str , #[subdiagnostic] pub (crate) uncovered : Uncovered , #[subdiagnostic] pub (crate) inform : Option < Inform > , #[subdiagnostic] pub (crate) interpreted_as_const : Option < InterpretedAsConst > , #[subdiagnostic] pub (crate) interpreted_as_const_sugg : Option < InterpretedAsConstSugg > , #[subdiagnostic] pub (crate) adt_defined_here : Option < AdtDefinedHere < 'tcx > > , #[note (mir_build_privately_uninhabited)] pub (crate) witness_1_is_privately_uninhabited : bool , pub (crate) witness_1 : String , #[note (mir_build_pattern_ty)] pub (crate) _p : () , pub (crate) pattern_ty : Ty < 'tcx > , #[subdiagnostic] pub (crate) let_suggestion : Option < SuggestLet > , #[subdiagnostic] pub (crate) misc_suggestion : Option < MiscPatternSuggestion > , }}}
mkitem!{mkstruct!{#[derive (Subdiagnostic)] #[note (mir_build_inform_irrefutable)] #[note (mir_build_more_information)] pub (crate) struct Inform ;}}
mkitem!{mkstruct!{#[derive (Subdiagnostic)] #[label (mir_build_confused)] pub (crate) struct InterpretedAsConst { #[primary_span] pub (crate) span : Span , pub (crate) variable : String , }}}
mkitem!{mkstruct!{pub (crate) struct AdtDefinedHere < 'tcx > { pub (crate) adt_def_span : Span , pub (crate) ty : Ty < 'tcx > , pub (crate) variants : Vec < Variant > , }}}
mkitem!{mkstruct!{pub (crate) struct Variant { pub (crate) span : Span , }}}
mkitem!{mkimpl!{impl < 'tcx > Subdiagnostic for AdtDefinedHere < 'tcx > { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("ty" , self . ty) ; let mut spans = MultiSpan :: from (self . adt_def_span) ; for Variant { span } in self . variants { spans . push_span_label (span , fluent :: mir_build_variant_defined_here) ; } diag . span_note (spans , fluent :: mir_build_adt_defined_here) ; } }}}
mkitem!{mkstruct!{#[derive (Subdiagnostic)] #[suggestion (mir_build_interpreted_as_const , code = "{variable}_var" , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct InterpretedAsConstSugg { #[primary_span] pub (crate) span : Span , pub (crate) variable : String , }}}
mkitem!{mkenum!{#[derive (Subdiagnostic)] pub (crate) enum SuggestLet { #[multipart_suggestion (mir_build_suggest_if_let , applicability = "has-placeholders")] If { #[suggestion_part (code = "if ")] start_span : Span , #[suggestion_part (code = " {{ todo!() }}")] semi_span : Span , count : usize , } , #[suggestion (mir_build_suggest_let_else , code = " else {{ todo!() }}" , applicability = "has-placeholders")] Else { #[primary_span] end_span : Span , count : usize , } , }}}
mkitem!{mkenum!{#[derive (Subdiagnostic)] pub (crate) enum MiscPatternSuggestion { #[suggestion (mir_build_suggest_attempted_int_lit , code = "_" , applicability = "maybe-incorrect")] AttemptedIntegerLiteral { #[primary_span] start_span : Span , } , }}}
mkitem!{mkstruct!{#[derive (LintDiagnostic)] #[diag (mir_build_rust_2024_incompatible_pat)] pub (crate) struct Rust2024IncompatiblePat { #[subdiagnostic] pub (crate) sugg : Rust2024IncompatiblePatSugg , pub (crate) bad_modifiers : bool , pub (crate) bad_ref_pats : bool , pub (crate) is_hard_error : bool , }}}
mkitem!{mkstruct!{pub (crate) struct Rust2024IncompatiblePatSugg { #[doc = " If true, our suggestion is to elide explicit binding modifiers."] #[doc = " If false, our suggestion is to make the pattern fully explicit."] pub (crate) suggest_eliding_modes : bool , pub (crate) suggestion : Vec < (Span , String) > , pub (crate) ref_pattern_count : usize , pub (crate) binding_mode_count : usize , #[doc = " Labels for where incompatibility-causing by-ref default binding modes were introduced."] pub (crate) default_mode_labels : FxIndexMap < Span , ty :: Mutability > , }}}
mkitem!{mkimpl!{impl Subdiagnostic for Rust2024IncompatiblePatSugg { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { for (span , def_br_mutbl) in self . default_mode_labels . into_iter () . rev () { if ! span . from_expansion () { let note_msg = "matching on a reference type with a non-reference pattern changes the default binding mode" ; let label_msg = format ! ("this matches on type `{}_`" , def_br_mutbl . ref_prefix_str ()) ; let mut label = MultiSpan :: from (span) ; label . push_span_label (span , label_msg) ; diag . span_note (label , note_msg) ; } } let applicability = if self . suggestion . iter () . all (| (span , _) | span . can_be_used_for_suggestions ()) { Applicability :: MachineApplicable } else { Applicability :: MaybeIncorrect } ; let msg = if self . suggest_eliding_modes { let plural_modes = pluralize ! (self . binding_mode_count) ; format ! ("remove the unnecessary binding modifier{plural_modes}") } else { let plural_derefs = pluralize ! (self . ref_pattern_count) ; let and_modes = if self . binding_mode_count > 0 { format ! (" and variable binding mode{}" , pluralize ! (self . binding_mode_count)) } else { String :: new () } ; format ! ("make the implied reference pattern{plural_derefs}{and_modes} explicit") } ; if ! self . suggestion . is_empty () { diag . multipart_suggestion_verbose (msg , self . suggestion , applicability) ; } } }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_loop_match_invalid_update)] pub (crate) struct LoopMatchInvalidUpdate { #[primary_span] pub lhs : Span , #[label] pub scrutinee : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_loop_match_invalid_match)] #[note] pub (crate) struct LoopMatchInvalidMatch { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_loop_match_unsupported_type)] #[note] pub (crate) struct LoopMatchUnsupportedType < 'tcx > { #[primary_span] pub span : Span , pub ty : Ty < 'tcx > , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_loop_match_bad_statements)] pub (crate) struct LoopMatchBadStatements { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_loop_match_bad_rhs)] pub (crate) struct LoopMatchBadRhs { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_loop_match_missing_assignment)] pub (crate) struct LoopMatchMissingAssignment { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_loop_match_arm_with_guard)] pub (crate) struct LoopMatchArmWithGuard { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_const_continue_not_const)] #[help] pub (crate) struct ConstContinueNotMonomorphicConst { #[primary_span] pub span : Span , #[subdiagnostic] pub reason : ConstContinueNotMonomorphicConstReason , }}}
mkitem!{mkenum!{#[derive (Subdiagnostic)] pub (crate) enum ConstContinueNotMonomorphicConstReason { #[label (mir_build_const_continue_not_const_constant_parameter)] ConstantParameter { #[primary_span] span : Span , } , #[label (mir_build_const_continue_not_const_const_block)] ConstBlock { #[primary_span] span : Span , } , #[label (mir_build_const_continue_not_const_const_other)] Other { #[primary_span] span : Span , } , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_const_continue_bad_const)] pub (crate) struct ConstContinueBadConst { #[primary_span] #[label] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_const_continue_missing_label_or_value)] pub (crate) struct ConstContinueMissingLabelOrValue { #[primary_span] pub span : Span , }}}
mkitem!{mkstruct!{#[derive (Diagnostic)] #[diag (mir_build_const_continue_unknown_jump_target)] pub (crate) struct ConstContinueUnknownJumpTarget { #[primary_span] pub span : Span , }}}