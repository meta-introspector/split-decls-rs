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
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_data_structures :: unord :: UnordMap ;}
mkuse!{use rustc_session :: { declare_lint , declare_lint_pass } ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use unicode_security :: general_security_profile :: IdentifierType ;}
mkuse!{use crate :: lints :: { ConfusableIdentifierPair , IdentifierNonAsciiChar , IdentifierUncommonCodepoints , MixedScriptConfusables , } ;}
mkuse!{use crate :: { EarlyContext , EarlyLintPass , LintContext } ;}
mkitem!{declare_lint ! { # [doc = " The `non_ascii_idents` lint detects non-ASCII identifiers."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " # #![allow(unused)]"] # [doc = " #![deny(non_ascii_idents)]"] # [doc = " fn main() {"] # [doc = "     let föö = 1;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " This lint allows projects that wish to retain the limit of only using"] # [doc = " ASCII characters to switch this lint to \"forbid\" (for example to ease"] # [doc = " collaboration or for security reasons)."] # [doc = " See [RFC 2457] for more details."] # [doc = ""] # [doc = " [RFC 2457]: https://github.com/rust-lang/rfcs/blob/master/text/2457-non-ascii-idents.md"] pub NON_ASCII_IDENTS , Allow , "detects non-ASCII identifiers" , crate_level_only }}
mkitem!{declare_lint ! { # [doc = " The `uncommon_codepoints` lint detects uncommon Unicode codepoints in"] # [doc = " identifiers."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #![allow(unused)]"] # [doc = " const µ: f64 = 0.000001;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " This lint warns about using characters which are not commonly used, and may"] # [doc = " cause visual confusion."] # [doc = ""] # [doc = " This lint is triggered by identifiers that contain a codepoint that is"] # [doc = " not part of the set of \"Allowed\" codepoints as described by [Unicode®"] # [doc = " Technical Standard #39 Unicode Security Mechanisms Section 3.1 General"] # [doc = " Security Profile for Identifiers][TR39Allowed]."] # [doc = ""] # [doc = " Note that the set of uncommon codepoints may change over time. Beware"] # [doc = " that if you \"forbid\" this lint that existing code may fail in the"] # [doc = " future."] # [doc = ""] # [doc = " [TR39Allowed]: https://www.unicode.org/reports/tr39/#General_Security_Profile"] pub UNCOMMON_CODEPOINTS , Warn , "detects uncommon Unicode codepoints in identifiers" , crate_level_only }}
mkitem!{declare_lint ! { # [doc = " The `confusable_idents` lint detects visually confusable pairs between"] # [doc = " identifiers."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " // Latin Capital Letter E With Caron"] # [doc = " pub const Ě: i32 = 1;"] # [doc = " // Latin Capital Letter E With Breve"] # [doc = " pub const Ĕ: i32 = 2;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " This lint warns when different identifiers may appear visually similar,"] # [doc = " which can cause confusion."] # [doc = ""] # [doc = " The confusable detection algorithm is based on [Unicode® Technical"] # [doc = " Standard #39 Unicode Security Mechanisms Section 4 Confusable"] # [doc = " Detection][TR39Confusable]. For every distinct identifier X execute"] # [doc = " the function `skeleton(X)`. If there exist two distinct identifiers X"] # [doc = " and Y in the same crate where `skeleton(X) = skeleton(Y)` report it."] # [doc = " The compiler uses the same mechanism to check if an identifier is too"] # [doc = " similar to a keyword."] # [doc = ""] # [doc = " Note that the set of confusable characters may change over time."] # [doc = " Beware that if you \"forbid\" this lint that existing code may fail in"] # [doc = " the future."] # [doc = ""] # [doc = " [TR39Confusable]: https://www.unicode.org/reports/tr39/#Confusable_Detection"] pub CONFUSABLE_IDENTS , Warn , "detects visually confusable pairs between identifiers" , crate_level_only }}
mkitem!{declare_lint ! { # [doc = " The `mixed_script_confusables` lint detects visually confusable"] # [doc = " characters in identifiers between different [scripts]."] # [doc = ""] # [doc = " [scripts]: https://en.wikipedia.org/wiki/Script_(Unicode)"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " // The Japanese katakana character エ can be confused with the Han character 工."] # [doc = " const エ: &'static str = \"アイウ\";"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " This lint warns when characters between different scripts may appear"] # [doc = " visually similar, which can cause confusion."] # [doc = ""] # [doc = " If the crate contains other identifiers in the same script that have"] # [doc = " non-confusable characters, then this lint will *not* be issued. For"] # [doc = " example, if the example given above has another identifier with"] # [doc = " katakana characters (such as `let カタカナ = 123;`), then this indicates"] # [doc = " that you are intentionally using katakana, and it will not warn about"] # [doc = " it."] # [doc = ""] # [doc = " Note that the set of confusable characters may change over time."] # [doc = " Beware that if you \"forbid\" this lint that existing code may fail in"] # [doc = " the future."] pub MIXED_SCRIPT_CONFUSABLES , Warn , "detects Unicode scripts whose mixed script confusables codepoints are solely used" , crate_level_only }}
mkitem!{declare_lint_pass ! (NonAsciiIdents => [NON_ASCII_IDENTS , UNCOMMON_CODEPOINTS , CONFUSABLE_IDENTS , MIXED_SCRIPT_CONFUSABLES]) ;}
mkitem!{mkimpl!{impl EarlyLintPass for NonAsciiIdents { fn check_crate (& mut self , cx : & EarlyContext < '_ > , _ : & ast :: Crate) { use std :: collections :: BTreeMap ; use rustc_session :: lint :: Level ; use rustc_span :: Span ; use unicode_security :: GeneralSecurityProfile ; let check_non_ascii_idents = cx . builder . lint_level (NON_ASCII_IDENTS) . level != Level :: Allow ; let check_uncommon_codepoints = cx . builder . lint_level (UNCOMMON_CODEPOINTS) . level != Level :: Allow ; let check_confusable_idents = cx . builder . lint_level (CONFUSABLE_IDENTS) . level != Level :: Allow ; let check_mixed_script_confusables = cx . builder . lint_level (MIXED_SCRIPT_CONFUSABLES) . level != Level :: Allow ; if ! check_non_ascii_idents && ! check_uncommon_codepoints && ! check_confusable_idents && ! check_mixed_script_confusables { return ; } let mut has_non_ascii_idents = false ; let symbols = cx . sess () . psess . symbol_gallery . symbols . lock () ; # [allow (rustc :: potential_query_instability)] let mut symbols : Vec < _ > = symbols . iter () . collect () ; symbols . sort_by_key (| k | k . 1) ; for & (ref symbol , & sp) in symbols . iter () { let symbol_str = symbol . as_str () ; if symbol_str . is_ascii () { continue ; } has_non_ascii_idents = true ; cx . emit_span_lint (NON_ASCII_IDENTS , sp , IdentifierNonAsciiChar) ; if check_uncommon_codepoints && ! symbol_str . chars () . all (GeneralSecurityProfile :: identifier_allowed) { let mut chars : Vec < _ > = symbol_str . chars () . map (| c | (c , GeneralSecurityProfile :: identifier_type (c))) . collect () ; for (id_ty , id_ty_descr) in [(IdentifierType :: Exclusion , "Exclusion") , (IdentifierType :: Technical , "Technical") , (IdentifierType :: Limited_Use , "Limited_Use") , (IdentifierType :: Not_NFKC , "Not_NFKC") ,] { let codepoints : Vec < _ > = chars . extract_if (.. , | (_ , ty) | * ty == Some (id_ty)) . collect () ; if codepoints . is_empty () { continue ; } cx . emit_span_lint (UNCOMMON_CODEPOINTS , sp , IdentifierUncommonCodepoints { codepoints_len : codepoints . len () , codepoints : codepoints . into_iter () . map (| (c , _) | c) . collect () , identifier_type : id_ty_descr , } ,) ; } let remaining = chars . extract_if (.. , | (c , _) | ! GeneralSecurityProfile :: identifier_allowed (* c)) . collect :: < Vec < _ > > () ; if ! remaining . is_empty () { cx . emit_span_lint (UNCOMMON_CODEPOINTS , sp , IdentifierUncommonCodepoints { codepoints_len : remaining . len () , codepoints : remaining . into_iter () . map (| (c , _) | c) . collect () , identifier_type : "Restricted" , } ,) ; } } } if has_non_ascii_idents && check_confusable_idents { let mut skeleton_map : UnordMap < Symbol , (Symbol , Span , bool) > = UnordMap :: with_capacity (symbols . len ()) ; let mut skeleton_buf = String :: new () ; for & (& symbol , & sp) in symbols . iter () { use unicode_security :: confusable_detection :: skeleton ; let symbol_str = symbol . as_str () ; let is_ascii = symbol_str . is_ascii () ; skeleton_buf . clear () ; skeleton_buf . extend (skeleton (symbol_str)) ; let skeleton_sym = if * symbol_str == * skeleton_buf { symbol } else { Symbol :: intern (& skeleton_buf) } ; skeleton_map . entry (skeleton_sym) . and_modify (| (existing_symbol , existing_span , existing_is_ascii) | { if ! * existing_is_ascii || ! is_ascii { cx . emit_span_lint (CONFUSABLE_IDENTS , sp , ConfusableIdentifierPair { existing_sym : * existing_symbol , sym : symbol , label : * existing_span , main_label : sp , } ,) ; } if * existing_is_ascii && ! is_ascii { * existing_symbol = symbol ; * existing_span = sp ; * existing_is_ascii = is_ascii ; } }) . or_insert ((symbol , sp , is_ascii)) ; } } if has_non_ascii_idents && check_mixed_script_confusables { use unicode_security :: is_potential_mixed_script_confusable_char ; use unicode_security :: mixed_script :: AugmentedScriptSet ; # [derive (Clone)] enum ScriptSetUsage { Suspicious (Vec < char > , Span) , Verified , } let mut script_states : FxIndexMap < AugmentedScriptSet , ScriptSetUsage > = Default :: default () ; let latin_augmented_script_set = AugmentedScriptSet :: for_char ('A') ; script_states . insert (latin_augmented_script_set , ScriptSetUsage :: Verified) ; let mut has_suspicious = false ; for & (ref symbol , & sp) in symbols . iter () { let symbol_str = symbol . as_str () ; for ch in symbol_str . chars () { if ch . is_ascii () { continue ; } if ! GeneralSecurityProfile :: identifier_allowed (ch) { continue ; } let augmented_script_set = AugmentedScriptSet :: for_char (ch) ; script_states . entry (augmented_script_set) . and_modify (| existing_state | { if let ScriptSetUsage :: Suspicious (ch_list , _) = existing_state { if is_potential_mixed_script_confusable_char (ch) { ch_list . push (ch) ; } else { * existing_state = ScriptSetUsage :: Verified ; } } }) . or_insert_with (| | { if ! is_potential_mixed_script_confusable_char (ch) { ScriptSetUsage :: Verified } else { has_suspicious = true ; ScriptSetUsage :: Suspicious (vec ! [ch] , sp) } }) ; } } if has_suspicious { # [allow (rustc :: potential_query_instability)] let verified_augmented_script_sets = script_states . iter () . flat_map (| (k , v) | match v { ScriptSetUsage :: Verified => Some (* k) , _ => None , }) . collect :: < Vec < _ > > () ; let mut lint_reports : BTreeMap < (Span , Vec < char >) , AugmentedScriptSet > = BTreeMap :: new () ; # [allow (rustc :: potential_query_instability)] 'outerloop : for (augment_script_set , usage) in script_states { let ScriptSetUsage :: Suspicious (mut ch_list , sp) = usage else { continue } ; if augment_script_set . is_all () { continue ; } for existing in verified_augmented_script_sets . iter () { if existing . is_all () { continue ; } let mut intersect = * existing ; intersect . intersect_with (augment_script_set) ; if ! intersect . is_empty () && ! intersect . is_all () { continue 'outerloop ; } } ch_list . sort_unstable () ; ch_list . dedup () ; lint_reports . insert ((sp , ch_list) , augment_script_set) ; } for ((sp , ch_list) , script_set) in lint_reports { let mut includes = String :: new () ; for (idx , ch) in ch_list . into_iter () . enumerate () { if idx != 0 { includes += ", " ; } let char_info = format ! ("'{}' (U+{:04X})" , ch , ch as u32) ; includes += & char_info ; } cx . emit_span_lint (MIXED_SCRIPT_CONFUSABLES , sp , MixedScriptConfusables { set : script_set . to_string () , includes } ,) ; } } } } }}}