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
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: visit :: Flags ;}
mkuse!{use crate :: { self as ty , Interner } ;}
mkitem!{bitflags :: bitflags ! { #[doc = " Flags that we track on types. These flags are propagated upwards"] #[doc = " through the type during type construction, so that we can quickly check"] #[doc = " whether the type has various kinds of types in it without recursing"] #[doc = " over the type itself."] #[derive (Debug , PartialEq , Eq , Clone , Copy)] pub struct TypeFlags : u32 { #[doc = " Does this have `Param`?"] const HAS_TY_PARAM = 1 << 0 ; #[doc = " Does this have `ReEarlyParam`?"] const HAS_RE_PARAM = 1 << 1 ; #[doc = " Does this have `ConstKind::Param`?"] const HAS_CT_PARAM = 1 << 2 ; const HAS_PARAM = TypeFlags :: HAS_TY_PARAM . bits () | TypeFlags :: HAS_RE_PARAM . bits () | TypeFlags :: HAS_CT_PARAM . bits () ; #[doc = " Does this have `Infer`?"] const HAS_TY_INFER = 1 << 3 ; #[doc = " Does this have `ReVar`?"] const HAS_RE_INFER = 1 << 4 ; #[doc = " Does this have `ConstKind::Infer`?"] const HAS_CT_INFER = 1 << 5 ; #[doc = " Does this have inference variables? Used to determine whether"] #[doc = " inference is required."] const HAS_INFER = TypeFlags :: HAS_TY_INFER . bits () | TypeFlags :: HAS_RE_INFER . bits () | TypeFlags :: HAS_CT_INFER . bits () ; #[doc = " Does this have `Placeholder`?"] const HAS_TY_PLACEHOLDER = 1 << 6 ; #[doc = " Does this have `RePlaceholder`?"] const HAS_RE_PLACEHOLDER = 1 << 7 ; #[doc = " Does this have `ConstKind::Placeholder`?"] const HAS_CT_PLACEHOLDER = 1 << 8 ; #[doc = " Does this have placeholders?"] const HAS_PLACEHOLDER = TypeFlags :: HAS_TY_PLACEHOLDER . bits () | TypeFlags :: HAS_RE_PLACEHOLDER . bits () | TypeFlags :: HAS_CT_PLACEHOLDER . bits () ; #[doc = " `true` if there are \"names\" of regions and so forth"] #[doc = " that are local to a particular fn/inferctxt"] const HAS_FREE_LOCAL_REGIONS = 1 << 9 ; #[doc = " `true` if there are \"names\" of types and regions and so forth"] #[doc = " that are local to a particular fn"] const HAS_FREE_LOCAL_NAMES = TypeFlags :: HAS_TY_PARAM . bits () | TypeFlags :: HAS_CT_PARAM . bits () | TypeFlags :: HAS_TY_INFER . bits () | TypeFlags :: HAS_CT_INFER . bits () | TypeFlags :: HAS_TY_PLACEHOLDER . bits () | TypeFlags :: HAS_CT_PLACEHOLDER . bits () | TypeFlags :: HAS_TY_FRESH . bits () | TypeFlags :: HAS_CT_FRESH . bits () | TypeFlags :: HAS_FREE_LOCAL_REGIONS . bits () | TypeFlags :: HAS_RE_ERASED . bits () ; #[doc = " Does this have `Projection`?"] const HAS_TY_PROJECTION = 1 << 10 ; #[doc = " Does this have `Free` aliases?"] const HAS_TY_FREE_ALIAS = 1 << 11 ; #[doc = " Does this have `Opaque`?"] const HAS_TY_OPAQUE = 1 << 12 ; #[doc = " Does this have `Inherent`?"] const HAS_TY_INHERENT = 1 << 13 ; #[doc = " Does this have `ConstKind::Unevaluated`?"] const HAS_CT_PROJECTION = 1 << 14 ; #[doc = " Does this have `Alias` or `ConstKind::Unevaluated`?"] #[doc = ""] #[doc = " Rephrased, could this term be normalized further?"] const HAS_ALIAS = TypeFlags :: HAS_TY_PROJECTION . bits () | TypeFlags :: HAS_TY_FREE_ALIAS . bits () | TypeFlags :: HAS_TY_OPAQUE . bits () | TypeFlags :: HAS_TY_INHERENT . bits () | TypeFlags :: HAS_CT_PROJECTION . bits () ; #[doc = " Is an error type/lifetime/const reachable?"] const HAS_ERROR = 1 << 15 ; #[doc = " Does this have any region that \"appears free\" in the type?"] #[doc = " Basically anything but `ReBound` and `ReErased`."] const HAS_FREE_REGIONS = 1 << 16 ; #[doc = " Does this have any `ReBound` regions?"] const HAS_RE_BOUND = 1 << 17 ; #[doc = " Does this have any `Bound` types?"] const HAS_TY_BOUND = 1 << 18 ; #[doc = " Does this have any `ConstKind::Bound` consts?"] const HAS_CT_BOUND = 1 << 19 ; #[doc = " Does this have any bound variables?"] #[doc = " Used to check if a global bound is safe to evaluate."] const HAS_BOUND_VARS = TypeFlags :: HAS_RE_BOUND . bits () | TypeFlags :: HAS_TY_BOUND . bits () | TypeFlags :: HAS_CT_BOUND . bits () ; #[doc = " Does this have any `ReErased` regions?"] const HAS_RE_ERASED = 1 << 20 ; #[doc = " Does this value have parameters/placeholders/inference variables which could be"] #[doc = " replaced later, in a way that would change the results of `impl` specialization?"] const STILL_FURTHER_SPECIALIZABLE = TypeFlags :: HAS_TY_PARAM . bits () | TypeFlags :: HAS_TY_PLACEHOLDER . bits () | TypeFlags :: HAS_TY_INFER . bits () | TypeFlags :: HAS_CT_PARAM . bits () | TypeFlags :: HAS_CT_PLACEHOLDER . bits () | TypeFlags :: HAS_CT_INFER . bits () ; #[doc = " Does this value have `InferTy::FreshTy/FreshIntTy/FreshFloatTy`?"] const HAS_TY_FRESH = 1 << 21 ; #[doc = " Does this value have `InferConst::Fresh`?"] const HAS_CT_FRESH = 1 << 22 ; #[doc = " Does this have any binders with bound vars (e.g. that need to be anonymized)?"] const HAS_BINDER_VARS = 1 << 23 ; #[doc = " Does this type have any coroutines in it?"] const HAS_TY_CORO = 1 << 24 ; } }}
mkitem!{mkstruct!{#[derive (Debug)] pub struct FlagComputation < I > { pub flags : TypeFlags , #[doc = " see `Ty::outer_exclusive_binder` for details"] pub outer_exclusive_binder : ty :: DebruijnIndex , interner : std :: marker :: PhantomData < I > , }}}
mkitem!{mkimpl!{impl < I : Interner > FlagComputation < I > { fn new () -> FlagComputation < I > { FlagComputation { flags : TypeFlags :: empty () , outer_exclusive_binder : ty :: INNERMOST , interner : std :: marker :: PhantomData , } } #[allow (rustc :: usage_of_ty_tykind)] pub fn for_kind (kind : & ty :: TyKind < I >) -> FlagComputation < I > { let mut result = FlagComputation :: new () ; result . add_kind (kind) ; result } pub fn for_predicate (binder : ty :: Binder < I , ty :: PredicateKind < I > >) -> FlagComputation < I > { let mut result = FlagComputation :: new () ; result . add_predicate (binder) ; result } pub fn for_const_kind (kind : & ty :: ConstKind < I >) -> FlagComputation < I > { let mut result = FlagComputation :: new () ; result . add_const_kind (kind) ; result } pub fn for_clauses (clauses : & [I :: Clause]) -> FlagComputation < I > { let mut result = FlagComputation :: new () ; for c in clauses { result . add_flags (c . as_predicate () . flags ()) ; result . add_exclusive_binder (c . as_predicate () . outer_exclusive_binder ()) ; } result } fn add_flags (& mut self , flags : TypeFlags) { self . flags = self . flags | flags ; } #[doc = " indicates that `self` refers to something at binding level `binder`"] fn add_bound_var (& mut self , binder : ty :: DebruijnIndex) { let exclusive_binder = binder . shifted_in (1) ; self . add_exclusive_binder (exclusive_binder) ; } #[doc = " indicates that `self` refers to something *inside* binding"] #[doc = " level `binder` -- not bound by `binder`, but bound by the next"] #[doc = " binder internal to it"] fn add_exclusive_binder (& mut self , exclusive_binder : ty :: DebruijnIndex) { self . outer_exclusive_binder = self . outer_exclusive_binder . max (exclusive_binder) ; } #[doc = " Adds the flags/depth from a set of types that appear within the current type, but within a"] #[doc = " region binder."] fn bound_computation < T , F > (& mut self , value : ty :: Binder < I , T > , f : F) where F : FnOnce (& mut Self , T) , { let mut computation = FlagComputation :: new () ; if ! value . bound_vars () . is_empty () { computation . add_flags (TypeFlags :: HAS_BINDER_VARS) ; } f (& mut computation , value . skip_binder ()) ; self . add_flags (computation . flags) ; let outer_exclusive_binder = computation . outer_exclusive_binder ; if outer_exclusive_binder > ty :: INNERMOST { self . add_exclusive_binder (outer_exclusive_binder . shifted_out (1)) ; } } #[allow (rustc :: usage_of_ty_tykind)] fn add_kind (& mut self , kind : & ty :: TyKind < I >) { match * kind { ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Float (_) | ty :: Uint (_) | ty :: Never | ty :: Str | ty :: Foreign (..) => { } ty :: Error (_) => self . add_flags (TypeFlags :: HAS_ERROR) , ty :: Param (_) => { self . add_flags (TypeFlags :: HAS_TY_PARAM) ; } ty :: Closure (_ , args) | ty :: CoroutineClosure (_ , args) | ty :: CoroutineWitness (_ , args) => { self . add_args (args . as_slice ()) ; } ty :: Coroutine (_ , args) => { self . add_flags (TypeFlags :: HAS_TY_CORO) ; self . add_args (args . as_slice ()) ; } ty :: Bound (debruijn , _) => { self . add_bound_var (debruijn) ; self . add_flags (TypeFlags :: HAS_TY_BOUND) ; } ty :: Placeholder (..) => { self . add_flags (TypeFlags :: HAS_TY_PLACEHOLDER) ; } ty :: Infer (infer) => match infer { ty :: FreshTy (_) | ty :: FreshIntTy (_) | ty :: FreshFloatTy (_) => { self . add_flags (TypeFlags :: HAS_TY_FRESH) } ty :: TyVar (_) | ty :: IntVar (_) | ty :: FloatVar (_) => { self . add_flags (TypeFlags :: HAS_TY_INFER) } } , ty :: Adt (_ , args) => { self . add_args (args . as_slice ()) ; } ty :: Alias (kind , data) => { self . add_flags (match kind { ty :: Projection => TypeFlags :: HAS_TY_PROJECTION , ty :: Free => TypeFlags :: HAS_TY_FREE_ALIAS , ty :: Opaque => TypeFlags :: HAS_TY_OPAQUE , ty :: Inherent => TypeFlags :: HAS_TY_INHERENT , }) ; self . add_alias_ty (data) ; } ty :: Dynamic (obj , r , _) => { for predicate in obj . iter () { self . bound_computation (predicate , | computation , predicate | match predicate { ty :: ExistentialPredicate :: Trait (tr) => { computation . add_args (tr . args . as_slice ()) } ty :: ExistentialPredicate :: Projection (p) => { computation . add_existential_projection (& p) ; } ty :: ExistentialPredicate :: AutoTrait (_) => { } }) ; } self . add_region (r) ; } ty :: Array (tt , len) => { self . add_ty (tt) ; self . add_const (len) ; } ty :: Pat (ty , pat) => { self . add_ty (ty) ; self . add_ty_pat (pat) ; } ty :: Slice (tt) => self . add_ty (tt) , ty :: RawPtr (ty , _) => { self . add_ty (ty) ; } ty :: Ref (r , ty , _) => { self . add_region (r) ; self . add_ty (ty) ; } ty :: Tuple (types) => { self . add_tys (types) ; } ty :: FnDef (_ , args) => { self . add_args (args . as_slice ()) ; } ty :: FnPtr (sig_tys , _) => self . bound_computation (sig_tys , | computation , sig_tys | { computation . add_tys (sig_tys . inputs_and_output) ; }) , ty :: UnsafeBinder (bound_ty) => { self . bound_computation (bound_ty . into () , | computation , ty | { computation . add_ty (ty) ; }) } } } fn add_ty_pat (& mut self , pat : < I as Interner > :: Pat) { self . add_flags (pat . flags ()) ; } fn add_predicate (& mut self , binder : ty :: Binder < I , ty :: PredicateKind < I > >) { self . bound_computation (binder , | computation , atom | computation . add_predicate_atom (atom)) ; } fn add_predicate_atom (& mut self , atom : ty :: PredicateKind < I >) { match atom { ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (trait_pred)) => { self . add_args (trait_pred . trait_ref . args . as_slice ()) ; } ty :: PredicateKind :: Clause (ty :: ClauseKind :: HostEffect (ty :: HostEffectPredicate { trait_ref , constness : _ , })) => { self . add_args (trait_ref . args . as_slice ()) ; } ty :: PredicateKind :: Clause (ty :: ClauseKind :: RegionOutlives (ty :: OutlivesPredicate (a , b ,))) => { self . add_region (a) ; self . add_region (b) ; } ty :: PredicateKind :: Clause (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty , region ,))) => { self . add_ty (ty) ; self . add_region (region) ; } ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstArgHasType (ct , ty)) => { self . add_const (ct) ; self . add_ty (ty) ; } ty :: PredicateKind :: Subtype (ty :: SubtypePredicate { a_is_expected : _ , a , b }) => { self . add_ty (a) ; self . add_ty (b) ; } ty :: PredicateKind :: Coerce (ty :: CoercePredicate { a , b }) => { self . add_ty (a) ; self . add_ty (b) ; } ty :: PredicateKind :: Clause (ty :: ClauseKind :: Projection (ty :: ProjectionPredicate { projection_term , term , })) => { self . add_alias_term (projection_term) ; self . add_term (term) ; } ty :: PredicateKind :: Clause (ty :: ClauseKind :: WellFormed (term)) => { self . add_term (term) ; } ty :: PredicateKind :: DynCompatible (_def_id) => { } ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstEvaluatable (uv)) => { self . add_const (uv) ; } ty :: PredicateKind :: ConstEquate (expected , found) => { self . add_const (expected) ; self . add_const (found) ; } ty :: PredicateKind :: NormalizesTo (ty :: NormalizesTo { alias , term }) => { self . add_alias_term (alias) ; self . add_term (term) ; } ty :: PredicateKind :: AliasRelate (t1 , t2 , _) => { self . add_term (t1) ; self . add_term (t2) ; } ty :: PredicateKind :: Clause (ty :: ClauseKind :: UnstableFeature (_sym)) => { } ty :: PredicateKind :: Ambiguous => { } } } fn add_ty (& mut self , ty : I :: Ty) { self . add_flags (ty . flags ()) ; self . add_exclusive_binder (ty . outer_exclusive_binder ()) ; } fn add_tys (& mut self , tys : I :: Tys) { for ty in tys . iter () { self . add_ty (ty) ; } } fn add_region (& mut self , r : I :: Region) { self . add_flags (r . flags ()) ; if let ty :: ReBound (debruijn , _) = r . kind () { self . add_bound_var (debruijn) ; } } fn add_const (& mut self , c : I :: Const) { self . add_flags (c . flags ()) ; self . add_exclusive_binder (c . outer_exclusive_binder ()) ; } fn add_const_kind (& mut self , c : & ty :: ConstKind < I >) { match * c { ty :: ConstKind :: Unevaluated (uv) => { self . add_args (uv . args . as_slice ()) ; self . add_flags (TypeFlags :: HAS_CT_PROJECTION) ; } ty :: ConstKind :: Infer (infer) => match infer { ty :: InferConst :: Fresh (_) => self . add_flags (TypeFlags :: HAS_CT_FRESH) , ty :: InferConst :: Var (_) => self . add_flags (TypeFlags :: HAS_CT_INFER) , } , ty :: ConstKind :: Bound (debruijn , _) => { self . add_bound_var (debruijn) ; self . add_flags (TypeFlags :: HAS_CT_BOUND) ; } ty :: ConstKind :: Param (_) => { self . add_flags (TypeFlags :: HAS_CT_PARAM) ; } ty :: ConstKind :: Placeholder (_) => { self . add_flags (TypeFlags :: HAS_CT_PLACEHOLDER) ; } ty :: ConstKind :: Value (cv) => self . add_ty (cv . ty ()) , ty :: ConstKind :: Expr (e) => self . add_args (e . args () . as_slice ()) , ty :: ConstKind :: Error (_) => self . add_flags (TypeFlags :: HAS_ERROR) , } } fn add_existential_projection (& mut self , projection : & ty :: ExistentialProjection < I >) { self . add_args (projection . args . as_slice ()) ; match projection . term . kind () { ty :: TermKind :: Ty (ty) => self . add_ty (ty) , ty :: TermKind :: Const (ct) => self . add_const (ct) , } } fn add_alias_ty (& mut self , alias_ty : ty :: AliasTy < I >) { self . add_args (alias_ty . args . as_slice ()) ; } fn add_alias_term (& mut self , alias_term : ty :: AliasTerm < I >) { self . add_args (alias_term . args . as_slice ()) ; } fn add_args (& mut self , args : & [I :: GenericArg]) { for arg in args { match arg . kind () { ty :: GenericArgKind :: Type (ty) => self . add_ty (ty) , ty :: GenericArgKind :: Lifetime (lt) => self . add_region (lt) , ty :: GenericArgKind :: Const (ct) => self . add_const (ct) , } } } fn add_term (& mut self , term : I :: Term) { match term . kind () { ty :: TermKind :: Ty (ty) => self . add_ty (ty) , ty :: TermKind :: Const (ct) => self . add_const (ct) , } } }}}