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
mkuse!{use std :: collections :: { HashMap , HashSet } ;}
mkuse!{use std :: fs :: read_to_string ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use annotate_snippets :: { Renderer , Snippet } ;}
mkuse!{use fluent_bundle :: { FluentBundle , FluentError , FluentResource } ;}
mkuse!{use fluent_syntax :: ast :: { Attribute , Entry , Expression , Identifier , InlineExpression , Message , Pattern , PatternElement , } ;}
mkuse!{use fluent_syntax :: parser :: ParserError ;}
mkuse!{use proc_macro :: tracked_path :: path ;}
mkuse!{use proc_macro :: { Diagnostic , Level , Span } ;}
mkuse!{use proc_macro2 :: TokenStream ;}
mkuse!{use quote :: quote ;}
mkuse!{use syn :: { Ident , LitStr , parse_macro_input } ;}
mkuse!{use unic_langid :: langid ;}

macro_rules! invocation_relative_path_to_absolute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invocation_relative_path_to_absolute in module {}", module_path!());
    };
}

mkfn!{
    invocation_relative_path_to_absolute_introspect!();
    #[doc = " Helper function for returning an absolute path for macro-invocation relative file paths."] #[doc = ""] #[doc = " If the input is already absolute, then the input is returned. If the input is not absolute,"] #[doc = " then it is appended to the directory containing the source file with this macro invocation."] fn invocation_relative_path_to_absolute (span : Span , path : & str) -> PathBuf { let path = Path :: new (path) ; if path . is_absolute () { path . to_path_buf () } else { let mut source_file_path = span . local_file () . unwrap () ; source_file_path . pop () ; source_file_path . push (path) ; source_file_path } }
}

macro_rules! finish_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function finish in module {}", module_path!());
    };
}

mkfn!{
    finish_introspect!();
    #[doc = " Final tokens."] fn finish (body : TokenStream , resource : TokenStream) -> proc_macro :: TokenStream { quote ! { #[doc = " Raw content of Fluent resource for this crate, generated by `fluent_messages` macro,"] #[doc = " imported by `rustc_driver` to include all crates' resources in one bundle."] pub static DEFAULT_LOCALE_RESOURCE : &'static str = # resource ; #[allow (non_upper_case_globals)] #[doc (hidden)] #[doc = " Auto-generated constants for type-checked references to Fluent messages."] pub (crate) mod fluent_generated { # body #[doc = " Constants expected to exist by the diagnostic derive macros to use as default Fluent"] #[doc = " identifiers for different subdiagnostic kinds."] pub mod _subdiag { #[doc = " Default for `#[help]`"] pub const help : rustc_errors :: SubdiagMessage = rustc_errors :: SubdiagMessage :: FluentAttr (std :: borrow :: Cow :: Borrowed ("help")) ; #[doc = " Default for `#[note]`"] pub const note : rustc_errors :: SubdiagMessage = rustc_errors :: SubdiagMessage :: FluentAttr (std :: borrow :: Cow :: Borrowed ("note")) ; #[doc = " Default for `#[warn]`"] pub const warn : rustc_errors :: SubdiagMessage = rustc_errors :: SubdiagMessage :: FluentAttr (std :: borrow :: Cow :: Borrowed ("warn")) ; #[doc = " Default for `#[label]`"] pub const label : rustc_errors :: SubdiagMessage = rustc_errors :: SubdiagMessage :: FluentAttr (std :: borrow :: Cow :: Borrowed ("label")) ; #[doc = " Default for `#[suggestion]`"] pub const suggestion : rustc_errors :: SubdiagMessage = rustc_errors :: SubdiagMessage :: FluentAttr (std :: borrow :: Cow :: Borrowed ("suggestion")) ; } } } . into () }
}

macro_rules! failed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function failed in module {}", module_path!());
    };
}

mkfn!{
    failed_introspect!();
    #[doc = " Tokens to be returned when the macro cannot proceed."] fn failed (crate_name : & Ident) -> proc_macro :: TokenStream { finish (quote ! { pub mod # crate_name { } } , quote ! { "" }) }
}

macro_rules! fluent_messages_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fluent_messages in module {}", module_path!());
    };
}

