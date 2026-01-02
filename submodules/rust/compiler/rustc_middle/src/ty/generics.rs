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
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use rustc_span :: { Span , Symbol , kw } ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use super :: { Clause , InstantiatedPredicates , ParamConst , ParamTy , Ty , TyCtxt } ;}
mkuse!{use crate :: ty ;}
mkuse!{use crate :: ty :: { EarlyBinder , GenericArgsRef } ;}
mkitem!{mkenum!{#[derive (Clone , Debug , TyEncodable , TyDecodable , HashStable)] pub enum GenericParamDefKind { Lifetime , Type { has_default : bool , synthetic : bool } , Const { has_default : bool } , }}}
mkitem!{mkimpl!{impl GenericParamDefKind { pub fn descr (& self) -> & 'static str { match self { GenericParamDefKind :: Lifetime => "lifetime" , GenericParamDefKind :: Type { .. } => "type" , GenericParamDefKind :: Const { .. } => "constant" , } } pub fn to_ord (& self) -> ast :: ParamKindOrd { match self { GenericParamDefKind :: Lifetime => ast :: ParamKindOrd :: Lifetime , GenericParamDefKind :: Type { .. } | GenericParamDefKind :: Const { .. } => { ast :: ParamKindOrd :: TypeOrConst } } } pub fn is_ty_or_const (& self) -> bool { match self { GenericParamDefKind :: Lifetime => false , GenericParamDefKind :: Type { .. } | GenericParamDefKind :: Const { .. } => true , } } pub fn is_synthetic (& self) -> bool { match self { GenericParamDefKind :: Type { synthetic , .. } => * synthetic , _ => false , } } }}}
mkitem!{mkstruct!{#[derive (Clone , Debug , TyEncodable , TyDecodable , HashStable)] pub struct GenericParamDef { pub name : Symbol , pub def_id : DefId , pub index : u32 , #[doc = " `pure_wrt_drop`, set by the (unsafe) `#[may_dangle]` attribute"] #[doc = " on generic parameter `'a`/`T`, asserts data behind the parameter"] #[doc = " `'a`/`T` won't be accessed during the parent type's `Drop` impl."] pub pure_wrt_drop : bool , pub kind : GenericParamDefKind , }}}
mkitem!{mkimpl!{impl GenericParamDef { pub fn to_early_bound_region_data (& self) -> ty :: EarlyParamRegion { if let GenericParamDefKind :: Lifetime = self . kind { ty :: EarlyParamRegion { index : self . index , name : self . name } } else { bug ! ("cannot convert a non-lifetime parameter def to an early bound region") } } pub fn is_anonymous_lifetime (& self) -> bool { match self . kind { GenericParamDefKind :: Lifetime => self . name == kw :: UnderscoreLifetime , _ => false , } } pub fn default_value < 'tcx > (& self , tcx : TyCtxt < 'tcx > ,) -> Option < EarlyBinder < 'tcx , ty :: GenericArg < 'tcx > > > { match self . kind { GenericParamDefKind :: Type { has_default : true , .. } => { Some (tcx . type_of (self . def_id) . map_bound (| t | t . into ())) } GenericParamDefKind :: Const { has_default : true , .. } => { Some (tcx . const_param_default (self . def_id) . map_bound (| c | c . into ())) } _ => None , } } pub fn to_error < 'tcx > (& self , tcx : TyCtxt < 'tcx >) -> ty :: GenericArg < 'tcx > { match & self . kind { ty :: GenericParamDefKind :: Lifetime => ty :: Region :: new_error_misc (tcx) . into () , ty :: GenericParamDefKind :: Type { .. } => Ty :: new_misc_error (tcx) . into () , ty :: GenericParamDefKind :: Const { .. } => ty :: Const :: new_misc_error (tcx) . into () , } } }}}
mkitem!{mkstruct!{#[derive (Default)] pub struct GenericParamCount { pub lifetimes : usize , pub types : usize , pub consts : usize , }}}
mkitem!{mkstruct!{#[doc = " Information about the formal type/lifetime parameters associated"] #[doc = " with an item or method. Analogous to `hir::Generics`."] #[doc = ""] #[doc = " The ordering of parameters is the same as in [`ty::GenericArg`] (excluding child generics):"] #[doc = " `Self` (optionally), `Lifetime` params..., `Type` params..."] #[derive (Clone , Debug , TyEncodable , TyDecodable , HashStable)] pub struct Generics { pub parent : Option < DefId > , pub parent_count : usize , pub own_params : Vec < GenericParamDef > , #[doc = " Reverse map to the `index` field of each `GenericParamDef`."] #[stable_hasher (ignore)] pub param_def_id_to_index : FxHashMap < DefId , u32 > , pub has_self : bool , pub has_late_bound_regions : Option < Span > , }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: GenericsOf < TyCtxt < 'tcx > > for & 'tcx Generics { fn count (& self) -> usize { self . parent_count + self . own_params . len () } }}}
mkitem!{mkimpl!{impl < 'tcx > Generics { #[doc = " Looks through the generics and all parents to find the index of the"] #[doc = " given param def-id. This is in comparison to the `param_def_id_to_index`"] #[doc = " struct member, which only stores information about this item's own"] #[doc = " generics."] pub fn param_def_id_to_index (& self , tcx : TyCtxt < 'tcx > , def_id : DefId) -> Option < u32 > { if let Some (idx) = self . param_def_id_to_index . get (& def_id) { Some (* idx) } else if let Some (parent) = self . parent { let parent = tcx . generics_of (parent) ; parent . param_def_id_to_index (tcx , def_id) } else { None } } #[inline] pub fn count (& self) -> usize { self . parent_count + self . own_params . len () } pub fn own_counts (& self) -> GenericParamCount { let mut own_counts = GenericParamCount :: default () ; for param in & self . own_params { match param . kind { GenericParamDefKind :: Lifetime => own_counts . lifetimes += 1 , GenericParamDefKind :: Type { .. } => own_counts . types += 1 , GenericParamDefKind :: Const { .. } => own_counts . consts += 1 , } } own_counts } pub fn own_defaults (& self) -> GenericParamCount { let mut own_defaults = GenericParamCount :: default () ; for param in & self . own_params { match param . kind { GenericParamDefKind :: Lifetime => () , GenericParamDefKind :: Type { has_default , .. } => { own_defaults . types += has_default as usize ; } GenericParamDefKind :: Const { has_default , .. } => { own_defaults . consts += has_default as usize ; } } } own_defaults } pub fn requires_monomorphization (& self , tcx : TyCtxt < 'tcx >) -> bool { if self . own_requires_monomorphization () { return true ; } if let Some (parent_def_id) = self . parent { let parent = tcx . generics_of (parent_def_id) ; parent . requires_monomorphization (tcx) } else { false } } pub fn own_requires_monomorphization (& self) -> bool { for param in & self . own_params { match param . kind { GenericParamDefKind :: Type { .. } | GenericParamDefKind :: Const { .. } => { return true ; } GenericParamDefKind :: Lifetime => { } } } false } #[doc = " Returns the `GenericParamDef` with the given index."] pub fn param_at (& 'tcx self , param_index : usize , tcx : TyCtxt < 'tcx >) -> & 'tcx GenericParamDef { if let Some (index) = param_index . checked_sub (self . parent_count) { & self . own_params [index] } else { tcx . generics_of (self . parent . expect ("parent_count > 0 but no parent?")) . param_at (param_index , tcx) } } pub fn params_to (& 'tcx self , param_index : usize , tcx : TyCtxt < 'tcx >) -> & 'tcx [GenericParamDef] { if let Some (index) = param_index . checked_sub (self . parent_count) { & self . own_params [.. index] } else { tcx . generics_of (self . parent . expect ("parent_count > 0 but no parent?")) . params_to (param_index , tcx) } } #[doc = " Returns the `GenericParamDef` associated with this `EarlyParamRegion`."] pub fn region_param (& 'tcx self , param : ty :: EarlyParamRegion , tcx : TyCtxt < 'tcx > ,) -> & 'tcx GenericParamDef { let param = self . param_at (param . index as usize , tcx) ; match param . kind { GenericParamDefKind :: Lifetime => param , _ => { bug ! ("expected lifetime parameter, but found another generic parameter: {param:#?}") } } } #[doc = " Returns the `GenericParamDef` associated with this `ParamTy`."] pub fn type_param (& 'tcx self , param : ParamTy , tcx : TyCtxt < 'tcx >) -> & 'tcx GenericParamDef { let param = self . param_at (param . index as usize , tcx) ; match param . kind { GenericParamDefKind :: Type { .. } => param , _ => bug ! ("expected type parameter, but found another generic parameter: {param:#?}") , } } #[doc = " Returns the `GenericParamDef` associated with this `ParamConst`."] pub fn const_param (& 'tcx self , param : ParamConst , tcx : TyCtxt < 'tcx >) -> & 'tcx GenericParamDef { let param = self . param_at (param . index as usize , tcx) ; match param . kind { GenericParamDefKind :: Const { .. } => param , _ => bug ! ("expected const parameter, but found another generic parameter: {param:#?}") , } } #[doc = " Returns `true` if `params` has `impl Trait`."] pub fn has_impl_trait (& 'tcx self) -> bool { self . own_params . iter () . any (| param | { matches ! (param . kind , ty :: GenericParamDefKind :: Type { synthetic : true , .. }) }) } #[doc = " Returns the args corresponding to the generic parameters"] #[doc = " of this item, excluding `Self`."] #[doc = ""] #[doc = " **This should only be used for diagnostics purposes.**"] pub fn own_args_no_defaults < 'a > (& 'tcx self , tcx : TyCtxt < 'tcx > , args : & 'a [ty :: GenericArg < 'tcx >] ,) -> & 'a [ty :: GenericArg < 'tcx >] { let mut own_params = self . parent_count .. self . count () ; if self . has_self && self . parent . is_none () { own_params . start = 1 ; } own_params . end -= self . own_params . iter () . rev () . take_while (| param | { param . default_value (tcx) . is_some_and (| default | { default . instantiate (tcx , args) == args [param . index as usize] }) }) . count () ; & args [own_params] } #[doc = " Returns the args corresponding to the generic parameters of this item, excluding `Self`."] #[doc = ""] #[doc = " **This should only be used for diagnostics purposes.**"] pub fn own_args (& 'tcx self , args : & 'tcx [ty :: GenericArg < 'tcx >] ,) -> & 'tcx [ty :: GenericArg < 'tcx >] { let own = & args [self . parent_count ..] [.. self . own_params . len ()] ; if self . has_self && self . parent . is_none () { & own [1 ..] } else { own } } #[doc = " Returns true if a concrete type is specified after a default type."] #[doc = " For example, consider `struct T<W = usize, X = Vec<W>>(W, X)`"] #[doc = " `T<usize, String>` will return true"] #[doc = " `T<usize>` will return false"] pub fn check_concrete_type_after_default (& 'tcx self , tcx : TyCtxt < 'tcx > , args : & 'tcx [ty :: GenericArg < 'tcx >] ,) -> bool { let mut default_param_seen = false ; for param in self . own_params . iter () { if let Some (inst) = param . default_value (tcx) . map (| default | default . instantiate (tcx , args)) { if inst == args [param . index as usize] { default_param_seen = true ; } else if default_param_seen { return true ; } } } false } pub fn is_empty (& 'tcx self) -> bool { self . count () == 0 } pub fn is_own_empty (& 'tcx self) -> bool { self . own_params . is_empty () } }}}
mkitem!{mkstruct!{#[doc = " Bounds on generics."] #[derive (Copy , Clone , Default , Debug , TyEncodable , TyDecodable , HashStable)] pub struct GenericPredicates < 'tcx > { pub parent : Option < DefId > , pub predicates : & 'tcx [(Clause < 'tcx > , Span)] , }}}
mkitem!{mkimpl!{impl < 'tcx > GenericPredicates < 'tcx > { pub fn instantiate (self , tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > ,) -> InstantiatedPredicates < 'tcx > { let mut instantiated = InstantiatedPredicates :: empty () ; self . instantiate_into (tcx , & mut instantiated , args) ; instantiated } pub fn instantiate_own (self , tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > ,) -> impl Iterator < Item = (Clause < 'tcx > , Span) > + DoubleEndedIterator + ExactSizeIterator { EarlyBinder :: bind (self . predicates) . iter_instantiated_copied (tcx , args) } pub fn instantiate_own_identity (self ,) -> impl Iterator < Item = (Clause < 'tcx > , Span) > + DoubleEndedIterator + ExactSizeIterator { EarlyBinder :: bind (self . predicates) . iter_identity_copied () } #[instrument (level = "debug" , skip (self , tcx))] fn instantiate_into (self , tcx : TyCtxt < 'tcx > , instantiated : & mut InstantiatedPredicates < 'tcx > , args : GenericArgsRef < 'tcx > ,) { if let Some (def_id) = self . parent { tcx . predicates_of (def_id) . instantiate_into (tcx , instantiated , args) ; } instantiated . predicates . extend (self . predicates . iter () . map (| (p , _) | EarlyBinder :: bind (* p) . instantiate (tcx , args)) ,) ; instantiated . spans . extend (self . predicates . iter () . map (| (_ , sp) | * sp)) ; } pub fn instantiate_identity (self , tcx : TyCtxt < 'tcx >) -> InstantiatedPredicates < 'tcx > { let mut instantiated = InstantiatedPredicates :: empty () ; self . instantiate_identity_into (tcx , & mut instantiated) ; instantiated } fn instantiate_identity_into (self , tcx : TyCtxt < 'tcx > , instantiated : & mut InstantiatedPredicates < 'tcx > ,) { if let Some (def_id) = self . parent { tcx . predicates_of (def_id) . instantiate_identity_into (tcx , instantiated) ; } instantiated . predicates . extend (self . predicates . iter () . map (| (p , _) | p)) ; instantiated . spans . extend (self . predicates . iter () . map (| (_ , s) | s)) ; } }}}
mkitem!{mkstruct!{#[doc = " `[const]` bounds for a given item. This is represented using a struct much like"] #[doc = " `GenericPredicates`, where you can either choose to only instantiate the \"own\""] #[doc = " bounds or all of the bounds including those from the parent. This distinction"] #[doc = " is necessary for code like `compare_method_predicate_entailment`."] #[derive (Copy , Clone , Default , Debug , TyEncodable , TyDecodable , HashStable)] pub struct ConstConditions < 'tcx > { pub parent : Option < DefId > , pub predicates : & 'tcx [(ty :: PolyTraitRef < 'tcx > , Span)] , }}}
mkitem!{mkimpl!{impl < 'tcx > ConstConditions < 'tcx > { pub fn instantiate (self , tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > ,) -> Vec < (ty :: PolyTraitRef < 'tcx > , Span) > { let mut instantiated = vec ! [] ; self . instantiate_into (tcx , & mut instantiated , args) ; instantiated } pub fn instantiate_own (self , tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > ,) -> impl Iterator < Item = (ty :: PolyTraitRef < 'tcx > , Span) > + DoubleEndedIterator + ExactSizeIterator { EarlyBinder :: bind (self . predicates) . iter_instantiated_copied (tcx , args) } pub fn instantiate_own_identity (self ,) -> impl Iterator < Item = (ty :: PolyTraitRef < 'tcx > , Span) > + DoubleEndedIterator + ExactSizeIterator { EarlyBinder :: bind (self . predicates) . iter_identity_copied () } #[instrument (level = "debug" , skip (self , tcx))] fn instantiate_into (self , tcx : TyCtxt < 'tcx > , instantiated : & mut Vec < (ty :: PolyTraitRef < 'tcx > , Span) > , args : GenericArgsRef < 'tcx > ,) { if let Some (def_id) = self . parent { tcx . const_conditions (def_id) . instantiate_into (tcx , instantiated , args) ; } instantiated . extend (self . predicates . iter () . map (| & (p , s) | (EarlyBinder :: bind (p) . instantiate (tcx , args) , s)) ,) ; } pub fn instantiate_identity (self , tcx : TyCtxt < 'tcx >) -> Vec < (ty :: PolyTraitRef < 'tcx > , Span) > { let mut instantiated = vec ! [] ; self . instantiate_identity_into (tcx , & mut instantiated) ; instantiated } fn instantiate_identity_into (self , tcx : TyCtxt < 'tcx > , instantiated : & mut Vec < (ty :: PolyTraitRef < 'tcx > , Span) > ,) { if let Some (def_id) = self . parent { tcx . const_conditions (def_id) . instantiate_identity_into (tcx , instantiated) ; } instantiated . extend (self . predicates . iter () . copied ()) ; } }}}