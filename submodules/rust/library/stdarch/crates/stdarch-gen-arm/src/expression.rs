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
mkuse!{use itertools :: Itertools ;}
mkuse!{use proc_macro2 :: { Literal , Punct , Spacing , TokenStream } ;}
mkuse!{use quote :: { ToTokens , TokenStreamExt , format_ident , quote } ;}
mkuse!{use regex :: Regex ;}
mkuse!{use serde :: de :: { self , MapAccess , Visitor } ;}
mkuse!{use serde :: { Deserialize , Deserializer , Serialize } ;}
mkuse!{use std :: fmt ;}
mkuse!{use std :: str :: FromStr ;}
mkuse!{use std :: sync :: LazyLock ;}
mkuse!{use crate :: intrinsic :: Intrinsic ;}
mkuse!{use crate :: wildstring :: WildStringPart ;}
mkuse!{use crate :: { context :: { self , Context , VariableType } , intrinsic :: { Argument , LLVMLink , StaticDefinition } , matching :: { MatchKindValues , MatchSizeValues } , typekinds :: { BaseType , BaseTypeKind , TypeKind } , wildcards :: Wildcard , wildstring :: WildString , } ;}
mkitem!{mkenum!{#[derive (Debug , Clone , Copy , Serialize , Deserialize)] pub enum IdentifierType { Variable , Symbol , }}}
mkitem!{mkenum!{#[derive (Debug , Clone , Serialize , Deserialize)] #[serde (untagged)] pub enum LetVariant { Basic (WildString , Box < Expression >) , WithType (WildString , TypeKind , Box < Expression >) , MutWithType (WildString , TypeKind , Box < Expression >) , }}}
mkitem!{mkstruct!{#[derive (Debug , Clone , Serialize , Deserialize)] pub struct FnCall (#[doc = " Function pointer"] pub Box < Expression > , #[doc = " Function arguments"] pub Vec < Expression > , #[doc = " Function turbofish arguments"] #[serde (default)] pub Vec < Expression > , #[doc = " Function requires unsafe wrapper"] #[serde (default)] pub bool ,) ;}}
mkitem!{mkimpl!{impl FnCall { pub fn new_expression (fn_ptr : Expression , arguments : Vec < Expression >) -> Expression { FnCall (Box :: new (fn_ptr) , arguments , Vec :: new () , false) . into () } pub fn new_unsafe_expression (fn_ptr : Expression , arguments : Vec < Expression >) -> Expression { FnCall (Box :: new (fn_ptr) , arguments , Vec :: new () , true) . into () } pub fn is_llvm_link_call (& self , llvm_link_name : & str) -> bool { self . is_expected_call (llvm_link_name) } pub fn is_target_feature_call (& self) -> bool { self . is_expected_call ("target_feature") } pub fn is_expected_call (& self , fn_call_name : & str) -> bool { if let Expression :: Identifier (fn_name , IdentifierType :: Symbol) = self . 0 . as_ref () { fn_name . to_string () == fn_call_name } else { false } } pub fn pre_build (& mut self , ctx : & mut Context) -> context :: Result { self . 0 . pre_build (ctx) ? ; self . 1 . iter_mut () . chain (self . 2 . iter_mut ()) . try_for_each (| ex | ex . pre_build (ctx)) } pub fn build (& mut self , intrinsic : & Intrinsic , ctx : & mut Context) -> context :: Result { self . 0 . build (intrinsic , ctx) ? ; self . 1 . iter_mut () . chain (self . 2 . iter_mut ()) . try_for_each (| ex | ex . build (intrinsic , ctx)) } }}}
mkitem!{mkimpl!{impl ToTokens for FnCall { fn to_tokens (& self , tokens : & mut TokenStream) { let FnCall (fn_ptr , arguments , turbofish , _requires_unsafe_wrapper) = self ; fn_ptr . to_tokens (tokens) ; if ! turbofish . is_empty () { tokens . append_all (quote ! { ::<# (# turbofish) ,*> }) ; } tokens . append_all (quote ! { (# (# arguments) ,*) }) } }}}
mkitem!{mkenum!{#[derive (Debug , Clone , Serialize , Deserialize)] #[serde (remote = "Self" , deny_unknown_fields)] pub enum Expression { #[doc = " (Re)Defines a variable"] Let (LetVariant) , #[doc = " Performs a variable assignment operation"] Assign (String , Box < Expression >) , #[doc = " Performs a macro call"] MacroCall (String , String) , #[doc = " Performs a function call"] FnCall (FnCall) , #[doc = " Performs a method call. The following:"] #[doc = " `MethodCall: [\"$object\", \"to_string\", []]`"] #[doc = " is tokenized as:"] #[doc = " `object.to_string()`."] MethodCall (Box < Expression > , String , Vec < Expression >) , #[doc = " Symbol identifier name, prepend with a `$` to treat it as a scope variable"] #[doc = " which engages variable tracking and enables inference."] #[doc = " E.g. `my_function_name` for a generic symbol or `$my_variable` for"] #[doc = " a variable."] Identifier (WildString , IdentifierType) , #[doc = " Constant signed integer number expression"] IntConstant (i32) , #[doc = " Constant floating point number expression"] FloatConstant (f32) , #[doc = " Constant boolean expression, either `true` or `false`"] BoolConstant (bool) , #[doc = " Array expression"] Array (Vec < Expression >) , #[doc = " Makes an LLVM link."] #[doc = ""] #[doc = " It stores the link's function name in the wildcard `{llvm_link}`, for use in"] #[doc = " subsequent expressions."] LLVMLink (LLVMLink) , #[doc = " Casts the given expression to the specified (unchecked) type"] CastAs (Box < Expression > , String) , #[doc = " Returns the LLVM `undef` symbol"] SvUndef , #[doc = " Multiplication"] Multiply (Box < Expression > , Box < Expression >) , #[doc = " Xor"] Xor (Box < Expression > , Box < Expression >) , #[doc = " Converts the specified constant to the specified type's kind"] ConvertConst (TypeKind , i32) , #[doc = " Yields the given type in the Rust representation"] Type (TypeKind) , MatchSize (TypeKind , MatchSizeValues < Box < Expression > >) , MatchKind (TypeKind , MatchKindValues < Box < Expression > >) , }}}
mkitem!{mkimpl!{impl Expression { pub fn pre_build (& mut self , ctx : & mut Context) -> context :: Result { match self { Self :: FnCall (fn_call) => fn_call . pre_build (ctx) , Self :: MethodCall (cl_ptr_ex , _ , arg_exs) => { cl_ptr_ex . pre_build (ctx) ? ; arg_exs . iter_mut () . try_for_each (| ex | ex . pre_build (ctx)) } Self :: Let (LetVariant :: Basic (_ , ex) | LetVariant :: WithType (_ , _ , ex) | LetVariant :: MutWithType (_ , _ , ex) ,) => ex . pre_build (ctx) , Self :: CastAs (ex , _) => ex . pre_build (ctx) , Self :: Multiply (lhs , rhs) | Self :: Xor (lhs , rhs) => { lhs . pre_build (ctx) ? ; rhs . pre_build (ctx) } Self :: MatchSize (match_ty , values) => { * self = * values . get (match_ty , ctx . local) ? . to_owned () ; self . pre_build (ctx) } Self :: MatchKind (match_ty , values) => { * self = * values . get (match_ty , ctx . local) ? . to_owned () ; self . pre_build (ctx) } _ => Ok (()) , } } pub fn build (& mut self , intrinsic : & Intrinsic , ctx : & mut Context) -> context :: Result { match self { Self :: LLVMLink (link) => link . build_and_save (ctx) , Self :: Identifier (identifier , id_type) => { identifier . build_acle (ctx . local) ? ; if let IdentifierType :: Variable = id_type { ctx . local . variables . get (& identifier . to_string ()) . map (| _ | ()) . ok_or_else (| | format ! ("invalid variable {identifier} being referenced")) } else { Ok (()) } } Self :: FnCall (fn_call) => { fn_call . build (intrinsic , ctx) ? ; #[allow (clippy :: collapsible_if)] if let Some (llvm_link_name) = ctx . local . substitutions . get (& Wildcard :: LLVMLink) { if fn_call . is_llvm_link_call (llvm_link_name) { * self = intrinsic . llvm_link () . expect ("got LLVMLink wildcard without a LLVM link in `compose`") . apply_conversions_to_call (fn_call . clone () , ctx) ? } } Ok (()) } Self :: MethodCall (cl_ptr_ex , _ , arg_exs) => { cl_ptr_ex . build (intrinsic , ctx) ? ; arg_exs . iter_mut () . try_for_each (| ex | ex . build (intrinsic , ctx)) } Self :: Let (variant) => { let (var_name , ex , ty) = match variant { LetVariant :: Basic (var_name , ex) => (var_name , ex , None) , LetVariant :: WithType (var_name , ty , ex) | LetVariant :: MutWithType (var_name , ty , ex) => { if let Some (w) = ty . wildcard () { ty . populate_wildcard (ctx . local . provide_type_wildcard (w) ?) ? ; } (var_name , ex , Some (ty . to_owned ())) } } ; var_name . build_acle (ctx . local) ? ; ctx . local . variables . insert (var_name . to_string () , (ty . unwrap_or_else (| | TypeKind :: Custom ("unknown" . to_string ())) , VariableType :: Internal ,) ,) ; ex . build (intrinsic , ctx) } Self :: CastAs (ex , _) => ex . build (intrinsic , ctx) , Self :: Multiply (lhs , rhs) | Self :: Xor (lhs , rhs) => { lhs . build (intrinsic , ctx) ? ; rhs . build (intrinsic , ctx) } Self :: ConvertConst (ty , num) => { if let Some (w) = ty . wildcard () { * ty = ctx . local . provide_type_wildcard (w) ? } if let Some (BaseType :: Sized (BaseTypeKind :: Float , _)) = ty . base () { * self = Expression :: FloatConstant (* num as f32) } else { * self = Expression :: IntConstant (* num) } Ok (()) } Self :: Type (ty) => { if let Some (w) = ty . wildcard () { * ty = ctx . local . provide_type_wildcard (w) ? } Ok (()) } _ => Ok (()) , } } #[doc = " True if the expression requires an `unsafe` context in a safe function."] #[doc = ""] #[doc = " The classification is somewhat fuzzy, based on actual usage (e.g. empirical function names)"] #[doc = " rather than a full parse. This is a reasonable approach because mistakes here will usually"] #[doc = " be caught at build time:"] #[doc = ""] #[doc = "  - Missing an `unsafe` is a build error."] #[doc = "  - An unnecessary `unsafe` is a warning, made into an error by the CI's `-D warnings`."] #[doc = ""] #[doc = " This **panics** if it encounters an expression that shouldn't appear in a safe function at"] #[doc = " all (such as `SvUndef`)."] pub fn requires_unsafe_wrapper (& self , ctx_fn : & str) -> bool { match self { Self :: LLVMLink (..) => false , Self :: Identifier (..) => false , Self :: IntConstant (..) => false , Self :: FloatConstant (..) => false , Self :: BoolConstant (..) => false , Self :: Type (..) => false , Self :: ConvertConst (..) => false , Self :: Assign (_var , exp) => exp . requires_unsafe_wrapper (ctx_fn) , Self :: Let (LetVariant :: Basic (_ , exp) | LetVariant :: WithType (_ , _ , exp) | LetVariant :: MutWithType (_ , _ , exp) ,) => exp . requires_unsafe_wrapper (ctx_fn) , Self :: Array (exps) => exps . iter () . any (| exp | exp . requires_unsafe_wrapper (ctx_fn)) , Self :: Multiply (lhs , rhs) | Self :: Xor (lhs , rhs) => { lhs . requires_unsafe_wrapper (ctx_fn) || rhs . requires_unsafe_wrapper (ctx_fn) } Self :: CastAs (exp , _ty) => exp . requires_unsafe_wrapper (ctx_fn) , Self :: FnCall (FnCall (fn_exp , args , turbo_args , requires_unsafe_wrapper)) => { let fn_name = fn_exp . to_string () ; fn_exp . requires_unsafe_wrapper (ctx_fn) || fn_name . starts_with ("_sv") || fn_name . starts_with ("simd_") || fn_name . ends_with ("transmute") || args . iter () . any (| exp | exp . requires_unsafe_wrapper (ctx_fn)) || turbo_args . iter () . any (| exp | exp . requires_unsafe_wrapper (ctx_fn)) || * requires_unsafe_wrapper } Self :: MethodCall (exp , fn_name , args) => match fn_name . as_str () { "as_signed" => true , "as_unsigned" => true , _ => { exp . requires_unsafe_wrapper (ctx_fn) || args . iter () . any (| exp | exp . requires_unsafe_wrapper (ctx_fn)) } } , Self :: MacroCall (_name , _args) => false , Self :: SvUndef => panic ! ("Refusing to wrap unsafe SvUndef in safe function '{ctx_fn}'.") , Self :: MatchKind (..) => { unimplemented ! ("The unsafety of {self:?} cannot be determined in '{ctx_fn}'.") } Self :: MatchSize (..) => { unimplemented ! ("The unsafety of {self:?} cannot be determined in '{ctx_fn}'.") } } } #[doc = " Determine if an expression is a `static_assert<...>` function call."] pub fn is_static_assert (& self) -> bool { match self { Expression :: FnCall (fn_call) => match fn_call . 0 . as_ref () { Expression :: Identifier (wild_string , _) => { if let WildStringPart :: String (function_name) = & wild_string . 0 [0] { function_name . starts_with ("static_assert") } else { false } } _ => panic ! ("Badly defined function call: {fn_call:?}") , } , _ => false , } } #[doc = " Determine if an espression is a LLVM binding"] pub fn is_llvm_link (& self) -> bool { matches ! (self , Expression :: LLVMLink (_)) } }}}
mkitem!{mkimpl!{impl FromStr for Expression { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { static MACRO_RE : LazyLock < Regex > = LazyLock :: new (| | Regex :: new (r"^(?P<name>[\w\d_]+)!\((?P<ex>.*?)\);?$") . unwrap ()) ; if s == "SvUndef" { Ok (Expression :: SvUndef) } else if MACRO_RE . is_match (s) { let c = MACRO_RE . captures (s) . unwrap () ; let ex = c ["ex"] . to_string () ; let _ : TokenStream = ex . parse () . map_err (| e | format ! ("could not parse macro call expression: {e:#?}")) ? ; Ok (Expression :: MacroCall (c ["name"] . to_string () , ex)) } else { let (s , id_type) = if let Some (varname) = s . strip_prefix ('$') { (varname , IdentifierType :: Variable) } else { (s , IdentifierType :: Symbol) } ; let identifier = s . trim () . parse () ? ; Ok (Expression :: Identifier (identifier , id_type)) } } }}}
mkitem!{mkimpl!{impl From < FnCall > for Expression { fn from (fn_call : FnCall) -> Self { Expression :: FnCall (fn_call) } }}}
mkitem!{mkimpl!{impl From < WildString > for Expression { fn from (ws : WildString) -> Self { Expression :: Identifier (ws , IdentifierType :: Symbol) } }}}
mkitem!{mkimpl!{impl From < & Argument > for Expression { fn from (a : & Argument) -> Self { Expression :: Identifier (a . name . to_owned () , IdentifierType :: Variable) } }}}
mkitem!{mkimpl!{impl TryFrom < & StaticDefinition > for Expression { type Error = String ; fn try_from (sd : & StaticDefinition) -> Result < Self , Self :: Error > { match sd { StaticDefinition :: Constant (imm) => Ok (imm . into ()) , StaticDefinition :: Generic (t) => t . parse () , } } }}}
mkitem!{mkimpl!{impl fmt :: Display for Expression { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Identifier (identifier , kind) => { write ! (f , "{}{identifier}" , matches ! (kind , IdentifierType :: Variable) . then_some ("$") . unwrap_or_default ()) } Self :: MacroCall (name , expression) => { write ! (f , "{name}!({expression})") } _ => Err (fmt :: Error) , } } }}}
mkitem!{mkimpl!{impl ToTokens for Expression { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Self :: Let (LetVariant :: Basic (var_name , exp)) => { let var_ident = format_ident ! ("{}" , var_name . to_string ()) ; tokens . append_all (quote ! { let # var_ident = # exp }) } Self :: Let (LetVariant :: WithType (var_name , ty , exp)) => { let var_ident = format_ident ! ("{}" , var_name . to_string ()) ; tokens . append_all (quote ! { let # var_ident : # ty = # exp }) } Self :: Let (LetVariant :: MutWithType (var_name , ty , exp)) => { let var_ident = format_ident ! ("{}" , var_name . to_string ()) ; tokens . append_all (quote ! { let mut # var_ident : # ty = # exp }) } Self :: Assign (var_name , exp) => { let var_name_str : & str ; if let Some (ch) = var_name . chars () . nth (0) { if ch == '*' { tokens . append (Punct :: new ('*' , Spacing :: Alone)) ; var_name_str = & var_name [1 .. var_name . len ()] ; } else { var_name_str = var_name . as_str () ; } } else { panic ! ("Invalid variable name, must be at least one character") } let var_ident = format_ident ! ("{}" , var_name_str) ; tokens . append_all (quote ! { # var_ident = # exp }) } Self :: MacroCall (name , ex) => { let name = format_ident ! ("{name}") ; let ex : TokenStream = ex . parse () . unwrap () ; tokens . append_all (quote ! { # name ! (# ex) }) } Self :: FnCall (fn_call) => fn_call . to_tokens (tokens) , Self :: MethodCall (exp , fn_name , args) => { let fn_ident = format_ident ! ("{}" , fn_name) ; tokens . append_all (quote ! { # exp .# fn_ident (# (# args) ,*) }) } Self :: Identifier (identifier , _) => { assert ! (! identifier . has_wildcards () , "expression {self:#?} was not built before calling to_tokens") ; identifier . to_string () . parse :: < TokenStream > () . unwrap_or_else (| _ | panic ! ("invalid syntax: {self:?}")) . to_tokens (tokens) ; } Self :: IntConstant (n) => tokens . append (Literal :: i32_unsuffixed (* n)) , Self :: FloatConstant (n) => tokens . append (Literal :: f32_unsuffixed (* n)) , Self :: BoolConstant (true) => tokens . append (format_ident ! ("true")) , Self :: BoolConstant (false) => tokens . append (format_ident ! ("false")) , Self :: Array (vec) => tokens . append_all (quote ! { [# (# vec) ,*] }) , Self :: LLVMLink (link) => link . to_tokens (tokens) , Self :: CastAs (ex , ty) => { let ty : TokenStream = ty . parse () . expect ("invalid syntax") ; tokens . append_all (quote ! { # ex as # ty }) } Self :: SvUndef => tokens . append_all (quote ! { simd_reinterpret (()) }) , Self :: Multiply (lhs , rhs) => tokens . append_all (quote ! { # lhs * # rhs }) , Self :: Xor (lhs , rhs) => tokens . append_all (quote ! { # lhs ^ # rhs }) , Self :: Type (ty) => ty . to_tokens (tokens) , _ => unreachable ! ("{self:?} cannot be converted to tokens.") , } } }}}
mkitem!{mkimpl!{impl Serialize for Expression { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { match self { Self :: IntConstant (v) => serializer . serialize_i32 (* v) , Self :: FloatConstant (v) => serializer . serialize_f32 (* v) , Self :: BoolConstant (v) => serializer . serialize_bool (* v) , Self :: Identifier (..) => serializer . serialize_str (& self . to_string ()) , Self :: MacroCall (..) => serializer . serialize_str (& self . to_string ()) , _ => Expression :: serialize (self , serializer) , } } }}}
mkitem!{mkimpl!{impl < 'de > Deserialize < 'de > for Expression { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct CustomExpressionVisitor ; impl < 'de > Visitor < 'de > for CustomExpressionVisitor { type Value = Expression ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("integer, float, boolean, string or map") } fn visit_bool < E > (self , v : bool) -> Result < Self :: Value , E > where E : de :: Error , { Ok (Expression :: BoolConstant (v)) } fn visit_i64 < E > (self , v : i64) -> Result < Self :: Value , E > where E : de :: Error , { Ok (Expression :: IntConstant (v as i32)) } fn visit_u64 < E > (self , v : u64) -> Result < Self :: Value , E > where E : de :: Error , { Ok (Expression :: IntConstant (v as i32)) } fn visit_f64 < E > (self , v : f64) -> Result < Self :: Value , E > where E : de :: Error , { Ok (Expression :: FloatConstant (v as f32)) } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { FromStr :: from_str (value) . map_err (de :: Error :: custom) } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : de :: SeqAccess < 'de > , { let arr = std :: iter :: from_fn (| | seq . next_element :: < Self :: Value > () . transpose ()) . try_collect () ? ; Ok (Expression :: Array (arr)) } fn visit_map < M > (self , map : M) -> Result < Expression , M :: Error > where M : MapAccess < 'de > , { Expression :: deserialize (de :: value :: MapAccessDeserializer :: new (map)) } } deserializer . deserialize_any (CustomExpressionVisitor) } }}}