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
mkuse!{use std :: array :: IntoIter ;}
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_ast :: NodeId ;}
mkuse!{use rustc_data_structures :: stable_hasher :: ToStableHashKey ;}
mkuse!{use rustc_data_structures :: unord :: UnordMap ;}
mkuse!{use rustc_error_messages :: { DiagArgValue , IntoDiagArg } ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable_Generic } ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use rustc_span :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_span :: hygiene :: MacroKind ;}
mkuse!{use crate :: definitions :: DefPathData ;}
mkuse!{use crate :: hir ;}
mkitem!{mkenum!{# [doc = " Encodes if a `DefKind::Ctor` is the constructor of an enum variant or a struct."] # [derive (Clone , Copy , PartialEq , Eq , Encodable , Decodable , Hash , Debug , HashStable_Generic)] pub enum CtorOf { # [doc = " This `DefKind::Ctor` is a synthesized constructor of a tuple or unit struct."] Struct , # [doc = " This `DefKind::Ctor` is a synthesized constructor of a tuple or unit variant."] Variant , }}}
mkitem!{mkenum!{# [doc = " What kind of constructor something is."] # [derive (Clone , Copy , PartialEq , Eq , Encodable , Decodable , Hash , Debug , HashStable_Generic)] pub enum CtorKind { # [doc = " Constructor function automatically created by a tuple struct/variant."] Fn , # [doc = " Constructor constant automatically created by a unit struct/variant."] Const , }}}
mkitem!{mkstruct!{# [doc = " A set of macro kinds, for macros that can have more than one kind"] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Encodable , Decodable , Hash , Debug)] # [derive (HashStable_Generic)] pub struct MacroKinds (u8) ;}}
mkitem!{bitflags :: bitflags ! { impl MacroKinds : u8 { const BANG = 1 << 0 ; const ATTR = 1 << 1 ; const DERIVE = 1 << 2 ; } }}
mkitem!{mkimpl!{impl From < MacroKind > for MacroKinds { fn from (kind : MacroKind) -> Self { match kind { MacroKind :: Bang => Self :: BANG , MacroKind :: Attr => Self :: ATTR , MacroKind :: Derive => Self :: DERIVE , } } }}}
mkitem!{mkimpl!{impl MacroKinds { # [doc = " Convert the MacroKinds to a static string."] # [doc = ""] # [doc = " This hardcodes all the possibilities, in order to return a static string."] pub fn descr (self) -> & 'static str { match self { Self :: BANG => "macro" , Self :: ATTR => "attribute macro" , Self :: DERIVE => "derive macro" , _ if self == (Self :: ATTR | Self :: BANG) => "attribute/function macro" , _ if self == (Self :: DERIVE | Self :: BANG) => "derive/function macro" , _ if self == (Self :: ATTR | Self :: DERIVE) => "attribute/derive macro" , _ if self . is_all () => "attribute/derive/function macro" , _ if self . is_empty () => "useless macro" , _ => unreachable ! () , } } # [doc = " Return an indefinite article (a/an) for use with `descr()`"] pub fn article (self) -> & 'static str { if self . contains (Self :: ATTR) { "an" } else { "a" } } }}}
mkitem!{mkenum!{# [doc = " An attribute that is not a macro; e.g., `#[inline]` or `#[rustfmt::skip]`."] # [derive (Clone , Copy , PartialEq , Eq , Encodable , Decodable , Hash , Debug , HashStable_Generic)] pub enum NonMacroAttrKind { # [doc = " Single-segment attribute defined by the language (`#[inline]`)"] Builtin (Symbol) , # [doc = " Multi-segment custom attribute living in a \"tool module\" (`#[rustfmt::skip]`)."] Tool , # [doc = " Single-segment custom attribute registered by a derive macro (`#[serde(default)]`)."] DeriveHelper , # [doc = " Single-segment custom attribute registered by a derive macro"] # [doc = " but used before that derive macro was expanded (deprecated)."] DeriveHelperCompat , }}}
mkitem!{mkenum!{# [doc = " What kind of definition something is; e.g., `mod` vs `struct`."] # [doc = " `enum DefPathData` may need to be updated if a new variant is added here."] # [derive (Clone , Copy , PartialEq , Eq , Encodable , Decodable , Hash , Debug , HashStable_Generic)] pub enum DefKind { Mod , # [doc = " Refers to the struct itself, [`DefKind::Ctor`] refers to its constructor if it exists."] Struct , Union , Enum , # [doc = " Refers to the variant itself, [`DefKind::Ctor`] refers to its constructor if it exists."] Variant , Trait , # [doc = " Type alias: `type Foo = Bar;`"] TyAlias , # [doc = " Type from an `extern` block."] ForeignTy , # [doc = " Trait alias: `trait IntIterator = Iterator<Item = i32>;`"] TraitAlias , # [doc = " Associated type: `trait MyTrait { type Assoc; }`"] AssocTy , # [doc = " Type parameter: the `T` in `struct Vec<T> { ... }`"] TyParam , Fn , Const , # [doc = " Constant generic parameter: `struct Foo<const N: usize> { ... }`"] ConstParam , Static { # [doc = " Whether it's a `unsafe static`, `safe static` (inside extern only) or just a `static`."] safety : hir :: Safety , # [doc = " Whether it's a `static mut` or just a `static`."] mutability : ast :: Mutability , # [doc = " Whether it's an anonymous static generated for nested allocations."] nested : bool , } , # [doc = " Refers to the struct or enum variant's constructor."] # [doc = ""] # [doc = " The reason `Ctor` exists in addition to [`DefKind::Struct`] and"] # [doc = " [`DefKind::Variant`] is because structs and enum variants exist"] # [doc = " in the *type* namespace, whereas struct and enum variant *constructors*"] # [doc = " exist in the *value* namespace."] # [doc = ""] # [doc = " You may wonder why enum variants exist in the type namespace as opposed"] # [doc = " to the value namespace. Check out [RFC 2593] for intuition on why that is."] # [doc = ""] # [doc = " [RFC 2593]: https://github.com/rust-lang/rfcs/pull/2593"] Ctor (CtorOf , CtorKind) , # [doc = " Associated function: `impl MyStruct { fn associated() {} }`"] # [doc = " or `trait Foo { fn associated() {} }`"] AssocFn , # [doc = " Associated constant: `trait MyTrait { const ASSOC: usize; }`"] AssocConst , Macro (MacroKinds) , ExternCrate , Use , # [doc = " An `extern` block."] ForeignMod , # [doc = " Anonymous constant, e.g. the `1 + 2` in `[u8; 1 + 2]`."] # [doc = ""] # [doc = " Not all anon-consts are actually still relevant in the HIR. We lower"] # [doc = " trivial const-arguments directly to `hir::ConstArgKind::Path`, at which"] # [doc = " point the definition for the anon-const ends up unused and incomplete."] # [doc = ""] # [doc = " We do not provide any a `Span` for the definition and pretty much all other"] # [doc = " queries also ICE when using this `DefId`. Given that the `DefId` of such"] # [doc = " constants should only be reachable by iterating all definitions of a"] # [doc = " given crate, you should not have to worry about this."] AnonConst , # [doc = " An inline constant, e.g. `const { 1 + 2 }`"] InlineConst , # [doc = " Opaque type, aka `impl Trait`."] OpaqueTy , # [doc = " A field in a struct, enum or union. e.g."] # [doc = " - `bar` in `struct Foo { bar: u8 }`"] # [doc = " - `Foo::Bar::0` in `enum Foo { Bar(u8) }`"] Field , # [doc = " Lifetime parameter: the `'a` in `struct Foo<'a> { ... }`"] LifetimeParam , # [doc = " A use of `global_asm!`."] GlobalAsm , Impl { of_trait : bool , } , # [doc = " A closure, coroutine, or coroutine-closure."] # [doc = ""] # [doc = " These are all represented with the same `ExprKind::Closure` in the AST and HIR,"] # [doc = " which makes it difficult to distinguish these during def collection. Therefore,"] # [doc = " we treat them all the same, and code which needs to distinguish them can match"] # [doc = " or `hir::ClosureKind` or `type_of`."] Closure , # [doc = " The definition of a synthetic coroutine body created by the lowering of a"] # [doc = " coroutine-closure, such as an async closure."] SyntheticCoroutineBody , }}}
mkitem!{mkimpl!{impl DefKind { # [doc = " Get an English description for the item's kind."] # [doc = ""] # [doc = " If you have access to `TyCtxt`, use `TyCtxt::def_descr` or"] # [doc = " `TyCtxt::def_kind_descr` instead, because they give better"] # [doc = " information for coroutines and associated functions."] pub fn descr (self , def_id : DefId) -> & 'static str { match self { DefKind :: Fn => "function" , DefKind :: Mod if def_id . is_crate_root () && ! def_id . is_local () => "crate" , DefKind :: Mod => "module" , DefKind :: Static { .. } => "static" , DefKind :: Enum => "enum" , DefKind :: Variant => "variant" , DefKind :: Ctor (CtorOf :: Variant , CtorKind :: Fn) => "tuple variant" , DefKind :: Ctor (CtorOf :: Variant , CtorKind :: Const) => "unit variant" , DefKind :: Struct => "struct" , DefKind :: Ctor (CtorOf :: Struct , CtorKind :: Fn) => "tuple struct" , DefKind :: Ctor (CtorOf :: Struct , CtorKind :: Const) => "unit struct" , DefKind :: OpaqueTy => "opaque type" , DefKind :: TyAlias => "type alias" , DefKind :: TraitAlias => "trait alias" , DefKind :: AssocTy => "associated type" , DefKind :: Union => "union" , DefKind :: Trait => "trait" , DefKind :: ForeignTy => "foreign type" , DefKind :: AssocFn => "associated function" , DefKind :: Const => "constant" , DefKind :: AssocConst => "associated constant" , DefKind :: TyParam => "type parameter" , DefKind :: ConstParam => "const parameter" , DefKind :: Macro (kinds) => kinds . descr () , DefKind :: LifetimeParam => "lifetime parameter" , DefKind :: Use => "import" , DefKind :: ForeignMod => "foreign module" , DefKind :: AnonConst => "constant expression" , DefKind :: InlineConst => "inline constant" , DefKind :: Field => "field" , DefKind :: Impl { .. } => "implementation" , DefKind :: Closure => "closure" , DefKind :: ExternCrate => "extern crate" , DefKind :: GlobalAsm => "global assembly block" , DefKind :: SyntheticCoroutineBody => "synthetic mir body" , } } # [doc = " Gets an English article for the definition."] # [doc = ""] # [doc = " If you have access to `TyCtxt`, use `TyCtxt::def_descr_article` or"] # [doc = " `TyCtxt::def_kind_descr_article` instead, because they give better"] # [doc = " information for coroutines and associated functions."] pub fn article (& self) -> & 'static str { match * self { DefKind :: AssocTy | DefKind :: AssocConst | DefKind :: AssocFn | DefKind :: Enum | DefKind :: OpaqueTy | DefKind :: Impl { .. } | DefKind :: Use | DefKind :: InlineConst | DefKind :: ExternCrate => "an" , DefKind :: Macro (kinds) => kinds . article () , _ => "a" , } } pub fn ns (& self) -> Option < Namespace > { match self { DefKind :: Mod | DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Variant | DefKind :: Trait | DefKind :: TyAlias | DefKind :: ForeignTy | DefKind :: TraitAlias | DefKind :: AssocTy | DefKind :: TyParam => Some (Namespace :: TypeNS) , DefKind :: Fn | DefKind :: Const | DefKind :: ConstParam | DefKind :: Static { .. } | DefKind :: Ctor (..) | DefKind :: AssocFn | DefKind :: AssocConst => Some (Namespace :: ValueNS) , DefKind :: Macro (..) => Some (Namespace :: MacroNS) , DefKind :: AnonConst | DefKind :: InlineConst | DefKind :: Field | DefKind :: LifetimeParam | DefKind :: ExternCrate | DefKind :: Closure | DefKind :: Use | DefKind :: ForeignMod | DefKind :: GlobalAsm | DefKind :: Impl { .. } | DefKind :: OpaqueTy | DefKind :: SyntheticCoroutineBody => None , } } pub fn def_path_data (self , name : Option < Symbol >) -> DefPathData { match self { DefKind :: Mod | DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Variant | DefKind :: Trait | DefKind :: TyAlias | DefKind :: ForeignTy | DefKind :: TraitAlias | DefKind :: TyParam | DefKind :: ExternCrate => DefPathData :: TypeNs (name . unwrap ()) , DefKind :: AssocTy => DefPathData :: TypeNs (name . unwrap ()) , DefKind :: Fn | DefKind :: Const | DefKind :: ConstParam | DefKind :: Static { .. } | DefKind :: AssocFn | DefKind :: AssocConst | DefKind :: Field => DefPathData :: ValueNs (name . unwrap ()) , DefKind :: Macro (..) => DefPathData :: MacroNs (name . unwrap ()) , DefKind :: LifetimeParam => DefPathData :: LifetimeNs (name . unwrap ()) , DefKind :: Ctor (..) => DefPathData :: Ctor , DefKind :: Use => DefPathData :: Use , DefKind :: ForeignMod => DefPathData :: ForeignMod , DefKind :: AnonConst => DefPathData :: AnonConst , DefKind :: InlineConst => DefPathData :: AnonConst , DefKind :: OpaqueTy => DefPathData :: OpaqueTy , DefKind :: GlobalAsm => DefPathData :: GlobalAsm , DefKind :: Impl { .. } => DefPathData :: Impl , DefKind :: Closure => DefPathData :: Closure , DefKind :: SyntheticCoroutineBody => DefPathData :: SyntheticCoroutineBody , } } pub fn is_assoc (self) -> bool { matches ! (self , DefKind :: AssocConst | DefKind :: AssocFn | DefKind :: AssocTy) } # [doc = " This is a \"module\" in name resolution sense."] # [inline] pub fn is_module_like (self) -> bool { matches ! (self , DefKind :: Mod | DefKind :: Enum | DefKind :: Trait) } # [inline] pub fn is_adt (self) -> bool { matches ! (self , DefKind :: Struct | DefKind :: Union | DefKind :: Enum) } # [inline] pub fn is_fn_like (self) -> bool { matches ! (self , DefKind :: Fn | DefKind :: AssocFn | DefKind :: Closure | DefKind :: SyntheticCoroutineBody) } # [doc = " Whether the corresponding item has generic parameters, ie. the `generics_of` query works."] pub fn has_generics (self) -> bool { match self { DefKind :: AnonConst | DefKind :: AssocConst | DefKind :: AssocFn | DefKind :: AssocTy | DefKind :: Closure | DefKind :: Const | DefKind :: Ctor (..) | DefKind :: Enum | DefKind :: Field | DefKind :: Fn | DefKind :: ForeignTy | DefKind :: Impl { .. } | DefKind :: InlineConst | DefKind :: OpaqueTy | DefKind :: Static { .. } | DefKind :: Struct | DefKind :: SyntheticCoroutineBody | DefKind :: Trait | DefKind :: TraitAlias | DefKind :: TyAlias | DefKind :: Union | DefKind :: Variant => true , DefKind :: ConstParam | DefKind :: ExternCrate | DefKind :: ForeignMod | DefKind :: GlobalAsm | DefKind :: LifetimeParam | DefKind :: Macro (_) | DefKind :: Mod | DefKind :: TyParam | DefKind :: Use => false , } } # [doc = " Whether `query get_codegen_attrs` should be used with this definition."] pub fn has_codegen_attrs (self) -> bool { match self { DefKind :: Fn | DefKind :: AssocFn | DefKind :: Ctor (..) | DefKind :: Closure | DefKind :: Static { .. } | DefKind :: SyntheticCoroutineBody => true , DefKind :: Mod | DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Variant | DefKind :: Trait | DefKind :: TyAlias | DefKind :: ForeignTy | DefKind :: TraitAlias | DefKind :: AssocTy | DefKind :: Const | DefKind :: AssocConst | DefKind :: Macro (..) | DefKind :: Use | DefKind :: ForeignMod | DefKind :: OpaqueTy | DefKind :: Impl { .. } | DefKind :: Field | DefKind :: TyParam | DefKind :: ConstParam | DefKind :: LifetimeParam | DefKind :: AnonConst | DefKind :: InlineConst | DefKind :: GlobalAsm | DefKind :: ExternCrate => false , } } # [doc = " Returns `true` if `self` is a kind of definition that does not have its own"] # [doc = " type-checking context, i.e. closure, coroutine or inline const."] # [inline] pub fn is_typeck_child (self) -> bool { match self { DefKind :: Closure | DefKind :: InlineConst | DefKind :: SyntheticCoroutineBody => true , DefKind :: Mod | DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Variant | DefKind :: Trait | DefKind :: TyAlias | DefKind :: ForeignTy | DefKind :: TraitAlias | DefKind :: AssocTy | DefKind :: TyParam | DefKind :: Fn | DefKind :: Const | DefKind :: ConstParam | DefKind :: Static { .. } | DefKind :: Ctor (_ , _) | DefKind :: AssocFn | DefKind :: AssocConst | DefKind :: Macro (_) | DefKind :: ExternCrate | DefKind :: Use | DefKind :: ForeignMod | DefKind :: AnonConst | DefKind :: OpaqueTy | DefKind :: Field | DefKind :: LifetimeParam | DefKind :: GlobalAsm | DefKind :: Impl { .. } => false , } } }}}
mkitem!{mkenum!{# [doc = " The resolution of a path or export."] # [doc = ""] # [doc = " For every path or identifier in Rust, the compiler must determine"] # [doc = " what the path refers to. This process is called name resolution,"] # [doc = " and `Res` is the primary result of name resolution."] # [doc = ""] # [doc = " For example, everything prefixed with `/* Res */` in this example has"] # [doc = " an associated `Res`:"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " fn str_to_string(s: & /* Res */ str) -> /* Res */ String {"] # [doc = "     /* Res */ String::from(/* Res */ s)"] # [doc = " }"] # [doc = ""] # [doc = " /* Res */ str_to_string(\"hello\");"] # [doc = " ```"] # [doc = ""] # [doc = " The associated `Res`s will be:"] # [doc = ""] # [doc = " - `str` will resolve to [`Res::PrimTy`];"] # [doc = " - `String` will resolve to [`Res::Def`], and the `Res` will include the [`DefId`]"] # [doc = "   for `String` as defined in the standard library;"] # [doc = " - `String::from` will also resolve to [`Res::Def`], with the [`DefId`]"] # [doc = "   pointing to `String::from`;"] # [doc = " - `s` will resolve to [`Res::Local`];"] # [doc = " - the call to `str_to_string` will resolve to [`Res::Def`], with the [`DefId`]"] # [doc = "   pointing to the definition of `str_to_string` in the current crate."] # [derive (Clone , Copy , PartialEq , Eq , Encodable , Decodable , Hash , Debug , HashStable_Generic)] pub enum Res < Id = hir :: HirId > { # [doc = " Definition having a unique ID (`DefId`), corresponds to something defined in user code."] # [doc = ""] # [doc = " **Not bound to a specific namespace.**"] Def (DefKind , DefId) , # [doc = " A primitive type such as `i32` or `str`."] # [doc = ""] # [doc = " **Belongs to the type namespace.**"] PrimTy (hir :: PrimTy) , # [doc = " The `Self` type, as used within a trait."] # [doc = ""] # [doc = " **Belongs to the type namespace.**"] # [doc = ""] # [doc = " See the examples on [`Res::SelfTyAlias`] for details."] SelfTyParam { # [doc = " The trait this `Self` is a generic parameter for."] trait_ : DefId , } , # [doc = " The `Self` type, as used somewhere other than within a trait."] # [doc = ""] # [doc = " **Belongs to the type namespace.**"] # [doc = ""] # [doc = " Examples:"] # [doc = " ```"] # [doc = " struct Bar(Box<Self>); // SelfTyAlias"] # [doc = ""] # [doc = " trait Foo {"] # [doc = "     fn foo() -> Box<Self>; // SelfTyParam"] # [doc = " }"] # [doc = ""] # [doc = " impl Bar {"] # [doc = "     fn blah() {"] # [doc = "         let _: Self; // SelfTyAlias"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " impl Foo for Bar {"] # [doc = "     fn foo() -> Box<Self /* SelfTyAlias */> {"] # [doc = "         let _: Self;        // SelfTyAlias"] # [doc = ""] # [doc = "         todo!()"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " *See also [`Res::SelfCtor`].*"] # [doc = ""] SelfTyAlias { # [doc = " The item introducing the `Self` type alias. Can be used in the `type_of` query"] # [doc = " to get the underlying type."] alias_to : DefId , # [doc = " Whether the `Self` type is disallowed from mentioning generics (i.e. when used in an"] # [doc = " anonymous constant)."] # [doc = ""] # [doc = " HACK(min_const_generics): self types also have an optional requirement to **not**"] # [doc = " mention any generic parameters to allow the following with `min_const_generics`:"] # [doc = " ```"] # [doc = " # struct Foo;"] # [doc = " impl Foo { fn test() -> [u8; size_of::<Self>()] { todo!() } }"] # [doc = ""] # [doc = " struct Bar([u8; baz::<Self>()]);"] # [doc = " const fn baz<T>() -> usize { 10 }"] # [doc = " ```"] # [doc = " We do however allow `Self` in repeat expression even if it is generic to not break code"] # [doc = " which already works on stable while causing the `const_evaluatable_unchecked` future"] # [doc = " compat lint:"] # [doc = " ```"] # [doc = " fn foo<T>() {"] # [doc = "     let _bar = [1_u8; size_of::<*mut T>()];"] # [doc = " }"] # [doc = " ```"] forbid_generic : bool , # [doc = " Is this within an `impl Foo for bar`?"] is_trait_impl : bool , } , # [doc = " The `Self` constructor, along with the [`DefId`]"] # [doc = " of the impl it is associated with."] # [doc = ""] # [doc = " **Belongs to the value namespace.**"] # [doc = ""] # [doc = " *See also [`Res::SelfTyParam`] and [`Res::SelfTyAlias`].*"] SelfCtor (DefId) , # [doc = " A local variable or function parameter."] # [doc = ""] # [doc = " **Belongs to the value namespace.**"] Local (Id) , # [doc = " A tool attribute module; e.g., the `rustfmt` in `#[rustfmt::skip]`."] # [doc = ""] # [doc = " **Belongs to the type namespace.**"] ToolMod , # [doc = " An attribute that is *not* implemented via macro."] # [doc = " E.g., `#[inline]` and `#[rustfmt::skip]`, which are essentially directives,"] # [doc = " as opposed to `#[test]`, which is a builtin macro."] # [doc = ""] # [doc = " **Belongs to the macro namespace.**"] NonMacroAttr (NonMacroAttrKind) , # [doc = " Name resolution failed. We use a dummy `Res` variant so later phases"] # [doc = " of the compiler won't crash and can instead report more errors."] # [doc = ""] # [doc = " **Not bound to a specific namespace.**"] Err , }}}
mkitem!{mkimpl!{impl < Id > IntoDiagArg for Res < Id > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . descr ())) } }}}
mkitem!{mkstruct!{# [doc = " The result of resolving a path before lowering to HIR,"] # [doc = " with \"module\" segments resolved and associated item"] # [doc = " segments deferred to type checking."] # [doc = " `base_res` is the resolution of the resolved part of the"] # [doc = " path, `unresolved_segments` is the number of unresolved"] # [doc = " segments."] # [doc = ""] # [doc = " ```text"] # [doc = " module::Type::AssocX::AssocY::MethodOrAssocType"] # [doc = " ^~~~~~~~~~~~  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"] # [doc = " base_res      unresolved_segments = 3"] # [doc = ""] # [doc = " <T as Trait>::AssocX::AssocY::MethodOrAssocType"] # [doc = "       ^~~~~~~~~~~~~~  ^~~~~~~~~~~~~~~~~~~~~~~~~"] # [doc = "       base_res        unresolved_segments = 2"] # [doc = " ```"] # [derive (Copy , Clone , Debug)] pub struct PartialRes { base_res : Res < NodeId > , unresolved_segments : usize , }}}
mkitem!{mkimpl!{impl PartialRes { # [inline] pub fn new (base_res : Res < NodeId >) -> Self { PartialRes { base_res , unresolved_segments : 0 } } # [inline] pub fn with_unresolved_segments (base_res : Res < NodeId > , mut unresolved_segments : usize) -> Self { if base_res == Res :: Err { unresolved_segments = 0 } PartialRes { base_res , unresolved_segments } } # [inline] pub fn base_res (& self) -> Res < NodeId > { self . base_res } # [inline] pub fn unresolved_segments (& self) -> usize { self . unresolved_segments } # [inline] pub fn full_res (& self) -> Option < Res < NodeId > > { (self . unresolved_segments == 0) . then_some (self . base_res) } # [inline] pub fn expect_full_res (& self) -> Res < NodeId > { self . full_res () . expect ("unexpected unresolved segments") } }}}
mkitem!{mkenum!{# [doc = " Different kinds of symbols can coexist even if they share the same textual name."] # [doc = " Therefore, they each have a separate universe (known as a \"namespace\")."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Encodable , Decodable)] # [derive (HashStable_Generic)] pub enum Namespace { # [doc = " The type namespace includes `struct`s, `enum`s, `union`s, `trait`s, and `mod`s"] # [doc = " (and, by extension, crates)."] # [doc = ""] # [doc = " Note that the type namespace includes other items; this is not an"] # [doc = " exhaustive list."] TypeNS , # [doc = " The value namespace includes `fn`s, `const`s, `static`s, and local variables (including function arguments)."] ValueNS , # [doc = " The macro namespace includes `macro_rules!` macros, declarative `macro`s,"] # [doc = " procedural macros, attribute macros, `derive` macros, and non-macro attributes"] # [doc = " like `#[inline]` and `#[rustfmt::skip]`."] MacroNS , }}}
mkitem!{mkimpl!{impl Namespace { # [doc = " The English description of the namespace."] pub fn descr (self) -> & 'static str { match self { Self :: TypeNS => "type" , Self :: ValueNS => "value" , Self :: MacroNS => "macro" , } } }}}
mkitem!{mkimpl!{impl IntoDiagArg for Namespace { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . descr ())) } }}}
mkitem!{mkimpl!{impl < CTX : crate :: HashStableContext > ToStableHashKey < CTX > for Namespace { type KeyType = Namespace ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> Namespace { * self } }}}
mkitem!{mkstruct!{# [doc = " Just a helper ‒ separate structure for each namespace."] # [derive (Copy , Clone , Default , Debug , HashStable_Generic)] pub struct PerNS < T > { pub value_ns : T , pub type_ns : T , pub macro_ns : T , }}}
mkitem!{mkimpl!{impl < T > PerNS < T > { pub fn map < U , F : FnMut (T) -> U > (self , mut f : F) -> PerNS < U > { PerNS { value_ns : f (self . value_ns) , type_ns : f (self . type_ns) , macro_ns : f (self . macro_ns) } } # [doc = " Note: Do you really want to use this? Often you know which namespace a"] # [doc = " name will belong in, and you can consider just that namespace directly,"] # [doc = " rather than iterating through all of them."] pub fn into_iter (self) -> IntoIter < T , 3 > { [self . value_ns , self . type_ns , self . macro_ns] . into_iter () } # [doc = " Note: Do you really want to use this? Often you know which namespace a"] # [doc = " name will belong in, and you can consider just that namespace directly,"] # [doc = " rather than iterating through all of them."] pub fn iter (& self) -> IntoIter < & T , 3 > { [& self . value_ns , & self . type_ns , & self . macro_ns] . into_iter () } }}}
mkitem!{mkimpl!{impl < T > :: std :: ops :: Index < Namespace > for PerNS < T > { type Output = T ; fn index (& self , ns : Namespace) -> & T { match ns { Namespace :: ValueNS => & self . value_ns , Namespace :: TypeNS => & self . type_ns , Namespace :: MacroNS => & self . macro_ns , } } }}}
mkitem!{mkimpl!{impl < T > :: std :: ops :: IndexMut < Namespace > for PerNS < T > { fn index_mut (& mut self , ns : Namespace) -> & mut T { match ns { Namespace :: ValueNS => & mut self . value_ns , Namespace :: TypeNS => & mut self . type_ns , Namespace :: MacroNS => & mut self . macro_ns , } } }}}
mkitem!{mkimpl!{impl < T > PerNS < Option < T > > { # [doc = " Returns `true` if all the items in this collection are `None`."] pub fn is_empty (& self) -> bool { self . type_ns . is_none () && self . value_ns . is_none () && self . macro_ns . is_none () } # [doc = " Returns an iterator over the items which are `Some`."] # [doc = ""] # [doc = " Note: Do you really want to use this? Often you know which namespace a"] # [doc = " name will belong in, and you can consider just that namespace directly,"] # [doc = " rather than iterating through all of them."] pub fn present_items (self) -> impl Iterator < Item = T > { [self . type_ns , self . value_ns , self . macro_ns] . into_iter () . flatten () } }}}
mkitem!{mkimpl!{impl CtorKind { pub fn from_ast (vdata : & ast :: VariantData) -> Option < (CtorKind , NodeId) > { match * vdata { ast :: VariantData :: Tuple (_ , node_id) => Some ((CtorKind :: Fn , node_id)) , ast :: VariantData :: Unit (node_id) => Some ((CtorKind :: Const , node_id)) , ast :: VariantData :: Struct { .. } => None , } } }}}
mkitem!{mkimpl!{impl NonMacroAttrKind { pub fn descr (self) -> & 'static str { match self { NonMacroAttrKind :: Builtin (..) => "built-in attribute" , NonMacroAttrKind :: Tool => "tool attribute" , NonMacroAttrKind :: DeriveHelper | NonMacroAttrKind :: DeriveHelperCompat => { "derive helper attribute" } } } pub fn article (self) -> & 'static str { "a" } # [doc = " Users of some attributes cannot mark them as used, so they are considered always used."] pub fn is_used (self) -> bool { match self { NonMacroAttrKind :: Tool | NonMacroAttrKind :: DeriveHelper | NonMacroAttrKind :: DeriveHelperCompat => true , NonMacroAttrKind :: Builtin (..) => false , } } }}}
mkitem!{mkimpl!{impl < Id > Res < Id > { # [doc = " Return the `DefId` of this `Def` if it has an ID, else panic."] pub fn def_id (& self) -> DefId where Id : Debug , { self . opt_def_id () . unwrap_or_else (| | panic ! ("attempted .def_id() on invalid res: {self:?}")) } # [doc = " Return `Some(..)` with the `DefId` of this `Res` if it has a ID, else `None`."] pub fn opt_def_id (& self) -> Option < DefId > { match * self { Res :: Def (_ , id) => Some (id) , Res :: Local (..) | Res :: PrimTy (..) | Res :: SelfTyParam { .. } | Res :: SelfTyAlias { .. } | Res :: SelfCtor (..) | Res :: ToolMod | Res :: NonMacroAttr (..) | Res :: Err => None , } } # [doc = " Return the `DefId` of this `Res` if it represents a module."] pub fn mod_def_id (& self) -> Option < DefId > { match * self { Res :: Def (DefKind :: Mod , id) => Some (id) , _ => None , } } # [doc = " If this is a \"module\" in name resolution sense, return its `DefId`."] # [inline] pub fn module_like_def_id (& self) -> Option < DefId > { match self { Res :: Def (def_kind , def_id) if def_kind . is_module_like () => Some (* def_id) , _ => None , } } # [doc = " A human readable name for the res kind (\"function\", \"module\", etc.)."] pub fn descr (& self) -> & 'static str { match * self { Res :: Def (kind , def_id) => kind . descr (def_id) , Res :: SelfCtor (..) => "self constructor" , Res :: PrimTy (..) => "builtin type" , Res :: Local (..) => "local variable" , Res :: SelfTyParam { .. } | Res :: SelfTyAlias { .. } => "self type" , Res :: ToolMod => "tool module" , Res :: NonMacroAttr (attr_kind) => attr_kind . descr () , Res :: Err => "unresolved item" , } } # [doc = " Gets an English article for the `Res`."] pub fn article (& self) -> & 'static str { match * self { Res :: Def (kind , _) => kind . article () , Res :: NonMacroAttr (kind) => kind . article () , Res :: Err => "an" , _ => "a" , } } pub fn map_id < R > (self , mut map : impl FnMut (Id) -> R) -> Res < R > { match self { Res :: Def (kind , id) => Res :: Def (kind , id) , Res :: SelfCtor (id) => Res :: SelfCtor (id) , Res :: PrimTy (id) => Res :: PrimTy (id) , Res :: Local (id) => Res :: Local (map (id)) , Res :: SelfTyParam { trait_ } => Res :: SelfTyParam { trait_ } , Res :: SelfTyAlias { alias_to , forbid_generic , is_trait_impl } => { Res :: SelfTyAlias { alias_to , forbid_generic , is_trait_impl } } Res :: ToolMod => Res :: ToolMod , Res :: NonMacroAttr (attr_kind) => Res :: NonMacroAttr (attr_kind) , Res :: Err => Res :: Err , } } pub fn apply_id < R , E > (self , mut map : impl FnMut (Id) -> Result < R , E >) -> Result < Res < R > , E > { Ok (match self { Res :: Def (kind , id) => Res :: Def (kind , id) , Res :: SelfCtor (id) => Res :: SelfCtor (id) , Res :: PrimTy (id) => Res :: PrimTy (id) , Res :: Local (id) => Res :: Local (map (id) ?) , Res :: SelfTyParam { trait_ } => Res :: SelfTyParam { trait_ } , Res :: SelfTyAlias { alias_to , forbid_generic , is_trait_impl } => { Res :: SelfTyAlias { alias_to , forbid_generic , is_trait_impl } } Res :: ToolMod => Res :: ToolMod , Res :: NonMacroAttr (attr_kind) => Res :: NonMacroAttr (attr_kind) , Res :: Err => Res :: Err , }) } # [track_caller] pub fn expect_non_local < OtherId > (self) -> Res < OtherId > { self . map_id (# [track_caller] | _ | panic ! ("unexpected `Res::Local`") ,) } pub fn macro_kinds (self) -> Option < MacroKinds > { match self { Res :: Def (DefKind :: Macro (kinds) , _) => Some (kinds) , Res :: NonMacroAttr (..) => Some (MacroKinds :: ATTR) , _ => None , } } # [doc = " Returns `None` if this is `Res::Err`"] pub fn ns (& self) -> Option < Namespace > { match self { Res :: Def (kind , ..) => kind . ns () , Res :: PrimTy (..) | Res :: SelfTyParam { .. } | Res :: SelfTyAlias { .. } | Res :: ToolMod => { Some (Namespace :: TypeNS) } Res :: SelfCtor (..) | Res :: Local (..) => Some (Namespace :: ValueNS) , Res :: NonMacroAttr (..) => Some (Namespace :: MacroNS) , Res :: Err => None , } } # [doc = " Always returns `true` if `self` is `Res::Err`"] pub fn matches_ns (& self , ns : Namespace) -> bool { self . ns () . is_none_or (| actual_ns | actual_ns == ns) } # [doc = " Returns whether such a resolved path can occur in a tuple struct/variant pattern"] pub fn expected_in_tuple_struct_pat (& self) -> bool { matches ! (self , Res :: Def (DefKind :: Ctor (_ , CtorKind :: Fn) , _) | Res :: SelfCtor (..)) } # [doc = " Returns whether such a resolved path can occur in a unit struct/variant pattern"] pub fn expected_in_unit_struct_pat (& self) -> bool { matches ! (self , Res :: Def (DefKind :: Ctor (_ , CtorKind :: Const) , _) | Res :: SelfCtor (..)) } }}}
mkitem!{mkenum!{# [doc = " Resolution for a lifetime appearing in a type."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum LifetimeRes { # [doc = " Successfully linked the lifetime to a generic parameter."] Param { # [doc = " Id of the generic parameter that introduced it."] param : LocalDefId , # [doc = " Id of the introducing place. That can be:"] # [doc = " - an item's id, for the item's generic parameters;"] # [doc = " - a TraitRef's ref_id, identifying the `for<...>` binder;"] # [doc = " - a FnPtr type's id."] # [doc = ""] # [doc = " This information is used for impl-trait lifetime captures, to know when to or not to"] # [doc = " capture any given lifetime."] binder : NodeId , } , # [doc = " Created a generic parameter for an anonymous lifetime."] Fresh { # [doc = " Id of the generic parameter that introduced it."] # [doc = ""] # [doc = " Creating the associated `LocalDefId` is the responsibility of lowering."] param : NodeId , # [doc = " Id of the introducing place. See `Param`."] binder : NodeId , # [doc = " Kind of elided lifetime"] kind : hir :: MissingLifetimeKind , } , # [doc = " This variant is used for anonymous lifetimes that we did not resolve during"] # [doc = " late resolution. Those lifetimes will be inferred by typechecking."] Infer , # [doc = " `'static` lifetime."] Static , # [doc = " Resolution failure."] Error , # [doc = " HACK: This is used to recover the NodeId of an elided lifetime."] ElidedAnchor { start : NodeId , end : NodeId } , }}}
mkitem!{pub type DocLinkResMap = UnordMap < (Symbol , Namespace) , Option < Res < NodeId > > > ;}