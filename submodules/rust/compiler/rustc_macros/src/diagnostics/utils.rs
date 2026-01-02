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
mkuse!{use std :: cell :: RefCell ;}
mkuse!{use std :: collections :: { BTreeSet , HashMap } ;}
mkuse!{use std :: fmt ;}
mkuse!{use std :: str :: FromStr ;}
mkuse!{use proc_macro :: Span ;}
mkuse!{use proc_macro2 :: { Ident , TokenStream } ;}
mkuse!{use quote :: { ToTokens , format_ident , quote } ;}
mkuse!{use syn :: meta :: ParseNestedMeta ;}
mkuse!{use syn :: punctuated :: Punctuated ;}
mkuse!{use syn :: spanned :: Spanned ;}
mkuse!{use syn :: { Attribute , Field , LitStr , Meta , Path , Token , Type , TypeTuple , parenthesized } ;}
mkuse!{use synstructure :: { BindingInfo , VariantInfo } ;}
mkuse!{use super :: error :: invalid_attr ;}
mkuse!{use crate :: diagnostics :: error :: { DiagnosticDeriveError , span_err , throw_invalid_attr , throw_span_err , } ;}
mkitem!{thread_local ! { pub (crate) static CODE_IDENT_COUNT : RefCell < u32 > = RefCell :: new (0) ; }}

macro_rules! new_code_ident_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_code_ident in module {}", module_path!());
    };
}

mkfn!{
    new_code_ident_introspect!();
    #[doc = " Returns an ident of the form `__code_N` where `N` is incremented once with every call."] pub (crate) fn new_code_ident () -> syn :: Ident { CODE_IDENT_COUNT . with (| count | { let ident = format_ident ! ("__code_{}" , * count . borrow ()) ; * count . borrow_mut () += 1 ; ident }) }
}

macro_rules! type_matches_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_matches_path in module {}", module_path!());
    };
}

mkfn!{
    type_matches_path_introspect!();
    #[doc = " Checks whether the type name of `ty` matches `name`."] #[doc = ""] #[doc = " Given some struct at `a::b::c::Foo`, this will return true for `c::Foo`, `b::c::Foo`, or"] #[doc = " `a::b::c::Foo`. This reasonably allows qualified names to be used in the macro."] pub (crate) fn type_matches_path (ty : & Type , name : & [& str]) -> bool { if let Type :: Path (ty) = ty { ty . path . segments . iter () . map (| s | s . ident . to_string ()) . rev () . zip (name . iter () . rev ()) . all (| (x , y) | & x . as_str () == y) } else { false } }
}

macro_rules! type_is_unit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_is_unit in module {}", module_path!());
    };
}

mkfn!{
    type_is_unit_introspect!();
    #[doc = " Checks whether the type `ty` is `()`."] pub (crate) fn type_is_unit (ty : & Type) -> bool { if let Type :: Tuple (TypeTuple { elems , .. }) = ty { elems . is_empty () } else { false } }
}

macro_rules! type_is_bool_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_is_bool in module {}", module_path!());
    };
}

mkfn!{
    type_is_bool_introspect!();
    #[doc = " Checks whether the type `ty` is `bool`."] pub (crate) fn type_is_bool (ty : & Type) -> bool { type_matches_path (ty , & ["bool"]) }
}

macro_rules! report_type_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_type_error in module {}", module_path!());
    };
}

mkfn!{
    report_type_error_introspect!();
    #[doc = " Reports a type error for field with `attr`."] pub (crate) fn report_type_error (attr : & Attribute , ty_name : & str ,) -> Result < ! , DiagnosticDeriveError > { let name = attr . path () . segments . last () . unwrap () . ident . to_string () ; let meta = & attr . meta ; throw_span_err ! (attr . span () . unwrap () , & format ! ("the `#[{}{}]` attribute can only be applied to fields of type {}" , name , match meta { Meta :: Path (_) => "" , Meta :: NameValue (_) => " = ..." , Meta :: List (_) => "(...)" , } , ty_name)) ; }
}

macro_rules! report_error_if_not_applied_to_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_error_if_not_applied_to_ty in module {}", module_path!());
    };
}