mkfn!{
    fluent_messages_introspect!();
    #[doc = " See [rustc_fluent_macro::fluent_messages]."] pub (crate) fn fluent_messages (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let crate_name = std :: env :: var ("CARGO_CRATE_NAME") . unwrap_or_else (| _ | "no_crate" . to_string ()) . replace ("rustc_" , "") ; let mut bundle = FluentBundle :: new (vec ! [langid ! ("en-US")]) ; let mut previous_attrs = HashSet :: new () ; let resource_str = parse_macro_input ! (input as LitStr) ; let resource_span = resource_str . span () . unwrap () ; let relative_ftl_path = resource_str . value () ; let absolute_ftl_path = invocation_relative_path_to_absolute (resource_span , & relative_ftl_path) ; let crate_name = Ident :: new (& crate_name , resource_str . span ()) ; path (absolute_ftl_path . to_str () . unwrap ()) ; let resource_contents = match read_to_string (absolute_ftl_path) { Ok (resource_contents) => resource_contents , Err (e) => { Diagnostic :: spanned (resource_span , Level :: Error , format ! ("could not open Fluent resource: {e}") ,) . emit () ; return failed (& crate_name) ; } } ; let mut bad = false ; for esc in ["\\n" , "\\\"" , "\\'"] { for _ in resource_contents . matches (esc) { bad = true ; Diagnostic :: spanned (resource_span , Level :: Error , format ! ("invalid escape `{esc}` in Fluent resource")) . note ("Fluent does not interpret these escape sequences (<https://projectfluent.org/fluent/guide/special.html>)") . emit () ; } } if bad { return failed (& crate_name) ; } let resource = match FluentResource :: try_new (resource_contents) { Ok (resource) => resource , Err ((this , errs)) => { Diagnostic :: spanned (resource_span , Level :: Error , "could not parse Fluent resource") . help ("see additional errors emitted") . emit () ; for ParserError { pos , slice : _ , kind } in errs { let mut err = kind . to_string () ; err . replace_range (0 .. 1 , & err . chars () . next () . unwrap () . to_lowercase () . to_string ()) ; let message = annotate_snippets :: Level :: Error . title (& err) . snippet (Snippet :: source (this . source ()) . origin (& relative_ftl_path) . fold (true) . annotation (annotate_snippets :: Level :: Error . span (pos . start .. pos . end - 1)) ,) ; let renderer = Renderer :: plain () ; eprintln ! ("{}\n" , renderer . render (message)) ; } return failed (& crate_name) ; } } ; let mut constants = TokenStream :: new () ; let mut previous_defns = HashMap :: new () ; let mut message_refs = Vec :: new () ; for entry in resource . entries () { if let Entry :: Message (msg) = entry { let Message { id : Identifier { name } , attributes , value , .. } = msg ; let _ = previous_defns . entry (name . to_string ()) . or_insert (resource_span) ; if name . contains ('-') { Diagnostic :: spanned (resource_span , Level :: Error , format ! ("name `{name}` contains a '-' character") ,) . help ("replace any '-'s with '_'s") . emit () ; } if let Some (Pattern { elements }) = value { for elt in elements { if let PatternElement :: Placeable { expression : Expression :: Inline (InlineExpression :: MessageReference { id , .. }) , } = elt { message_refs . push ((id . name , * name)) ; } } } let crate_prefix = format ! ("{crate_name}_") ; let snake_name = name . replace ('-' , "_") ; if ! snake_name . starts_with (& crate_prefix) { Diagnostic :: spanned (resource_span , Level :: Error , format ! ("name `{name}` does not start with the crate name") ,) . help (format ! ("prepend `{crate_prefix}` to the slug name: `{crate_prefix}{snake_name}`")) . emit () ; } ; let snake_name = Ident :: new (& snake_name , resource_str . span ()) ; if ! previous_attrs . insert (snake_name . clone ()) { continue ; } let docstr = format ! ("Constant referring to Fluent message `{name}` from `{crate_name}`") ; constants . extend (quote ! { #[doc = # docstr] pub const # snake_name : rustc_errors :: DiagMessage = rustc_errors :: DiagMessage :: FluentIdentifier (std :: borrow :: Cow :: Borrowed (# name) , None) ; }) ; for Attribute { id : Identifier { name : attr_name } , .. } in attributes { let snake_name = Ident :: new (& format ! ("{crate_prefix}{}" , attr_name . replace ('-' , "_")) , resource_str . span () ,) ; if ! previous_attrs . insert (snake_name . clone ()) { continue ; } if attr_name . contains ('-') { Diagnostic :: spanned (resource_span , Level :: Error , format ! ("attribute `{attr_name}` contains a '-' character") ,) . help ("replace any '-'s with '_'s") . emit () ; } let msg = format ! ("Constant referring to Fluent message `{name}.{attr_name}` from `{crate_name}`") ; constants . extend (quote ! { #[doc = # msg] pub const # snake_name : rustc_errors :: SubdiagMessage = rustc_errors :: SubdiagMessage :: FluentAttr (std :: borrow :: Cow :: Borrowed (# attr_name)) ; }) ; } let ident = quote :: format_ident ! ("{snake_name}_refs") ; let vrefs = variable_references (msg) ; constants . extend (quote ! { #[cfg (test)] pub const # ident : & [& str] = & [# (# vrefs) ,*] ; }) } } for (mref , name) in message_refs . into_iter () { if ! previous_defns . contains_key (mref) { Diagnostic :: spanned (resource_span , Level :: Error , format ! ("referenced message `{mref}` does not exist (in message `{name}`)") ,) . help (& format ! ("you may have meant to use a variable reference (`{{${mref}}}`)")) . emit () ; } } if let Err (errs) = bundle . add_resource (resource) { for e in errs { match e { FluentError :: Overriding { kind , id } => { Diagnostic :: spanned (resource_span , Level :: Error , format ! ("overrides existing {kind}: `{id}`") ,) . emit () ; } FluentError :: ResolverError (_) | FluentError :: ParserError (_) => unreachable ! () , } } } finish (constants , quote ! { include_str ! (# relative_ftl_path) }) }
}

macro_rules! variable_references_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function variable_references in module {}", module_path!());
    };
}

mkfn!{
    variable_references_introspect!();
    fn variable_references < 'a > (msg : & Message < & 'a str >) -> Vec < & 'a str > { let mut refs = vec ! [] ; if let Some (Pattern { elements }) = & msg . value { for elt in elements { if let PatternElement :: Placeable { expression : Expression :: Inline (InlineExpression :: VariableReference { id }) , } = elt { refs . push (id . name) ; } } } for attr in & msg . attributes { for elt in & attr . value . elements { if let PatternElement :: Placeable { expression : Expression :: Inline (InlineExpression :: VariableReference { id }) , } = elt { refs . push (id . name) ; } } } refs }
}