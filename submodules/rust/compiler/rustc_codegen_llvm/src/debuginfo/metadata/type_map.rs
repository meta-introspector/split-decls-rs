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
mkuse!{use rustc_abi :: { Align , Size , VariantIdx } ;}
mkuse!{use rustc_data_structures :: fingerprint :: Fingerprint ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ;}
mkuse!{use rustc_macros :: HashStable ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: { self , ExistentialTraitRef , Ty , TyCtxt } ;}
mkuse!{use super :: { DefinitionLocation , SmallVec , UNKNOWN_LINE_NUMBER , unknown_file_metadata } ;}
mkuse!{use crate :: common :: { AsCCharPtr , CodegenCx } ;}
mkuse!{use crate :: debuginfo :: utils :: { DIB , create_DIArray , debug_context } ;}
mkuse!{use crate :: llvm :: debuginfo :: { DIFlags , DIScope , DIType } ;}
mkuse!{use crate :: llvm :: { self } ;}
mkmod!{private, { 
                getname!(private);
                getsrc!(private);
                getpath!(private);
                get_deps!(private);
                get_crates!(private);
                mkinclude!(private);
                mkuse!{use rustc_macros :: HashStable ;}
mkitem!{mkstruct!{#[derive (Debug , Clone , Copy , PartialEq , Eq , Hash , HashStable)] pub (crate) struct HiddenZst ;}} 
            }}
mkitem!{mkenum!{#[doc = " A unique identifier for anything that we create a debuginfo node for."] #[doc = " The types it contains are expected to already be normalized (which"] #[doc = " is asserted in the constructors)."] #[doc = ""] #[doc = " Note that there are some things that only show up in debuginfo, like"] #[doc = " the separate type descriptions for each enum variant. These get an ID"] #[doc = " too because they have their own debuginfo node in LLVM IR."] #[derive (Debug , Clone , Copy , PartialEq , Eq , Hash , HashStable)] pub (super) enum UniqueTypeId < 'tcx > { #[doc = " The ID of a regular type as it shows up at the language level."] Ty (Ty < 'tcx > , private :: HiddenZst) , #[doc = " The ID for the single DW_TAG_variant_part nested inside the top-level"] #[doc = " DW_TAG_structure_type that describes enums and coroutines."] VariantPart (Ty < 'tcx > , private :: HiddenZst) , #[doc = " The ID for the artificial struct type describing a single enum variant."] VariantStructType (Ty < 'tcx > , VariantIdx , private :: HiddenZst) , #[doc = " The ID for the additional wrapper struct type describing an enum variant in CPP-like mode."] VariantStructTypeCppLikeWrapper (Ty < 'tcx > , VariantIdx , private :: HiddenZst) , #[doc = " The ID of the artificial type we create for VTables."] VTableTy (Ty < 'tcx > , Option < ExistentialTraitRef < 'tcx > > , private :: HiddenZst) , }}}
mkitem!{mkimpl!{impl < 'tcx > UniqueTypeId < 'tcx > { pub (crate) fn for_ty (tcx : TyCtxt < 'tcx > , t : Ty < 'tcx >) -> Self { assert_eq ! (t , tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , t)) ; UniqueTypeId :: Ty (t , private :: HiddenZst) } pub (crate) fn for_enum_variant_part (tcx : TyCtxt < 'tcx > , enum_ty : Ty < 'tcx >) -> Self { assert_eq ! (enum_ty , tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , enum_ty)) ; UniqueTypeId :: VariantPart (enum_ty , private :: HiddenZst) } pub (crate) fn for_enum_variant_struct_type (tcx : TyCtxt < 'tcx > , enum_ty : Ty < 'tcx > , variant_idx : VariantIdx ,) -> Self { assert_eq ! (enum_ty , tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , enum_ty)) ; UniqueTypeId :: VariantStructType (enum_ty , variant_idx , private :: HiddenZst) } pub (crate) fn for_enum_variant_struct_type_wrapper (tcx : TyCtxt < 'tcx > , enum_ty : Ty < 'tcx > , variant_idx : VariantIdx ,) -> Self { assert_eq ! (enum_ty , tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , enum_ty)) ; UniqueTypeId :: VariantStructTypeCppLikeWrapper (enum_ty , variant_idx , private :: HiddenZst) } pub (crate) fn for_vtable_ty (tcx : TyCtxt < 'tcx > , self_type : Ty < 'tcx > , implemented_trait : Option < ExistentialTraitRef < 'tcx > > ,) -> Self { assert_eq ! (self_type , tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , self_type)) ; assert_eq ! (implemented_trait , tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , implemented_trait)) ; UniqueTypeId :: VTableTy (self_type , implemented_trait , private :: HiddenZst) } #[doc = " Generates a string version of this [UniqueTypeId], which can be used as the `UniqueId`"] #[doc = " argument of the various `LLVMRustDIBuilderCreate*Type()` methods."] #[doc = ""] #[doc = " Right now this takes the form of a hex-encoded opaque hash value."] fn generate_unique_id_string (self , tcx : TyCtxt < 'tcx >) -> String { let mut hasher = StableHasher :: new () ; tcx . with_stable_hashing_context (| mut hcx | { hcx . while_hashing_spans (false , | hcx | self . hash_stable (hcx , & mut hasher)) }) ; hasher . finish :: < Fingerprint > () . to_hex () } pub (crate) fn expect_ty (self) -> Ty < 'tcx > { match self { UniqueTypeId :: Ty (ty , _) => ty , _ => bug ! ("Expected `UniqueTypeId::Ty` but found `{:?}`" , self) , } } }}}
mkitem!{mkstruct!{#[doc = " The `TypeMap` is where the debug context holds the type metadata nodes"] #[doc = " created so far. The debuginfo nodes are identified by `UniqueTypeId`."] #[derive (Default)] pub (crate) struct TypeMap < 'll , 'tcx > { pub (super) unique_id_to_di_node : RefCell < FxHashMap < UniqueTypeId < 'tcx > , & 'll DIType > > , }}}
mkitem!{mkimpl!{impl < 'll , 'tcx > TypeMap < 'll , 'tcx > { #[doc = " Adds a `UniqueTypeId` to metadata mapping to the `TypeMap`. The method will"] #[doc = " fail if the mapping already exists."] pub (super) fn insert (& self , unique_type_id : UniqueTypeId < 'tcx > , metadata : & 'll DIType) { if self . unique_id_to_di_node . borrow_mut () . insert (unique_type_id , metadata) . is_some () { bug ! ("type metadata for unique ID '{:?}' is already in the `TypeMap`!" , unique_type_id) ; } } pub (super) fn di_node_for_unique_id (& self , unique_type_id : UniqueTypeId < 'tcx > ,) -> Option < & 'll DIType > { self . unique_id_to_di_node . borrow () . get (& unique_type_id) . cloned () } }}}
mkitem!{mkstruct!{pub (crate) struct DINodeCreationResult < 'll > { pub di_node : & 'll DIType , pub already_stored_in_typemap : bool , }}}
mkitem!{mkimpl!{impl < 'll > DINodeCreationResult < 'll > { pub (crate) fn new (di_node : & 'll DIType , already_stored_in_typemap : bool) -> Self { DINodeCreationResult { di_node , already_stored_in_typemap } } }}}
mkitem!{mkenum!{#[derive (Debug , Copy , Clone , Eq , PartialEq)] pub (crate) enum Stub < 'll > { Struct , Union , VTableTy { vtable_holder : & 'll DIType } , }}}
mkitem!{mkstruct!{pub (crate) struct StubInfo < 'll , 'tcx > { metadata : & 'll DIType , unique_type_id : UniqueTypeId < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'll , 'tcx > StubInfo < 'll , 'tcx > { pub (super) fn new (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > , build : impl FnOnce (& CodegenCx < 'll , 'tcx > , & str) -> & 'll DIType ,) -> StubInfo < 'll , 'tcx > { let unique_type_id_str = unique_type_id . generate_unique_id_string (cx . tcx) ; let di_node = build (cx , & unique_type_id_str) ; StubInfo { metadata : di_node , unique_type_id } } }}}

macro_rules! stub_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stub in module {}", module_path!());
    };
}

mkfn!{
    stub_introspect!();
    #[doc = " Create a stub debuginfo node onto which fields and nested types can be attached."] pub (super) fn stub < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , kind : Stub < 'll > , unique_type_id : UniqueTypeId < 'tcx > , name : & str , def_location : Option < DefinitionLocation < 'll > > , (size , align) : (Size , Align) , containing_scope : Option < & 'll DIScope > , flags : DIFlags ,) -> StubInfo < 'll , 'tcx > { let empty_array = create_DIArray (DIB (cx) , & []) ; let unique_type_id_str = unique_type_id . generate_unique_id_string (cx . tcx) ; let (file_metadata , line_number) = if let Some (def_location) = def_location { (def_location . 0 , def_location . 1) } else { (unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER) } ; let metadata = match kind { Stub :: Struct | Stub :: VTableTy { .. } => { let vtable_holder = match kind { Stub :: VTableTy { vtable_holder } => Some (vtable_holder) , _ => None , } ; unsafe { llvm :: LLVMRustDIBuilderCreateStructType (DIB (cx) , containing_scope , name . as_c_char_ptr () , name . len () , file_metadata , line_number , size . bits () , align . bits () as u32 , flags , None , empty_array , 0 , vtable_holder , unique_type_id_str . as_c_char_ptr () , unique_type_id_str . len () ,) } } Stub :: Union => unsafe { llvm :: LLVMRustDIBuilderCreateUnionType (DIB (cx) , containing_scope , name . as_c_char_ptr () , name . len () , file_metadata , line_number , size . bits () , align . bits () as u32 , flags , Some (empty_array) , 0 , unique_type_id_str . as_c_char_ptr () , unique_type_id_str . len () ,) } , } ; StubInfo { metadata , unique_type_id } }
}
mkitem!{mkstruct!{struct AdtStackPopGuard < 'll , 'tcx , 'a > { cx : & 'a CodegenCx < 'll , 'tcx > , }}}
mkitem!{mkimpl!{impl < 'll , 'tcx , 'a > Drop for AdtStackPopGuard < 'll , 'tcx , 'a > { fn drop (& mut self) { debug_context (self . cx) . adt_stack . borrow_mut () . pop () ; } }}}

macro_rules! build_type_with_children_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_type_with_children in module {}", module_path!());
    };
}

mkfn!{
    build_type_with_children_introspect!();
    #[doc = " This function enables creating debuginfo nodes that can recursively refer to themselves."] #[doc = " It will first insert the given stub into the type map and only then execute the `members`"] #[doc = " and `generics` closures passed in. These closures have access to the stub so they can"] #[doc = " directly attach fields to them. If the type of a field transitively refers back"] #[doc = " to the type currently being built, the stub will already be found in the type map,"] #[doc = " which effectively breaks the recursion cycle."] pub (super) fn build_type_with_children < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , stub_info : StubInfo < 'll , 'tcx > , members : impl FnOnce (& CodegenCx < 'll , 'tcx > , & 'll DIType) -> SmallVec < & 'll DIType > , generics : impl FnOnce (& CodegenCx < 'll , 'tcx >) -> SmallVec < Option < & 'll DIType > > ,) -> DINodeCreationResult < 'll > { assert_eq ! (debug_context (cx) . type_map . di_node_for_unique_id (stub_info . unique_type_id) , None) ; let mut _adt_stack_pop_guard = None ; if let UniqueTypeId :: Ty (ty , ..) = stub_info . unique_type_id && let ty :: Adt (adt_def , args) = ty . kind () { let def_id = adt_def . did () ; let is_expanding_recursive = { let stack = debug_context (cx) . adt_stack . borrow () ; stack . iter () . enumerate () . rev () . skip (1) . filter (| (_ , (ancestor_def_id , _)) | def_id == * ancestor_def_id) . any (| (ancestor_index , (_ , ancestor_args)) | { args . iter () . zip (ancestor_args . iter ()) . filter_map (| (arg , ancestor_arg) | arg . as_type () . zip (ancestor_arg . as_type ())) . any (| (arg , ancestor_arg) | (arg != ancestor_arg && arg . contains (ancestor_arg)) && stack [ancestor_index + 1 .. stack . len ()] . iter () . all (| (_ , intermediate_args) | intermediate_args . iter () . filter_map (| arg | arg . as_type ()) . any (| mid_arg | mid_arg . contains (ancestor_arg)))) }) } ; if is_expanding_recursive { return DINodeCreationResult :: new (stub_info . metadata , false) ; } else { debug_context (cx) . adt_stack . borrow_mut () . push ((def_id , args)) ; _adt_stack_pop_guard = Some (AdtStackPopGuard { cx }) ; } } debug_context (cx) . type_map . insert (stub_info . unique_type_id , stub_info . metadata) ; let members : SmallVec < _ > = members (cx , stub_info . metadata) . into_iter () . map (| node | Some (node)) . collect () ; let generics = generics (cx) ; if ! (members . is_empty () && generics . is_empty ()) { unsafe { let members_array = create_DIArray (DIB (cx) , & members [..]) ; let generics_array = create_DIArray (DIB (cx) , & generics [..]) ; llvm :: LLVMRustDICompositeTypeReplaceArrays (DIB (cx) , stub_info . metadata , Some (members_array) , Some (generics_array) ,) ; } } DINodeCreationResult { di_node : stub_info . metadata , already_stored_in_typemap : true } }
}