mkfn!{
    report_error_if_not_applied_to_ty_introspect!();
    #[doc = " Reports an error if the field's type does not match `path`."] fn report_error_if_not_applied_to_ty (attr : & Attribute , info : & FieldInfo < '_ > , path : & [& str] , ty_name : & str ,) -> Result < () , DiagnosticDeriveError > { if ! type_matches_path (info . ty . inner_type () , path) { report_type_error (attr , ty_name) ? ; } Ok (()) }
}

macro_rules! report_error_if_not_applied_to_applicability_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_error_if_not_applied_to_applicability in module {}", module_path!());
    };
}

mkfn!{
    report_error_if_not_applied_to_applicability_introspect!();
    #[doc = " Reports an error if the field's type is not `Applicability`."] pub (crate) fn report_error_if_not_applied_to_applicability (attr : & Attribute , info : & FieldInfo < '_ > ,) -> Result < () , DiagnosticDeriveError > { report_error_if_not_applied_to_ty (attr , info , & ["rustc_errors" , "Applicability"] , "`Applicability`" ,) }
}

macro_rules! report_error_if_not_applied_to_span_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_error_if_not_applied_to_span in module {}", module_path!());
    };
}

mkfn!{
    report_error_if_not_applied_to_span_introspect!();
    #[doc = " Reports an error if the field's type is not `Span`."] pub (crate) fn report_error_if_not_applied_to_span (attr : & Attribute , info : & FieldInfo < '_ > ,) -> Result < () , DiagnosticDeriveError > { if ! type_matches_path (info . ty . inner_type () , & ["rustc_span" , "Span"]) && ! type_matches_path (info . ty . inner_type () , & ["rustc_errors" , "MultiSpan"]) { report_type_error (attr , "`Span` or `MultiSpan`") ? ; } Ok (()) }
}
mkitem!{mkenum!{#[doc = " Inner type of a field and type of wrapper."] #[derive (Copy , Clone)] pub (crate) enum FieldInnerTy < 'ty > { #[doc = " Field is wrapped in a `Option<$inner>`."] Option (& 'ty Type) , #[doc = " Field is wrapped in a `Vec<$inner>`."] Vec (& 'ty Type) , #[doc = " Field isn't wrapped in an outer type."] Plain (& 'ty Type) , }}}
mkitem!{mkimpl!{impl < 'ty > FieldInnerTy < 'ty > { #[doc = " Returns inner type for a field, if there is one."] #[doc = ""] #[doc = " - If `ty` is an `Option<Inner>`, returns `FieldInnerTy::Option(Inner)`."] #[doc = " - If `ty` is a `Vec<Inner>`, returns `FieldInnerTy::Vec(Inner)`."] #[doc = " - Otherwise returns `FieldInnerTy::Plain(ty)`."] pub (crate) fn from_type (ty : & 'ty Type) -> Self { fn single_generic_type (ty : & Type) -> & Type { let Type :: Path (ty_path) = ty else { panic ! ("expected path type") ; } ; let path = & ty_path . path ; let ty = path . segments . last () . unwrap () ; let syn :: PathArguments :: AngleBracketed (bracketed) = & ty . arguments else { panic ! ("expected bracketed generic arguments") ; } ; assert_eq ! (bracketed . args . len () , 1) ; let syn :: GenericArgument :: Type (ty) = & bracketed . args [0] else { panic ! ("expected generic parameter to be a type generic") ; } ; ty } if type_matches_path (ty , & ["std" , "option" , "Option"]) { FieldInnerTy :: Option (single_generic_type (ty)) } else if type_matches_path (ty , & ["std" , "vec" , "Vec"]) { FieldInnerTy :: Vec (single_generic_type (ty)) } else { FieldInnerTy :: Plain (ty) } } #[doc = " Returns `true` if `FieldInnerTy::with` will result in iteration for this inner type (i.e."] #[doc = " that cloning might be required for values moved in the loop body)."] pub (crate) fn will_iterate (& self) -> bool { match self { FieldInnerTy :: Vec (..) => true , FieldInnerTy :: Option (..) | FieldInnerTy :: Plain (_) => false , } } #[doc = " Returns the inner type."] pub (crate) fn inner_type (& self) -> & 'ty Type { match self { FieldInnerTy :: Option (inner) | FieldInnerTy :: Vec (inner) | FieldInnerTy :: Plain (inner) => { inner } } } #[doc = " Surrounds `inner` with destructured wrapper type, exposing inner type as `binding`."] pub (crate) fn with (& self , binding : impl ToTokens , inner : impl ToTokens) -> TokenStream { match self { FieldInnerTy :: Option (..) => quote ! { if let Some (# binding) = # binding { # inner } } , FieldInnerTy :: Vec (..) => quote ! { for # binding in # binding { # inner } } , FieldInnerTy :: Plain (t) if type_is_bool (t) => quote ! { if # binding { # inner } } , FieldInnerTy :: Plain (..) => quote ! { # inner } , } } pub (crate) fn span (& self) -> proc_macro2 :: Span { match self { FieldInnerTy :: Option (ty) | FieldInnerTy :: Vec (ty) | FieldInnerTy :: Plain (ty) => ty . span () , } } }}}
mkitem!{mkstruct!{#[doc = " Field information passed to the builder. Deliberately omits attrs to discourage the"] #[doc = " `generate_*` methods from walking the attributes themselves."] pub (crate) struct FieldInfo < 'a > { pub (crate) binding : & 'a BindingInfo < 'a > , pub (crate) ty : FieldInnerTy < 'a > , pub (crate) span : & 'a proc_macro2 :: Span , }}}
mkitem!{mktrait!{#[doc = " Small helper trait for abstracting over `Option` fields that contain a value and a `Span`"] #[doc = " for error reporting if they are set more than once."] pub (crate) trait SetOnce < T > { fn set_once (& mut self , value : T , span : Span) ; fn value (self) -> Option < T > ; fn value_ref (& self) -> Option < & T > ; }}}
mkitem!{#[doc = " An [`Option<T>`] that keeps track of the span that caused it to be set; used with [`SetOnce`]."] pub (super) type SpannedOption < T > = Option < (T , Span) > ;}
mkitem!{mkimpl!{impl < T > SetOnce < T > for SpannedOption < T > { fn set_once (& mut self , value : T , span : Span) { match self { None => { * self = Some ((value , span)) ; } Some ((_ , prev_span)) => { span_err (span , "attribute specified multiple times") . span_note (* prev_span , "previously specified here") . emit () ; } } } fn value (self) -> Option < T > { self . map (| (v , _) | v) } fn value_ref (& self) -> Option < & T > { self . as_ref () . map (| (v , _) | v) } }}}
mkitem!{pub (super) type FieldMap = HashMap < String , TokenStream > ;}
mkitem!{mktrait!{pub (crate) trait HasFieldMap { #[doc = " Returns the binding for the field with the given name, if it exists on the type."] fn get_field_binding (& self , field : & String) -> Option < & TokenStream > ; #[doc = " In the strings in the attributes supplied to this macro, we want callers to be able to"] #[doc = " reference fields in the format string. For example:"] #[doc = ""] #[doc = " ```ignore (not-usage-example)"] #[doc = " /// Suggest `==` when users wrote `===`."] #[doc = " #[suggestion(slug = \"parser-not-javascript-eq\", code = \"{lhs} == {rhs}\")]"] #[doc = " struct NotJavaScriptEq {"] #[doc = "     #[primary_span]"] #[doc = "     span: Span,"] #[doc = "     lhs: Ident,"] #[doc = "     rhs: Ident,"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " We want to automatically pick up that `{lhs}` refers `self.lhs` and `{rhs}` refers to"] #[doc = " `self.rhs`, then generate this call to `format!`:"] #[doc = ""] #[doc = " ```ignore (not-usage-example)"] #[doc = " format!(\"{lhs} == {rhs}\", lhs = self.lhs, rhs = self.rhs)"] #[doc = " ```"] #[doc = ""] #[doc = " This function builds the entire call to `format!`."] fn build_format (& self , input : & str , span : proc_macro2 :: Span) -> TokenStream { let mut referenced_fields : BTreeSet < String > = BTreeSet :: new () ; let mut it = input . chars () . peekable () ; while let Some (c) = it . next () { if c != '{' { continue ; } if * it . peek () . unwrap_or (& '\0') == '{' { assert_eq ! (it . next () . unwrap () , '{') ; continue ; } let mut eat_argument = | | -> Option < String > { let mut result = String :: new () ; while let Some (c) = it . next () { result . push (c) ; let next = * it . peek () . unwrap_or (& '\0') ; if next == '}' { break ; } else if next == ':' { assert_eq ! (it . next () . unwrap () , ':') ; break ; } } while it . next () ? != '}' { continue ; } Some (result) } ; if let Some (referenced_field) = eat_argument () { referenced_fields . insert (referenced_field) ; } } let args = referenced_fields . into_iter () . map (| field : String | { let field_ident = format_ident ! ("{}" , field) ; let value = match self . get_field_binding (& field) { Some (value) => value . clone () , None => { span_err (span . unwrap () , format ! ("`{field}` doesn't refer to a field on this type") ,) . emit () ; quote ! { "{#field}" } } } ; quote ! { # field_ident = # value } }) ; quote ! { format ! (# input # (,# args) *) } } }}}
mkitem!{mkenum!{#[doc = " `Applicability` of a suggestion - mirrors `rustc_errors::Applicability` - and used to represent"] #[doc = " the user's selection of applicability if specified in an attribute."] #[derive (Clone , Copy)] pub (crate) enum Applicability { MachineApplicable , MaybeIncorrect , HasPlaceholders , Unspecified , }}}
mkitem!{mkimpl!{impl FromStr for Applicability { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "machine-applicable" => Ok (Applicability :: MachineApplicable) , "maybe-incorrect" => Ok (Applicability :: MaybeIncorrect) , "has-placeholders" => Ok (Applicability :: HasPlaceholders) , "unspecified" => Ok (Applicability :: Unspecified) , _ => Err (()) , } } }}}
mkitem!{mkimpl!{impl quote :: ToTokens for Applicability { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . extend (match self { Applicability :: MachineApplicable => { quote ! { rustc_errors :: Applicability :: MachineApplicable } } Applicability :: MaybeIncorrect => { quote ! { rustc_errors :: Applicability :: MaybeIncorrect } } Applicability :: HasPlaceholders => { quote ! { rustc_errors :: Applicability :: HasPlaceholders } } Applicability :: Unspecified => { quote ! { rustc_errors :: Applicability :: Unspecified } } }) ; } }}}

macro_rules! build_field_mapping_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_field_mapping in module {}", module_path!());
    };
}

mkfn!{
    build_field_mapping_introspect!();
    #[doc = " Build the mapping of field names to fields. This allows attributes to peek values from"] #[doc = " other fields."] pub (super) fn build_field_mapping (variant : & VariantInfo < '_ >) -> HashMap < String , TokenStream > { let mut fields_map = FieldMap :: new () ; for binding in variant . bindings () { if let Some (ident) = & binding . ast () . ident { fields_map . insert (ident . to_string () , quote ! { # binding }) ; } } fields_map }
}
mkitem!{mkenum!{#[derive (Copy , Clone , Debug)] pub (super) enum AllowMultipleAlternatives { No , Yes , }}}

macro_rules! parse_suggestion_values_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_suggestion_values in module {}", module_path!());
    };
}

mkfn!{
    parse_suggestion_values_introspect!();
    fn parse_suggestion_values (nested : ParseNestedMeta < '_ > , allow_multiple : AllowMultipleAlternatives ,) -> syn :: Result < Vec < LitStr > > { let values = if let Ok (val) = nested . value () { vec ! [val . parse () ?] } else { let content ; parenthesized ! (content in nested . input) ; if let AllowMultipleAlternatives :: No = allow_multiple { span_err (nested . input . span () . unwrap () , "expected exactly one string literal for `code = ...`" ,) . emit () ; vec ! [] } else { let literals = Punctuated :: < LitStr , Token ! [,] > :: parse_terminated (& content) ; match literals { Ok (p) if p . is_empty () => { span_err (content . span () . unwrap () , "expected at least one string literal for `code(...)`" ,) . emit () ; vec ! [] } Ok (p) => p . into_iter () . collect () , Err (_) => { span_err (content . span () . unwrap () , "`code(...)` must contain only string literals" ,) . emit () ; vec ! [] } } } } ; Ok (values) }
}

macro_rules! build_suggestion_code_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_suggestion_code in module {}", module_path!());
    };
}

mkfn!{
    build_suggestion_code_introspect!();
    #[doc = " Constructs the `format!()` invocation(s) necessary for a `#[suggestion*(code = \"foo\")]` or"] #[doc = " `#[suggestion*(code(\"foo\", \"bar\"))]` attribute field"] pub (super) fn build_suggestion_code (code_field : & Ident , nested : ParseNestedMeta < '_ > , fields : & impl HasFieldMap , allow_multiple : AllowMultipleAlternatives ,) -> TokenStream { let values = match parse_suggestion_values (nested , allow_multiple) { Ok (x) => x , Err (e) => return e . into_compile_error () , } ; if let AllowMultipleAlternatives :: Yes = allow_multiple { let formatted_strings : Vec < _ > = values . into_iter () . map (| value | fields . build_format (& value . value () , value . span ())) . collect () ; quote ! { let # code_field = [# (# formatted_strings) ,*] . into_iter () ; } } else if let [value] = values . as_slice () { let formatted_str = fields . build_format (& value . value () , value . span ()) ; quote ! { let # code_field = # formatted_str ; } } else { quote ! { let # code_field = String :: new () ; } } }
}
mkitem!{mkenum!{#[doc = " Possible styles for suggestion subdiagnostics."] #[derive (Clone , Copy , PartialEq)] pub (super) enum SuggestionKind { Normal , Short , Hidden , Verbose , ToolOnly , }}}
mkitem!{mkimpl!{impl FromStr for SuggestionKind { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "normal" => Ok (SuggestionKind :: Normal) , "short" => Ok (SuggestionKind :: Short) , "hidden" => Ok (SuggestionKind :: Hidden) , "verbose" => Ok (SuggestionKind :: Verbose) , "tool-only" => Ok (SuggestionKind :: ToolOnly) , _ => Err (()) , } } }}}
mkitem!{mkimpl!{impl fmt :: Display for SuggestionKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { SuggestionKind :: Normal => write ! (f , "normal") , SuggestionKind :: Short => write ! (f , "short") , SuggestionKind :: Hidden => write ! (f , "hidden") , SuggestionKind :: Verbose => write ! (f , "verbose") , SuggestionKind :: ToolOnly => write ! (f , "tool-only") , } } }}}
mkitem!{mkimpl!{impl SuggestionKind { pub (crate) fn to_suggestion_style (& self) -> TokenStream { match self { SuggestionKind :: Normal => { quote ! { rustc_errors :: SuggestionStyle :: ShowCode } } SuggestionKind :: Short => { quote ! { rustc_errors :: SuggestionStyle :: HideCodeInline } } SuggestionKind :: Hidden => { quote ! { rustc_errors :: SuggestionStyle :: HideCodeAlways } } SuggestionKind :: Verbose => { quote ! { rustc_errors :: SuggestionStyle :: ShowAlways } } SuggestionKind :: ToolOnly => { quote ! { rustc_errors :: SuggestionStyle :: CompletelyHidden } } } } fn from_suffix (s : & str) -> Option < Self > { match s { "" => Some (SuggestionKind :: Normal) , "_short" => Some (SuggestionKind :: Short) , "_hidden" => Some (SuggestionKind :: Hidden) , "_verbose" => Some (SuggestionKind :: Verbose) , _ => None , } } }}}
mkitem!{mkenum!{#[doc = " Types of subdiagnostics that can be created using attributes"] #[derive (Clone)] pub (super) enum SubdiagnosticKind { #[doc = " `#[label(...)]`"] Label , #[doc = " `#[note(...)]`"] Note , #[doc = " `#[note_once(...)]`"] NoteOnce , #[doc = " `#[help(...)]`"] Help , #[doc = " `#[help_once(...)]`"] HelpOnce , #[doc = " `#[warning(...)]`"] Warn , #[doc = " `#[suggestion{,_short,_hidden,_verbose}]`"] Suggestion { suggestion_kind : SuggestionKind , applicability : SpannedOption < Applicability > , #[doc = " Identifier for variable used for formatted code, e.g. `___code_0`. Enables separation"] #[doc = " of formatting and diagnostic emission so that `arg` calls can happen in-between.."] code_field : syn :: Ident , #[doc = " Initialization logic for `code_field`'s variable, e.g."] #[doc = " `let __formatted_code = /* whatever */;`"] code_init : TokenStream , } , #[doc = " `#[multipart_suggestion{,_short,_hidden,_verbose}]`"] MultipartSuggestion { suggestion_kind : SuggestionKind , applicability : SpannedOption < Applicability > , } , }}}
mkitem!{mkstruct!{pub (super) struct SubdiagnosticVariant { pub (super) kind : SubdiagnosticKind , pub (super) slug : Option < Path > , pub (super) no_span : bool , }}}
mkitem!{mkimpl!{impl SubdiagnosticVariant { #[doc = " Constructs a `SubdiagnosticVariant` from a field or type attribute such as `#[note]`,"] #[doc = " `#[error(parser::add_paren, no_span)]` or `#[suggestion(code = \"...\")]`. Returns the"] #[doc = " `SubdiagnosticKind` and the diagnostic slug, if specified."] pub (super) fn from_attr (attr : & Attribute , fields : & impl HasFieldMap ,) -> Result < Option < SubdiagnosticVariant > , DiagnosticDeriveError > { if is_doc_comment (attr) { return Ok (None) ; } let span = attr . span () . unwrap () ; let name = attr . path () . segments . last () . unwrap () . ident . to_string () ; let name = name . as_str () ; let mut kind = match name { "label" => SubdiagnosticKind :: Label , "note" => SubdiagnosticKind :: Note , "note_once" => SubdiagnosticKind :: NoteOnce , "help" => SubdiagnosticKind :: Help , "help_once" => SubdiagnosticKind :: HelpOnce , "warning" => SubdiagnosticKind :: Warn , _ => { if let Some (suggestion_kind) = name . strip_prefix ("suggestion") . and_then (SuggestionKind :: from_suffix) { if suggestion_kind != SuggestionKind :: Normal { invalid_attr (attr) . help (format ! (r#"Use `#[suggestion(..., style = "{suggestion_kind}")]` instead"#)) . emit () ; } SubdiagnosticKind :: Suggestion { suggestion_kind : SuggestionKind :: Normal , applicability : None , code_field : new_code_ident () , code_init : TokenStream :: new () , } } else if let Some (suggestion_kind) = name . strip_prefix ("multipart_suggestion") . and_then (SuggestionKind :: from_suffix) { if suggestion_kind != SuggestionKind :: Normal { invalid_attr (attr) . help (format ! (r#"Use `#[multipart_suggestion(..., style = "{suggestion_kind}")]` instead"#)) . emit () ; } SubdiagnosticKind :: MultipartSuggestion { suggestion_kind : SuggestionKind :: Normal , applicability : None , } } else { throw_invalid_attr ! (attr) ; } } } ; let list = match & attr . meta { Meta :: List (list) => { list } Meta :: Path (_) => { match kind { SubdiagnosticKind :: Label | SubdiagnosticKind :: Note | SubdiagnosticKind :: NoteOnce | SubdiagnosticKind :: Help | SubdiagnosticKind :: HelpOnce | SubdiagnosticKind :: Warn | SubdiagnosticKind :: MultipartSuggestion { .. } => { return Ok (Some (SubdiagnosticVariant { kind , slug : None , no_span : false })) ; } SubdiagnosticKind :: Suggestion { .. } => { throw_span_err ! (span , "suggestion without `code = \"...\"`") } } } _ => { throw_invalid_attr ! (attr) } } ; let mut code = None ; let mut suggestion_kind = None ; let mut first = true ; let mut slug = None ; let mut no_span = false ; list . parse_nested_meta (| nested | { if nested . input . is_empty () || nested . input . peek (Token ! [,]) { if first { slug = Some (nested . path) ; } else if nested . path . is_ident ("no_span") { no_span = true ; } else { span_err (nested . input . span () . unwrap () , "a diagnostic slug must be the first argument to the attribute") . emit () ; } first = false ; return Ok (()) ; } first = false ; let nested_name = nested . path . segments . last () . unwrap () . ident . to_string () ; let nested_name = nested_name . as_str () ; let path_span = nested . path . span () . unwrap () ; let val_span = nested . input . span () . unwrap () ; macro_rules ! get_string { () => { { let Ok (value) = nested . value () . and_then (| x | x . parse ::< LitStr > ()) else { span_err (val_span , "expected `= \"xxx\"`") . emit () ; return Ok (()) ; } ; value } } ; } let mut has_errors = false ; let input = nested . input ; match (nested_name , & mut kind) { ("code" , SubdiagnosticKind :: Suggestion { code_field , .. }) => { let code_init = build_suggestion_code (code_field , nested , fields , AllowMultipleAlternatives :: Yes ,) ; code . set_once (code_init , path_span) ; } ("applicability" , SubdiagnosticKind :: Suggestion { applicability , .. } | SubdiagnosticKind :: MultipartSuggestion { applicability , .. } ,) => { let value = get_string ! () ; let value = Applicability :: from_str (& value . value ()) . unwrap_or_else (| () | { span_err (value . span () . unwrap () , "invalid applicability") . emit () ; has_errors = true ; Applicability :: Unspecified }) ; applicability . set_once (value , span) ; } ("style" , SubdiagnosticKind :: Suggestion { .. } | SubdiagnosticKind :: MultipartSuggestion { .. } ,) => { let value = get_string ! () ; let value = value . value () . parse () . unwrap_or_else (| () | { span_err (value . span () . unwrap () , "invalid suggestion style") . help ("valid styles are `normal`, `short`, `hidden`, `verbose` and `tool-only`") . emit () ; has_errors = true ; SuggestionKind :: Normal }) ; suggestion_kind . set_once (value , span) ; } (_ , SubdiagnosticKind :: Suggestion { .. }) => { span_err (path_span , "invalid nested attribute") . help ("only `no_span`, `style`, `code` and `applicability` are valid nested attributes" ,) . emit () ; has_errors = true ; } (_ , SubdiagnosticKind :: MultipartSuggestion { .. }) => { span_err (path_span , "invalid nested attribute") . help ("only `no_span`, `style` and `applicability` are valid nested attributes") . emit () ; has_errors = true ; } _ => { span_err (path_span , "only `no_span` is a valid nested attribute") . emit () ; has_errors = true ; } } if has_errors { let _ = input . parse :: < TokenStream > () ; } Ok (()) }) ? ; match kind { SubdiagnosticKind :: Suggestion { ref code_field , ref mut code_init , suggestion_kind : ref mut kind_field , .. } => { if let Some (kind) = suggestion_kind . value () { * kind_field = kind ; } * code_init = if let Some (init) = code . value () { init } else { span_err (span , "suggestion without `code = \"...\"`") . emit () ; quote ! { let # code_field = std :: iter :: empty () ; } } ; } SubdiagnosticKind :: MultipartSuggestion { suggestion_kind : ref mut kind_field , .. } => { if let Some (kind) = suggestion_kind . value () { * kind_field = kind ; } } SubdiagnosticKind :: Label | SubdiagnosticKind :: Note | SubdiagnosticKind :: NoteOnce | SubdiagnosticKind :: Help | SubdiagnosticKind :: HelpOnce | SubdiagnosticKind :: Warn => { } } Ok (Some (SubdiagnosticVariant { kind , slug , no_span })) } }}}
mkitem!{mkimpl!{impl quote :: IdentFragment for SubdiagnosticKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { SubdiagnosticKind :: Label => write ! (f , "label") , SubdiagnosticKind :: Note => write ! (f , "note") , SubdiagnosticKind :: NoteOnce => write ! (f , "note_once") , SubdiagnosticKind :: Help => write ! (f , "help") , SubdiagnosticKind :: HelpOnce => write ! (f , "help_once") , SubdiagnosticKind :: Warn => write ! (f , "warn") , SubdiagnosticKind :: Suggestion { .. } => write ! (f , "suggestions_with_style") , SubdiagnosticKind :: MultipartSuggestion { .. } => { write ! (f , "multipart_suggestion_with_style") } } } fn span (& self) -> Option < proc_macro2 :: Span > { None } }}}

macro_rules! should_generate_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function should_generate_arg in module {}", module_path!());
    };
}

mkfn!{
    should_generate_arg_introspect!();
    #[doc = " Returns `true` if `field` should generate a `arg` call rather than any other diagnostic"] #[doc = " call (like `span_label`)."] pub (super) fn should_generate_arg (field : & Field) -> bool { field . attrs . iter () . all (| attr | is_doc_comment (attr)) }
}

macro_rules! is_doc_comment_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_doc_comment in module {}", module_path!());
    };
}

mkfn!{
    is_doc_comment_introspect!();
    pub (super) fn is_doc_comment (attr : & Attribute) -> bool { attr . path () . segments . last () . unwrap () . ident == "doc" }
}