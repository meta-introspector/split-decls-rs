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
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use rustc_ast :: AttrId ;}
mkuse!{use rustc_ast :: attr :: AttributeExt ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_data_structures :: stable_hasher :: { HashStable , StableCompare , StableHasher , ToStableHashKey , } ;}
mkuse!{use rustc_error_messages :: { DiagArgValue , DiagMessage , IntoDiagArg , MultiSpan } ;}
mkuse!{use rustc_hir_id :: { HashStableContext , HirId , ItemLocalId } ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable_Generic } ;}
mkuse!{use rustc_span :: def_id :: DefPathHash ;}
mkuse!{pub use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: { Ident , MacroRulesNormalizedIdent , Span , Symbol , sym } ;}
mkuse!{use serde :: { Deserialize , Serialize } ;}
mkuse!{pub use self :: Level :: * ;}
mkmod!{builtin, { 
                getname!(builtin);
                getsrc!(builtin);
                getpath!(builtin);
                get_deps!(builtin);
                get_crates!(builtin);
                mkinclude!(builtin);
                 
            }}
mkitem!{#[macro_export] macro_rules ! pluralize { ($ x : expr) => { if $ x == 1 { "" } else { "s" } } ; ("has" , $ x : expr) => { if $ x == 1 { "has" } else { "have" } } ; ("is" , $ x : expr) => { if $ x == 1 { "is" } else { "are" } } ; ("was" , $ x : expr) => { if $ x == 1 { "was" } else { "were" } } ; ("this" , $ x : expr) => { if $ x == 1 { "this" } else { "these" } } ; }}

macro_rules! listify_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function listify in module {}", module_path!());
    };
}

mkfn!{
    listify_introspect!();
    #[doc = " Grammatical tool for displaying messages to end users in a nice form."] #[doc = ""] #[doc = " Take a list of items and a function to turn those items into a `String`, and output a display"] #[doc = " friendly comma separated list of those items."] pub fn listify < T > (list : & [T] , fmt : impl Fn (& T) -> String) -> Option < String > { Some (match list { [only] => fmt (& only) , [others @ .. , last] => format ! ("{} and {}" , others . iter () . map (| i | fmt (i)) . collect ::< Vec < _ >> () . join (", ") , fmt (& last) ,) , [] => return None , }) }
}
mkitem!{mkenum!{#[doc = " Indicates the confidence in the correctness of a suggestion."] #[doc = ""] #[doc = " All suggestions are marked with an `Applicability`. Tools use the applicability of a suggestion"] #[doc = " to determine whether it should be automatically applied or if the user should be consulted"] #[doc = " before applying the suggestion."] #[derive (Copy , Clone , Debug , Hash , Encodable , Decodable , Serialize , Deserialize)] #[derive (PartialEq , Eq , PartialOrd , Ord)] pub enum Applicability { #[doc = " The suggestion is definitely what the user intended, or maintains the exact meaning of the code."] #[doc = " This suggestion should be automatically applied."] #[doc = ""] #[doc = " In case of multiple `MachineApplicable` suggestions (whether as part of"] #[doc = " the same `multipart_suggestion` or not), all of them should be"] #[doc = " automatically applied."] MachineApplicable , #[doc = " The suggestion may be what the user intended, but it is uncertain. The suggestion should"] #[doc = " result in valid Rust code if it is applied."] MaybeIncorrect , #[doc = " The suggestion contains placeholders like `(...)` or `{ /* fields */ }`. The suggestion"] #[doc = " cannot be applied automatically because it will not result in valid Rust code. The user"] #[doc = " will need to fill in the placeholders."] HasPlaceholders , #[doc = " The applicability of the suggestion is unknown."] Unspecified , }}}
mkitem!{mkenum!{#[doc = " Each lint expectation has a `LintExpectationId` assigned by the `LintLevelsBuilder`."] #[doc = " Expected diagnostics get the lint level `Expect` which stores the `LintExpectationId`"] #[doc = " to match it with the actual expectation later on."] #[doc = ""] #[doc = " The `LintExpectationId` has to be stable between compilations, as diagnostic"] #[doc = " instances might be loaded from cache. Lint messages can be emitted during an"] #[doc = " `EarlyLintPass` operating on the AST and during a `LateLintPass` traversing the"] #[doc = " HIR tree. The AST doesn't have enough information to create a stable id. The"] #[doc = " `LintExpectationId` will instead store the [`AttrId`] defining the expectation."] #[doc = " These `LintExpectationId` will be updated to use the stable [`HirId`] once the"] #[doc = " AST has been lowered. The transformation is done by the `LintLevelsBuilder`"] #[doc = ""] #[doc = " Each lint inside the `expect` attribute is tracked individually, the `lint_index`"] #[doc = " identifies the lint inside the attribute and ensures that the IDs are unique."] #[doc = ""] #[doc = " The index values have a type of `u16` to reduce the size of the `LintExpectationId`."] #[doc = " It's reasonable to assume that no user will define 2^16 attributes on one node or"] #[doc = " have that amount of lints listed. `u16` values should therefore suffice."] #[derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Encodable , Decodable)] pub enum LintExpectationId { #[doc = " Used for lints emitted during the `EarlyLintPass`. This id is not"] #[doc = " hash stable and should not be cached."] Unstable { attr_id : AttrId , lint_index : Option < u16 > } , #[doc = " The [`HirId`] that the lint expectation is attached to. This id is"] #[doc = " stable and can be cached. The additional index ensures that nodes with"] #[doc = " several expectations can correctly match diagnostics to the individual"] #[doc = " expectation."] Stable { hir_id : HirId , attr_index : u16 , lint_index : Option < u16 > } , }}}
mkitem!{mkimpl!{impl LintExpectationId { pub fn is_stable (& self) -> bool { match self { LintExpectationId :: Unstable { .. } => false , LintExpectationId :: Stable { .. } => true , } } pub fn get_lint_index (& self) -> Option < u16 > { let (LintExpectationId :: Unstable { lint_index , .. } | LintExpectationId :: Stable { lint_index , .. }) = self ; * lint_index } pub fn set_lint_index (& mut self , new_lint_index : Option < u16 >) { let (LintExpectationId :: Unstable { lint_index , .. } | LintExpectationId :: Stable { lint_index , .. }) = self ; * lint_index = new_lint_index } }}}
mkitem!{mkimpl!{impl < HCX : HashStableContext > HashStable < HCX > for LintExpectationId { #[inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { match self { LintExpectationId :: Stable { hir_id , attr_index , lint_index : Some (lint_index) } => { hir_id . hash_stable (hcx , hasher) ; attr_index . hash_stable (hcx , hasher) ; lint_index . hash_stable (hcx , hasher) ; } _ => { unreachable ! ("HashStable should only be called for filled and stable `LintExpectationId`") } } } }}}
mkitem!{mkimpl!{impl < HCX : HashStableContext > ToStableHashKey < HCX > for LintExpectationId { type KeyType = (DefPathHash , ItemLocalId , u16 , u16) ; #[inline] fn to_stable_hash_key (& self , hcx : & HCX) -> Self :: KeyType { match self { LintExpectationId :: Stable { hir_id , attr_index , lint_index : Some (lint_index) } => { let (def_path_hash , lint_idx) = hir_id . to_stable_hash_key (hcx) ; (def_path_hash , lint_idx , * attr_index , * lint_index) } _ => { unreachable ! ("HashStable should only be called for a filled `LintExpectationId`") } } } }}}
mkitem!{mkenum!{#[doc = " Setting for how to handle a lint."] #[doc = ""] #[doc = " See: <https://doc.rust-lang.org/rustc/lints/levels.html>"] #[derive (Clone , Copy , PartialEq , PartialOrd , Eq , Ord , Debug , Hash , Encodable , Decodable , HashStable_Generic)] pub enum Level { #[doc = " The `allow` level will not issue any message."] Allow , #[doc = " The `expect` level will suppress the lint message but in turn produce a message"] #[doc = " if the lint wasn't issued in the expected scope. `Expect` should not be used as"] #[doc = " an initial level for a lint."] #[doc = ""] #[doc = " Note that this still means that the lint is enabled in this position and should"] #[doc = " be emitted, this will in turn fulfill the expectation and suppress the lint."] #[doc = ""] #[doc = " See RFC 2383."] #[doc = ""] #[doc = " Requires a [`LintExpectationId`] to later link a lint emission to the actual"] #[doc = " expectation. It can be ignored in most cases."] Expect , #[doc = " The `warn` level will produce a warning if the lint was violated, however the"] #[doc = " compiler will continue with its execution."] Warn , #[doc = " This lint level is a special case of [`Warn`], that can't be overridden. This is used"] #[doc = " to ensure that a lint can't be suppressed. This lint level can currently only be set"] #[doc = " via the console and is therefore session specific."] #[doc = ""] #[doc = " Requires a [`LintExpectationId`] to fulfill expectations marked via the"] #[doc = " `#[expect]` attribute, that will still be suppressed due to the level."] ForceWarn , #[doc = " The `deny` level will produce an error and stop further execution after the lint"] #[doc = " pass is complete."] Deny , #[doc = " `Forbid` is equivalent to the `deny` level but can't be overwritten like the previous"] #[doc = " levels."] Forbid , }}}
mkitem!{mkimpl!{impl Level { #[doc = " Converts a level to a lower-case string."] pub fn as_str (self) -> & 'static str { match self { Level :: Allow => "allow" , Level :: Expect => "expect" , Level :: Warn => "warn" , Level :: ForceWarn => "force-warn" , Level :: Deny => "deny" , Level :: Forbid => "forbid" , } } #[doc = " Converts a lower-case string to a level. This will never construct the expect"] #[doc = " level as that would require a [`LintExpectationId`]."] pub fn from_str (x : & str) -> Option < Self > { match x { "allow" => Some (Level :: Allow) , "warn" => Some (Level :: Warn) , "deny" => Some (Level :: Deny) , "forbid" => Some (Level :: Forbid) , "expect" | _ => None , } } #[doc = " Converts an `Attribute` to a level."] pub fn from_attr (attr : & impl AttributeExt) -> Option < (Self , Option < LintExpectationId >) > { attr . name () . and_then (| name | Self :: from_symbol (name , | | Some (attr . id ()))) } #[doc = " Converts a `Symbol` to a level."] pub fn from_symbol (s : Symbol , id : impl FnOnce () -> Option < AttrId > ,) -> Option < (Self , Option < LintExpectationId >) > { match s { sym :: allow => Some ((Level :: Allow , None)) , sym :: expect => { if let Some (attr_id) = id () { Some ((Level :: Expect , Some (LintExpectationId :: Unstable { attr_id , lint_index : None }) ,)) } else { None } } sym :: warn => Some ((Level :: Warn , None)) , sym :: deny => Some ((Level :: Deny , None)) , sym :: forbid => Some ((Level :: Forbid , None)) , _ => None , } } pub fn to_cmd_flag (self) -> & 'static str { match self { Level :: Warn => "-W" , Level :: Deny => "-D" , Level :: Forbid => "-F" , Level :: Allow => "-A" , Level :: ForceWarn => "--force-warn" , Level :: Expect => { unreachable ! ("the expect level does not have a commandline flag") } } } pub fn is_error (self) -> bool { match self { Level :: Allow | Level :: Expect | Level :: Warn | Level :: ForceWarn => false , Level :: Deny | Level :: Forbid => true , } } }}}
mkitem!{mkimpl!{impl IntoDiagArg for Level { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . to_cmd_flag ())) } }}}
mkitem!{mkstruct!{#[doc = " Specification of a single lint."] #[derive (Copy , Clone , Debug)] pub struct Lint { #[doc = " A string identifier for the lint."] #[doc = ""] #[doc = " This identifies the lint in attributes and in command-line arguments."] #[doc = " In those contexts it is always lowercase, but this field is compared"] #[doc = " in a way which is case-insensitive for ASCII characters. This allows"] #[doc = " `declare_lint!()` invocations to follow the convention of upper-case"] #[doc = " statics without repeating the name."] #[doc = ""] #[doc = " The name is written with underscores, e.g., \"unused_imports\"."] #[doc = " On the command line, underscores become dashes."] #[doc = ""] #[doc = " See <https://rustc-dev-guide.rust-lang.org/diagnostics.html#lint-naming>"] #[doc = " for naming guidelines."] pub name : & 'static str , #[doc = " Default level for the lint."] #[doc = ""] #[doc = " See <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-levels>"] #[doc = " for guidelines on choosing a default level."] pub default_level : Level , #[doc = " Description of the lint or the issue it detects."] #[doc = ""] #[doc = " e.g., \"imports that are never used\""] pub desc : & 'static str , #[doc = " Starting at the given edition, default to the given lint level. If this is `None`, then use"] #[doc = " `default_level`."] pub edition_lint_opts : Option < (Edition , Level) > , #[doc = " `true` if this lint is reported even inside expansions of external macros."] pub report_in_external_macro : bool , pub future_incompatible : Option < FutureIncompatibleInfo > , #[doc = " `true` if this lint is being loaded by another tool (e.g. Clippy)."] pub is_externally_loaded : bool , #[doc = " `Some` if this lint is feature gated, otherwise `None`."] pub feature_gate : Option < Symbol > , pub crate_level_only : bool , #[doc = " `true` if this lint should not be filtered out under any circustamces"] #[doc = " (e.g. the unknown_attributes lint)"] pub eval_always : bool , }}}
mkitem!{mkstruct!{#[doc = " Extra information for a future incompatibility lint."] #[derive (Copy , Clone , Debug)] pub struct FutureIncompatibleInfo { #[doc = " e.g., a URL for an issue/PR/RFC or error code"] pub reference : & 'static str , #[doc = " The reason for the lint used by diagnostics to provide"] #[doc = " the right help message"] pub reason : FutureIncompatibilityReason , #[doc = " Whether to explain the reason to the user."] #[doc = ""] #[doc = " Set to false for lints that already include a more detailed"] #[doc = " explanation."] pub explain_reason : bool , #[doc = " If set to `true`, this will make future incompatibility warnings show up in cargo's"] #[doc = " reports."] #[doc = ""] #[doc = " When a future incompatibility warning is first inroduced, set this to `false`"] #[doc = " (or, rather, don't override the default). This allows crate developers an opportunity"] #[doc = " to fix the warning before blasting all dependents with a warning they can't fix"] #[doc = " (dependents have to wait for a new release of the affected crate to be published)."] #[doc = ""] #[doc = " After a lint has been in this state for a while, consider setting this to true, so it"] #[doc = " warns for everyone. It is a good signal that it is ready if you can determine that all"] #[doc = " or most affected crates on crates.io have been updated."] pub report_in_deps : bool , }}}
mkitem!{mkenum!{#[doc = " The reason for future incompatibility"] #[doc = ""] #[doc = " Future-incompatible lints come in roughly two categories:"] #[doc = ""] #[doc = " 1. There was a mistake in the compiler (such as a soundness issue), and"] #[doc = "    we're trying to fix it, but it may be a breaking change."] #[doc = " 2. A change across an Edition boundary, typically used for the"] #[doc = "    introduction of new language features that can't otherwise be"] #[doc = "    introduced in a backwards-compatible way."] #[doc = ""] #[doc = " See <https://rustc-dev-guide.rust-lang.org/bug-fix-procedure.html> and"] #[doc = " <https://rustc-dev-guide.rust-lang.org/diagnostics.html#future-incompatible-lints>"] #[doc = " for more information."] #[derive (Copy , Clone , Debug)] pub enum FutureIncompatibilityReason { #[doc = " This will be an error in a future release for all editions"] #[doc = ""] #[doc = " Choose this variant when you are first introducing a \"future"] #[doc = " incompatible\" warning that is intended to eventually be fixed in the"] #[doc = " future."] #[doc = ""] #[doc = " After a lint has been in this state for a while and you feel like it is ready to graduate"] #[doc = " to warning everyone, consider setting [`FutureIncompatibleInfo::report_in_deps`] to true."] #[doc = " (see it's documentation for more guidance)"] #[doc = ""] #[doc = " After some period of time, lints with this variant can be turned into"] #[doc = " hard errors (and the lint removed). Preferably when there is some"] #[doc = " confidence that the number of impacted projects is very small (few"] #[doc = " should have a broken dependency in their dependency tree)."] FutureReleaseError , #[doc = " Code that changes meaning in some way in a"] #[doc = " future release."] #[doc = ""] #[doc = " Choose this variant when the semantics of existing code is changing,"] #[doc = " (as opposed to [`FutureIncompatibilityReason::FutureReleaseError`],"] #[doc = " which is for when code is going to be rejected in the future)."] FutureReleaseSemanticsChange , #[doc = " Previously accepted code that will become an"] #[doc = " error in the provided edition"] #[doc = ""] #[doc = " Choose this variant for code that you want to start rejecting across"] #[doc = " an edition boundary. This will automatically include the lint in the"] #[doc = " `rust-20xx-compatibility` lint group, which is used by `cargo fix"] #[doc = " --edition` to do migrations. The lint *should* be auto-fixable with"] #[doc = " [`Applicability::MachineApplicable`]."] #[doc = ""] #[doc = " The lint can either be `Allow` or `Warn` by default. If it is `Allow`,"] #[doc = " users usually won't see this warning unless they are doing an edition"] #[doc = " migration manually or there is a problem during the migration (cargo's"] #[doc = " automatic migrations will force the level to `Warn`). If it is `Warn`"] #[doc = " by default, users on all editions will see this warning (only do this"] #[doc = " if you think it is important for everyone to be aware of the change,"] #[doc = " and to encourage people to update their code on all editions)."] #[doc = ""] #[doc = " See also [`FutureIncompatibilityReason::EditionSemanticsChange`] if"] #[doc = " you have code that is changing semantics across the edition (as"] #[doc = " opposed to being rejected)."] EditionError (Edition) , #[doc = " Code that changes meaning in some way in"] #[doc = " the provided edition"] #[doc = ""] #[doc = " This is the same as [`FutureIncompatibilityReason::EditionError`],"] #[doc = " except for situations where the semantics change across an edition. It"] #[doc = " slightly changes the text of the diagnostic, but is otherwise the"] #[doc = " same."] EditionSemanticsChange (Edition) , #[doc = " This will be an error in the provided edition *and* in a future"] #[doc = " release."] #[doc = ""] #[doc = " This variant a combination of [`FutureReleaseError`] and [`EditionError`]."] #[doc = " This is useful in rare cases when we want to have \"preview\" of a breaking"] #[doc = " change in an edition, but do a breaking change later on all editions anyway."] #[doc = ""] #[doc = " [`EditionError`]: FutureIncompatibilityReason::EditionError"] #[doc = " [`FutureReleaseError`]: FutureIncompatibilityReason::FutureReleaseError"] EditionAndFutureReleaseError (Edition) , #[doc = " This will change meaning in the provided edition *and* in a future"] #[doc = " release."] #[doc = ""] #[doc = " This variant a combination of [`FutureReleaseSemanticsChange`]"] #[doc = " and [`EditionSemanticsChange`]. This is useful in rare cases when we"] #[doc = " want to have \"preview\" of a breaking change in an edition, but do a"] #[doc = " breaking change later on all editions anyway."] #[doc = ""] #[doc = " [`EditionSemanticsChange`]: FutureIncompatibilityReason::EditionSemanticsChange"] #[doc = " [`FutureReleaseSemanticsChange`]: FutureIncompatibilityReason::FutureReleaseSemanticsChange"] EditionAndFutureReleaseSemanticsChange (Edition) , #[doc = " A custom reason."] #[doc = ""] #[doc = " Choose this variant if the built-in text of the diagnostic of the"] #[doc = " other variants doesn't match your situation. This is behaviorally"] #[doc = " equivalent to"] #[doc = " [`FutureIncompatibilityReason::FutureReleaseError`]."] Custom (& 'static str) , }}}
mkitem!{mkimpl!{impl FutureIncompatibilityReason { pub fn edition (self) -> Option < Edition > { match self { Self :: EditionError (e) | Self :: EditionSemanticsChange (e) | Self :: EditionAndFutureReleaseError (e) | Self :: EditionAndFutureReleaseSemanticsChange (e) => Some (e) , FutureIncompatibilityReason :: FutureReleaseError | FutureIncompatibilityReason :: FutureReleaseSemanticsChange | FutureIncompatibilityReason :: Custom (_) => None , } } }}}
mkitem!{mkimpl!{impl FutureIncompatibleInfo { pub const fn default_fields_for_macro () -> Self { FutureIncompatibleInfo { reference : "" , reason : FutureIncompatibilityReason :: FutureReleaseError , explain_reason : true , report_in_deps : false , } } }}}
mkitem!{mkimpl!{impl Lint { pub const fn default_fields_for_macro () -> Self { Lint { name : "" , default_level : Level :: Forbid , desc : "" , edition_lint_opts : None , is_externally_loaded : false , report_in_external_macro : false , future_incompatible : None , feature_gate : None , crate_level_only : false , eval_always : false , } } #[doc = " Gets the lint's name, with ASCII letters converted to lowercase."] pub fn name_lower (& self) -> String { self . name . to_ascii_lowercase () } pub fn default_level (& self , edition : Edition) -> Level { self . edition_lint_opts . filter (| (e , _) | * e <= edition) . map (| (_ , l) | l) . unwrap_or (self . default_level) } }}}
mkitem!{mkstruct!{#[doc = " Identifies a lint known to the compiler."] #[derive (Clone , Copy , Debug)] pub struct LintId { pub lint : & 'static Lint , }}}
mkitem!{mkimpl!{impl PartialEq for LintId { fn eq (& self , other : & LintId) -> bool { std :: ptr :: eq (self . lint , other . lint) } }}}
mkitem!{mkimpl!{impl Eq for LintId { }}}
mkitem!{mkimpl!{impl std :: hash :: Hash for LintId { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { let ptr = self . lint as * const Lint ; ptr . hash (state) ; } }}}
mkitem!{mkimpl!{impl LintId { #[doc = " Gets the `LintId` for a `Lint`."] pub fn of (lint : & 'static Lint) -> LintId { LintId { lint } } pub fn lint_name_raw (& self) -> & 'static str { self . lint . name } #[doc = " Gets the name of the lint."] pub fn to_string (& self) -> String { self . lint . name_lower () } }}}
mkitem!{mkimpl!{impl < HCX > HashStable < HCX > for LintId { #[inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . lint_name_raw () . hash_stable (hcx , hasher) ; } }}}
mkitem!{mkimpl!{impl < HCX > ToStableHashKey < HCX > for LintId { type KeyType = & 'static str ; #[inline] fn to_stable_hash_key (& self , _ : & HCX) -> & 'static str { self . lint_name_raw () } }}}
mkitem!{mkimpl!{impl StableCompare for LintId { const CAN_USE_UNSTABLE_SORT : bool = true ; fn stable_cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . lint_name_raw () . cmp (& other . lint_name_raw ()) } }}}
mkitem!{mkstruct!{#[derive (Debug)] pub struct AmbiguityErrorDiag { pub msg : String , pub span : Span , pub label_span : Span , pub label_msg : String , pub note_msg : String , pub b1_span : Span , pub b1_note_msg : String , pub b1_help_msgs : Vec < String > , pub b2_span : Span , pub b2_note_msg : String , pub b2_help_msgs : Vec < String > , }}}
mkitem!{mkenum!{#[derive (Debug , Clone)] pub enum DeprecatedSinceKind { InEffect , InFuture , InVersion (String) , }}}
mkitem!{mkenum!{#[derive (Debug)] pub enum BuiltinLintDiag { AbsPathWithModule (Span) , ProcMacroDeriveResolutionFallback { span : Span , ns_descr : & 'static str , ident : Ident , } , MacroExpandedMacroExportsAccessedByAbsolutePaths (Span) , ElidedLifetimesInPaths (usize , Span , bool , Span) , UnknownCrateTypes { span : Span , candidate : Option < Symbol > , } , UnusedImports { remove_whole_use : bool , num_to_remove : usize , remove_spans : Vec < Span > , test_module_span : Option < Span > , span_snippets : Vec < String > , } , RedundantImport (Vec < (Span , bool) > , Ident) , DeprecatedMacro { suggestion : Option < Symbol > , suggestion_span : Span , note : Option < Symbol > , path : String , since_kind : DeprecatedSinceKind , } , UnusedDocComment (Span) , UnusedBuiltinAttribute { attr_name : Symbol , macro_name : String , invoc_span : Span , attr_span : Span , } , PatternsInFnsWithoutBody { span : Span , ident : Ident , is_foreign : bool , } , LegacyDeriveHelpers (Span) , OrPatternsBackCompat (Span , String) , ReservedPrefix (Span , String) , #[doc = " `'r#` in edition < 2021."] RawPrefix (Span) , #[doc = " `##` or `#\"` in edition < 2024."] ReservedString { is_string : bool , suggestion : Span , } , TrailingMacro (bool , Ident) , BreakWithLabelAndLoop (Span) , UnicodeTextFlow (Span , String) , UnexpectedCfgName ((Symbol , Span) , Option < (Symbol , Span) >) , UnexpectedCfgValue ((Symbol , Span) , Option < (Symbol , Span) >) , DeprecatedWhereclauseLocation (Span , Option < (Span , String) >) , MissingUnsafeOnExtern { suggestion : Span , } , SingleUseLifetime { #[doc = " Span of the parameter which declares this lifetime."] param_span : Span , #[doc = " Span of the code that should be removed when eliding this lifetime."] #[doc = " This span should include leading or trailing comma."] deletion_span : Option < Span > , #[doc = " Span of the single use, or None if the lifetime is never used."] #[doc = " If true, the lifetime will be fully elided."] use_span : Option < (Span , bool) > , ident : Ident , } , NamedArgumentUsedPositionally { #[doc = " Span where the named argument is used by position and will be replaced with the named"] #[doc = " argument name"] position_sp_to_replace : Option < Span > , #[doc = " Span where the named argument is used by position and is used for lint messages"] position_sp_for_msg : Option < Span > , #[doc = " Span where the named argument's name is (so we know where to put the warning message)"] named_arg_sp : Span , #[doc = " String containing the named arguments name"] named_arg_name : String , #[doc = " Indicates if the named argument is used as a width/precision for formatting"] is_formatting_arg : bool , } , ByteSliceInPackedStructWithDerive { ty : String , } , UnusedExternCrate { span : Span , removal_span : Span , } , ExternCrateNotIdiomatic { vis_span : Span , ident_span : Span , } , AmbiguousGlobImports { diag : AmbiguityErrorDiag , } , AmbiguousGlobReexports { #[doc = " The name for which collision(s) have occurred."] name : String , #[doc = " The name space for which the collision(s) occurred in."] namespace : String , #[doc = " Span where the name is first re-exported."] first_reexport_span : Span , #[doc = " Span where the same name is also re-exported."] duplicate_reexport_span : Span , } , HiddenGlobReexports { #[doc = " The name of the local binding which shadows the glob re-export."] name : String , #[doc = " The namespace for which the shadowing occurred in."] namespace : String , #[doc = " The glob reexport that is shadowed by the local binding."] glob_reexport_span : Span , #[doc = " The local binding that shadows the glob reexport."] private_item_span : Span , } , ReexportPrivateDependency { name : String , kind : String , krate : Symbol , } , UnusedQualifications { #[doc = " The span of the unnecessarily-qualified path to remove."] removal_span : Span , } , UnsafeAttrOutsideUnsafe { attribute_name_span : Span , sugg_spans : (Span , Span) , } , AssociatedConstElidedLifetime { elided : bool , span : Span , lifetimes_in_scope : MultiSpan , } , RedundantImportVisibility { span : Span , max_vis : String , import_vis : String , } , UnknownDiagnosticAttribute { span : Span , typo_name : Option < Symbol > , } , MacroUseDeprecated , UnusedMacroUse , PrivateExternCrateReexport { source : Ident , extern_crate_span : Span , } , UnusedLabel , MacroIsPrivate (Ident) , UnusedMacroDefinition (Symbol) , MacroRuleNeverUsed (usize , Symbol) , UnstableFeature (DiagMessage) , AvoidUsingIntelSyntax , AvoidUsingAttSyntax , IncompleteInclude , UnnameableTestItems , DuplicateMacroAttribute , CfgAttrNoAttributes , MetaVariableStillRepeating (MacroRulesNormalizedIdent) , MetaVariableWrongOperator , DuplicateMatcherBinding , UnknownMacroVariable (MacroRulesNormalizedIdent) , UnusedCrateDependency { extern_crate : Symbol , local_crate : Symbol , } , IllFormedAttributeInput { suggestions : Vec < String > , docs : Option < & 'static str > , } , OutOfScopeMacroCalls { span : Span , path : String , location : String , } , }}}
mkitem!{pub type RegisteredTools = FxIndexSet < Ident > ;}
mkitem!{#[doc = " Declares a static item of type `&'static Lint`."] #[doc = ""] #[doc = " See <https://rustc-dev-guide.rust-lang.org/diagnostics.html> for"] #[doc = " documentation and guidelines on writing lints."] #[doc = ""] #[doc = " The macro call should start with a doc comment explaining the lint"] #[doc = " which will be embedded in the rustc user documentation book. It should"] #[doc = " be written in markdown and have a format that looks like this:"] #[doc = ""] #[doc = " ```rust,ignore (doc-example)"] #[doc = " /// The `my_lint_name` lint detects [short explanation here]."] #[doc = " ///"] #[doc = " /// ### Example"] #[doc = " ///"] #[doc = " /// ```rust"] #[doc = " /// [insert a concise example that triggers the lint]"] #[doc = " /// ```"] #[doc = " ///"] #[doc = " /// {{produces}}"] #[doc = " ///"] #[doc = " /// ### Explanation"] #[doc = " ///"] #[doc = " /// This should be a detailed explanation of *why* the lint exists,"] #[doc = " /// and also include suggestions on how the user should fix the problem."] #[doc = " /// Try to keep the text simple enough that a beginner can understand,"] #[doc = " /// and include links to other documentation for terminology that a"] #[doc = " /// beginner may not be familiar with. If this is \"allow\" by default,"] #[doc = " /// it should explain why (are there false positives or other issues?). If"] #[doc = " /// this is a future-incompatible lint, it should say so, with text that"] #[doc = " /// looks roughly like this:"] #[doc = " ///"] #[doc = " /// This is a [future-incompatible] lint to transition this to a hard"] #[doc = " /// error in the future. See [issue #xxxxx] for more details."] #[doc = " ///"] #[doc = " /// [issue #xxxxx]: https://github.com/rust-lang/rust/issues/xxxxx"] #[doc = " ```"] #[doc = ""] #[doc = " The `{{produces}}` tag will be automatically replaced with the output from"] #[doc = " the example by the build system. If the lint example is too complex to run"] #[doc = " as a simple example (for example, it needs an extern crate), mark the code"] #[doc = " block with `ignore` and manually replace the `{{produces}}` line with the"] #[doc = " expected output in a `text` code block."] #[doc = ""] #[doc = " If this is a rustdoc-only lint, then only include a brief introduction"] #[doc = " with a link with the text `[rustdoc book]` so that the validator knows"] #[doc = " that this is for rustdoc only (see BROKEN_INTRA_DOC_LINKS as an example)."] #[doc = ""] #[doc = " Commands to view and test the documentation:"] #[doc = ""] #[doc = " * `./x.py doc --stage=1 src/doc/rustc --open`: Builds the rustc book and opens it."] #[doc = " * `./x.py test src/tools/lint-docs`: Validates that the lint docs have the"] #[doc = "   correct style, and that the code example actually emits the expected"] #[doc = "   lint."] #[doc = ""] #[doc = " If you have already built the compiler, and you want to make changes to"] #[doc = " just the doc comments, then use the `--keep-stage=0` flag with the above"] #[doc = " commands to avoid rebuilding the compiler."] #[macro_export] macro_rules ! declare_lint { ($ (#[$ attr : meta]) * $ vis : vis $ NAME : ident , $ Level : ident , $ desc : expr) => ($ crate :: declare_lint ! ($ (#[$ attr]) * $ vis $ NAME , $ Level , $ desc ,) ;) ; ($ (#[$ attr : meta]) * $ vis : vis $ NAME : ident , $ Level : ident , $ desc : expr , $ (@ eval_always = $ eval_always : literal) ? $ (@ feature_gate = $ gate : ident ;) ? $ (@ future_incompatible = FutureIncompatibleInfo { reason : $ reason : expr , $ ($ field : ident : $ val : expr) ,* $ (,) * } ;) ? $ (@ edition $ lint_edition : ident => $ edition_level : ident ;) ? $ ($ v : ident) ,*) => ($ (#[$ attr]) * $ vis static $ NAME : &$ crate :: Lint = &$ crate :: Lint { name : stringify ! ($ NAME) , default_level : $ crate ::$ Level , desc : $ desc , is_externally_loaded : false , $ ($ v : true ,) * $ (feature_gate : Some (rustc_span :: sym ::$ gate) ,) ? $ (future_incompatible : Some ($ crate :: FutureIncompatibleInfo { reason : $ reason , $ ($ field : $ val ,) * ..$ crate :: FutureIncompatibleInfo :: default_fields_for_macro () }) ,) ? $ (edition_lint_opts : Some (($ crate :: Edition ::$ lint_edition , $ crate ::$ edition_level)) ,) ? $ (eval_always : $ eval_always ,) ? ..$ crate :: Lint :: default_fields_for_macro () } ;) ; }}
mkitem!{#[macro_export] macro_rules ! declare_tool_lint { ($ (#[$ attr : meta]) * $ vis : vis $ tool : ident ::$ NAME : ident , $ Level : ident , $ desc : expr $ (, @ eval_always = $ eval_always : literal) ? $ (, @ feature_gate = $ gate : ident ;) ?) => ($ crate :: declare_tool_lint ! { $ (#[$ attr]) * $ vis $ tool ::$ NAME , $ Level , $ desc , false $ (, @ eval_always = $ eval_always) ? $ (, @ feature_gate = $ gate ;) ? }) ; ($ (#[$ attr : meta]) * $ vis : vis $ tool : ident ::$ NAME : ident , $ Level : ident , $ desc : expr , report_in_external_macro : $ rep : expr $ (, @ eval_always = $ eval_always : literal) ? $ (, @ feature_gate = $ gate : ident ;) ?) => ($ crate :: declare_tool_lint ! { $ (#[$ attr]) * $ vis $ tool ::$ NAME , $ Level , $ desc , $ rep $ (, @ eval_always = $ eval_always) ? $ (, @ feature_gate = $ gate ;) ? }) ; ($ (#[$ attr : meta]) * $ vis : vis $ tool : ident ::$ NAME : ident , $ Level : ident , $ desc : expr , $ external : expr $ (, @ eval_always = $ eval_always : literal) ? $ (, @ feature_gate = $ gate : ident ;) ?) => ($ (#[$ attr]) * $ vis static $ NAME : &$ crate :: Lint = &$ crate :: Lint { name : & concat ! (stringify ! ($ tool) , "::" , stringify ! ($ NAME)) , default_level : $ crate ::$ Level , desc : $ desc , edition_lint_opts : None , report_in_external_macro : $ external , future_incompatible : None , is_externally_loaded : true , $ (feature_gate : Some (rustc_span :: sym ::$ gate) ,) ? crate_level_only : false , $ (eval_always : $ eval_always ,) ? ..$ crate :: Lint :: default_fields_for_macro () } ;) ; }}
mkitem!{pub type LintVec = Vec < & 'static Lint > ;}
mkitem!{mktrait!{pub trait LintPass { fn name (& self) -> & 'static str ; fn get_lints (& self) -> LintVec ; }}}
mkitem!{#[doc = " Implements `LintPass for $ty` with the given list of `Lint` statics."] #[macro_export] macro_rules ! impl_lint_pass { ($ ty : ty => [$ ($ lint : expr) ,* $ (,) ?]) => { impl $ crate :: LintPass for $ ty { fn name (& self) -> &'static str { stringify ! ($ ty) } fn get_lints (& self) -> $ crate :: LintVec { vec ! [$ ($ lint) ,*] } } impl $ ty { #[allow (unused)] pub fn lint_vec () -> $ crate :: LintVec { vec ! [$ ($ lint) ,*] } } } ; }}
mkitem!{#[doc = " Declares a type named `$name` which implements `LintPass`."] #[doc = " To the right of `=>` a comma separated list of `Lint` statics is given."] #[macro_export] macro_rules ! declare_lint_pass { ($ (#[$ m : meta]) * $ name : ident => [$ ($ lint : expr) ,* $ (,) ?]) => { $ (#[$ m]) * #[derive (Copy , Clone)] pub struct $ name ; $ crate :: impl_lint_pass ! ($ name => [$ ($ lint) ,*]) ; } ; }}