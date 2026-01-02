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
mkuse!{use std :: cell :: RefCell ;}
mkuse!{use std :: hash :: { Hash , Hasher } ;}
mkuse!{use std :: ops :: Range ;}
mkuse!{use std :: str ;}
mkuse!{use rustc_abi :: { FIRST_VARIANT , ReprOptions , VariantIdx } ;}
mkuse!{use rustc_data_structures :: fingerprint :: Fingerprint ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_data_structures :: intern :: Interned ;}
mkuse!{use rustc_data_structures :: stable_hasher :: { HashStable , HashingControls , StableHasher } ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: { CtorKind , DefKind , Res } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: { self as hir , LangItem , find_attr } ;}
mkuse!{use rustc_index :: { IndexSlice , IndexVec } ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use rustc_query_system :: ich :: StableHashingContext ;}
mkuse!{use rustc_session :: DataTypeKind ;}
mkuse!{use rustc_type_ir :: solve :: AdtDestructorKind ;}
mkuse!{use tracing :: { debug , info , trace } ;}
mkuse!{use super :: { AsyncDestructor , Destructor , FieldDef , GenericPredicates , Ty , TyCtxt , VariantDef , VariantDiscr , } ;}
mkuse!{use crate :: mir :: interpret :: ErrorHandled ;}
mkuse!{use crate :: ty ;}
mkuse!{use crate :: ty :: util :: { Discr , IntTypeExt } ;}
mkitem!{mkstruct!{# [derive (Clone , Copy , PartialEq , Eq , Hash , HashStable , TyEncodable , TyDecodable)] pub struct AdtFlags (u16) ;}}
mkitem!{bitflags :: bitflags ! { impl AdtFlags : u16 { const NO_ADT_FLAGS = 0 ; # [doc = " Indicates whether the ADT is an enum."] const IS_ENUM = 1 << 0 ; # [doc = " Indicates whether the ADT is a union."] const IS_UNION = 1 << 1 ; # [doc = " Indicates whether the ADT is a struct."] const IS_STRUCT = 1 << 2 ; # [doc = " Indicates whether the ADT is a struct and has a constructor."] const HAS_CTOR = 1 << 3 ; # [doc = " Indicates whether the type is `PhantomData`."] const IS_PHANTOM_DATA = 1 << 4 ; # [doc = " Indicates whether the type has a `#[fundamental]` attribute."] const IS_FUNDAMENTAL = 1 << 5 ; # [doc = " Indicates whether the type is `Box`."] const IS_BOX = 1 << 6 ; # [doc = " Indicates whether the type is `ManuallyDrop`."] const IS_MANUALLY_DROP = 1 << 7 ; # [doc = " Indicates whether the variant list of this ADT is `#[non_exhaustive]`."] # [doc = " (i.e., this flag is never set unless this ADT is an enum)."] const IS_VARIANT_LIST_NON_EXHAUSTIVE = 1 << 8 ; # [doc = " Indicates whether the type is `UnsafeCell`."] const IS_UNSAFE_CELL = 1 << 9 ; # [doc = " Indicates whether the type is `UnsafePinned`."] const IS_UNSAFE_PINNED = 1 << 10 ; } }}
mkitem!{rustc_data_structures :: external_bitflags_debug ! { AdtFlags }}
mkitem!{mkstruct!{# [doc = " The definition of a user-defined type, e.g., a `struct`, `enum`, or `union`."] # [doc = ""] # [doc = " These are all interned (by `mk_adt_def`) into the global arena."] # [doc = ""] # [doc = " The initialism *ADT* stands for an [*algebraic data type (ADT)*][adt]."] # [doc = " This is slightly wrong because `union`s are not ADTs."] # [doc = " Moreover, Rust only allows recursive data types through indirection."] # [doc = ""] # [doc = " [adt]: https://en.wikipedia.org/wiki/Algebraic_data_type"] # [doc = ""] # [doc = " # Recursive types"] # [doc = ""] # [doc = " It may seem impossible to represent recursive types using [`Ty`],"] # [doc = " since [`TyKind::Adt`] includes [`AdtDef`], which includes its fields,"] # [doc = " creating a cycle. However, `AdtDef` does not actually include the *types*"] # [doc = " of its fields; it includes just their [`DefId`]s."] # [doc = ""] # [doc = " [`TyKind::Adt`]: ty::TyKind::Adt"] # [doc = ""] # [doc = " For example, the following type:"] # [doc = ""] # [doc = " ```"] # [doc = " struct S { x: Box<S> }"] # [doc = " ```"] # [doc = ""] # [doc = " is essentially represented with [`Ty`] as the following pseudocode:"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " struct S { x }"] # [doc = " ```"] # [doc = ""] # [doc = " where `x` here represents the `DefId` of `S.x`. Then, the `DefId`"] # [doc = " can be used with [`TyCtxt::type_of()`] to get the type of the field."] # [derive (TyEncodable , TyDecodable)] pub struct AdtDefData { # [doc = " The `DefId` of the struct, enum or union item."] pub did : DefId , # [doc = " Variants of the ADT. If this is a struct or union, then there will be a single variant."] variants : IndexVec < VariantIdx , VariantDef > , # [doc = " Flags of the ADT (e.g., is this a struct? is this non-exhaustive?)."] flags : AdtFlags , # [doc = " Repr options provided by the user."] repr : ReprOptions , }}}
mkitem!{mkimpl!{impl PartialEq for AdtDefData { # [inline] fn eq (& self , other : & Self) -> bool { let Self { did : self_def_id , variants : _ , flags : _ , repr : _ } = self ; let Self { did : other_def_id , variants : _ , flags : _ , repr : _ } = other ; let res = self_def_id == other_def_id ; if cfg ! (debug_assertions) && res { let deep = self . flags == other . flags && self . repr == other . repr && self . variants == other . variants ; assert ! (deep , "AdtDefData for the same def-id has differing data") ; } res } }}}
mkitem!{mkimpl!{impl Eq for AdtDefData { }}}
mkitem!{mkimpl!{# [doc = " There should be only one AdtDef for each `did`, therefore"] # [doc = " it is fine to implement `Hash` only based on `did`."] impl Hash for AdtDefData { # [inline] fn hash < H : Hasher > (& self , s : & mut H) { self . did . hash (s) } }}}
mkitem!{mkimpl!{impl < 'a > HashStable < StableHashingContext < 'a > > for AdtDefData { fn hash_stable (& self , hcx : & mut StableHashingContext < 'a > , hasher : & mut StableHasher) { thread_local ! { static CACHE : RefCell < FxHashMap < (usize , HashingControls) , Fingerprint >> = Default :: default () ; } let hash : Fingerprint = CACHE . with (| cache | { let addr = self as * const AdtDefData as usize ; let hashing_controls = hcx . hashing_controls () ; * cache . borrow_mut () . entry ((addr , hashing_controls)) . or_insert_with (| | { let ty :: AdtDefData { did , ref variants , ref flags , ref repr } = * self ; let mut hasher = StableHasher :: new () ; did . hash_stable (hcx , & mut hasher) ; variants . hash_stable (hcx , & mut hasher) ; flags . hash_stable (hcx , & mut hasher) ; repr . hash_stable (hcx , & mut hasher) ; hasher . finish () }) }) ; hash . hash_stable (hcx , hasher) ; } }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , PartialEq , Eq , Hash , HashStable)] # [rustc_pass_by_value] pub struct AdtDef < 'tcx > (pub Interned < 'tcx , AdtDefData >) ;}}
mkitem!{mkimpl!{impl < 'tcx > AdtDef < 'tcx > { # [inline] pub fn did (self) -> DefId { self . 0 . 0 . did } # [inline] pub fn variants (self) -> & 'tcx IndexSlice < VariantIdx , VariantDef > { & self . 0 . 0 . variants } # [inline] pub fn variant (self , idx : VariantIdx) -> & 'tcx VariantDef { & self . 0 . 0 . variants [idx] } # [inline] pub fn flags (self) -> AdtFlags { self . 0 . 0 . flags } # [inline] pub fn repr (self) -> ReprOptions { self . 0 . 0 . repr } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: AdtDef < TyCtxt < 'tcx > > for AdtDef < 'tcx > { fn def_id (self) -> DefId { self . did () } fn is_struct (self) -> bool { self . is_struct () } fn struct_tail_ty (self , interner : TyCtxt < 'tcx >) -> Option < ty :: EarlyBinder < 'tcx , Ty < 'tcx > > > { Some (interner . type_of (self . non_enum_variant () . tail_opt () ? . did)) } fn is_phantom_data (self) -> bool { self . is_phantom_data () } fn is_manually_drop (self) -> bool { self . is_manually_drop () } fn all_field_tys (self , tcx : TyCtxt < 'tcx > ,) -> ty :: EarlyBinder < 'tcx , impl IntoIterator < Item = Ty < 'tcx > > > { ty :: EarlyBinder :: bind (self . all_fields () . map (move | field | tcx . type_of (field . did) . skip_binder ()) ,) } fn sizedness_constraint (self , tcx : TyCtxt < 'tcx > , sizedness : ty :: SizedTraitKind ,) -> Option < ty :: EarlyBinder < 'tcx , Ty < 'tcx > > > { self . sizedness_constraint (tcx , sizedness) } fn is_fundamental (self) -> bool { self . is_fundamental () } fn destructor (self , tcx : TyCtxt < 'tcx >) -> Option < AdtDestructorKind > { Some (match tcx . constness (self . destructor (tcx) ? . did) { hir :: Constness :: Const => AdtDestructorKind :: Const , hir :: Constness :: NotConst => AdtDestructorKind :: NotConst , }) } }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , Eq , PartialEq , HashStable , TyEncodable , TyDecodable)] pub enum AdtKind { Struct , Union , Enum , }}}
mkitem!{mkimpl!{impl From < AdtKind > for DataTypeKind { fn from (val : AdtKind) -> Self { match val { AdtKind :: Struct => DataTypeKind :: Struct , AdtKind :: Union => DataTypeKind :: Union , AdtKind :: Enum => DataTypeKind :: Enum , } } }}}
mkitem!{mkimpl!{impl AdtDefData { # [doc = " Creates a new `AdtDefData`."] pub (super) fn new (tcx : TyCtxt < '_ > , did : DefId , kind : AdtKind , variants : IndexVec < VariantIdx , VariantDef > , repr : ReprOptions ,) -> Self { debug ! ("AdtDef::new({:?}, {:?}, {:?}, {:?})" , did , kind , variants , repr) ; let mut flags = AdtFlags :: NO_ADT_FLAGS ; if kind == AdtKind :: Enum && find_attr ! (tcx . get_all_attrs (did) , AttributeKind :: NonExhaustive (..)) { debug ! ("found non-exhaustive variant list for {:?}" , did) ; flags = flags | AdtFlags :: IS_VARIANT_LIST_NON_EXHAUSTIVE ; } flags |= match kind { AdtKind :: Enum => AdtFlags :: IS_ENUM , AdtKind :: Union => AdtFlags :: IS_UNION , AdtKind :: Struct => AdtFlags :: IS_STRUCT , } ; if kind == AdtKind :: Struct && variants [FIRST_VARIANT] . ctor . is_some () { flags |= AdtFlags :: HAS_CTOR ; } if find_attr ! (tcx . get_all_attrs (did) , AttributeKind :: Fundamental) { flags |= AdtFlags :: IS_FUNDAMENTAL ; } if tcx . is_lang_item (did , LangItem :: PhantomData) { flags |= AdtFlags :: IS_PHANTOM_DATA ; } if tcx . is_lang_item (did , LangItem :: OwnedBox) { flags |= AdtFlags :: IS_BOX ; } if tcx . is_lang_item (did , LangItem :: ManuallyDrop) { flags |= AdtFlags :: IS_MANUALLY_DROP ; } if tcx . is_lang_item (did , LangItem :: UnsafeCell) { flags |= AdtFlags :: IS_UNSAFE_CELL ; } if tcx . is_lang_item (did , LangItem :: UnsafePinned) { flags |= AdtFlags :: IS_UNSAFE_PINNED ; } AdtDefData { did , variants , flags , repr } } }}}
mkitem!{mkimpl!{impl < 'tcx > AdtDef < 'tcx > { # [doc = " Returns `true` if this is a struct."] # [inline] pub fn is_struct (self) -> bool { self . flags () . contains (AdtFlags :: IS_STRUCT) } # [doc = " Returns `true` if this is a union."] # [inline] pub fn is_union (self) -> bool { self . flags () . contains (AdtFlags :: IS_UNION) } # [doc = " Returns `true` if this is an enum."] # [inline] pub fn is_enum (self) -> bool { self . flags () . contains (AdtFlags :: IS_ENUM) } # [doc = " Returns `true` if the variant list of this ADT is `#[non_exhaustive]`."] # [doc = ""] # [doc = " Note that this function will return `true` even if the ADT has been"] # [doc = " defined in the crate currently being compiled. If that's not what you"] # [doc = " want, see [`Self::variant_list_has_applicable_non_exhaustive`]."] # [inline] pub fn is_variant_list_non_exhaustive (self) -> bool { self . flags () . contains (AdtFlags :: IS_VARIANT_LIST_NON_EXHAUSTIVE) } # [doc = " Returns `true` if the variant list of this ADT is `#[non_exhaustive]`"] # [doc = " and has been defined in another crate."] # [inline] pub fn variant_list_has_applicable_non_exhaustive (self) -> bool { self . is_variant_list_non_exhaustive () && ! self . did () . is_local () } # [doc = " Returns the kind of the ADT."] # [inline] pub fn adt_kind (self) -> AdtKind { if self . is_enum () { AdtKind :: Enum } else if self . is_union () { AdtKind :: Union } else { AdtKind :: Struct } } # [doc = " Returns a description of this abstract data type."] pub fn descr (self) -> & 'static str { match self . adt_kind () { AdtKind :: Struct => "struct" , AdtKind :: Union => "union" , AdtKind :: Enum => "enum" , } } # [doc = " Returns a description of a variant of this abstract data type."] # [inline] pub fn variant_descr (self) -> & 'static str { match self . adt_kind () { AdtKind :: Struct => "struct" , AdtKind :: Union => "union" , AdtKind :: Enum => "variant" , } } # [doc = " If this function returns `true`, it implies that `is_struct` must return `true`."] # [inline] pub fn has_ctor (self) -> bool { self . flags () . contains (AdtFlags :: HAS_CTOR) } # [doc = " Returns `true` if this type is `#[fundamental]` for the purposes"] # [doc = " of coherence checking."] # [inline] pub fn is_fundamental (self) -> bool { self . flags () . contains (AdtFlags :: IS_FUNDAMENTAL) } # [doc = " Returns `true` if this is `PhantomData<T>`."] # [inline] pub fn is_phantom_data (self) -> bool { self . flags () . contains (AdtFlags :: IS_PHANTOM_DATA) } # [doc = " Returns `true` if this is `Box<T>`."] # [inline] pub fn is_box (self) -> bool { self . flags () . contains (AdtFlags :: IS_BOX) } # [doc = " Returns `true` if this is `UnsafeCell<T>`."] # [inline] pub fn is_unsafe_cell (self) -> bool { self . flags () . contains (AdtFlags :: IS_UNSAFE_CELL) } # [doc = " Returns `true` if this is `UnsafePinned<T>`."] # [inline] pub fn is_unsafe_pinned (self) -> bool { self . flags () . contains (AdtFlags :: IS_UNSAFE_PINNED) } # [doc = " Returns `true` if this is `ManuallyDrop<T>`."] # [inline] pub fn is_manually_drop (self) -> bool { self . flags () . contains (AdtFlags :: IS_MANUALLY_DROP) } # [doc = " Returns `true` if this type has a destructor."] pub fn has_dtor (self , tcx : TyCtxt < 'tcx >) -> bool { self . destructor (tcx) . is_some () } # [doc = " Asserts this is a struct or union and returns its unique variant."] pub fn non_enum_variant (self) -> & 'tcx VariantDef { assert ! (self . is_struct () || self . is_union ()) ; self . variant (FIRST_VARIANT) } # [inline] pub fn predicates (self , tcx : TyCtxt < 'tcx >) -> GenericPredicates < 'tcx > { tcx . predicates_of (self . did ()) } # [doc = " Returns an iterator over all fields contained"] # [doc = " by this ADT (nested unnamed fields are not expanded)."] # [inline] pub fn all_fields (self) -> impl Iterator < Item = & 'tcx FieldDef > + Clone { self . variants () . iter () . flat_map (| v | v . fields . iter ()) } # [doc = " Whether the ADT lacks fields. Note that this includes uninhabited enums,"] # [doc = " e.g., `enum Void {}` is considered payload free as well."] pub fn is_payloadfree (self) -> bool { if self . variants () . iter () . any (| v | { matches ! (v . discr , VariantDiscr :: Explicit (_)) && v . ctor_kind () != Some (CtorKind :: Const) }) { return false ; } self . variants () . iter () . all (| v | v . fields . is_empty ()) } # [doc = " Return a `VariantDef` given a variant id."] pub fn variant_with_id (self , vid : DefId) -> & 'tcx VariantDef { self . variants () . iter () . find (| v | v . def_id == vid) . expect ("variant_with_id: unknown variant") } # [doc = " Return a `VariantDef` given a constructor id."] pub fn variant_with_ctor_id (self , cid : DefId) -> & 'tcx VariantDef { self . variants () . iter () . find (| v | v . ctor_def_id () == Some (cid)) . expect ("variant_with_ctor_id: unknown variant") } # [doc = " Return the index of `VariantDef` given a variant id."] # [inline] pub fn variant_index_with_id (self , vid : DefId) -> VariantIdx { self . variants () . iter_enumerated () . find (| (_ , v) | v . def_id == vid) . expect ("variant_index_with_id: unknown variant") . 0 } # [doc = " Return the index of `VariantDef` given a constructor id."] pub fn variant_index_with_ctor_id (self , cid : DefId) -> VariantIdx { self . variants () . iter_enumerated () . find (| (_ , v) | v . ctor_def_id () == Some (cid)) . expect ("variant_index_with_ctor_id: unknown variant") . 0 } pub fn variant_of_res (self , res : Res) -> & 'tcx VariantDef { match res { Res :: Def (DefKind :: Variant , vid) => self . variant_with_id (vid) , Res :: Def (DefKind :: Ctor (..) , cid) => self . variant_with_ctor_id (cid) , Res :: Def (DefKind :: Struct , _) | Res :: Def (DefKind :: Union , _) | Res :: Def (DefKind :: TyAlias , _) | Res :: Def (DefKind :: AssocTy , _) | Res :: SelfTyParam { .. } | Res :: SelfTyAlias { .. } | Res :: SelfCtor (..) => self . non_enum_variant () , _ => bug ! ("unexpected res {:?} in variant_of_res" , res) , } } # [inline] pub fn eval_explicit_discr (self , tcx : TyCtxt < 'tcx > , expr_did : DefId ,) -> Result < Discr < 'tcx > , ErrorGuaranteed > { assert ! (self . is_enum ()) ; let repr_type = self . repr () . discr_type () ; match tcx . const_eval_poly (expr_did) { Ok (val) => { let typing_env = ty :: TypingEnv :: post_analysis (tcx , expr_did) ; let ty = repr_type . to_ty (tcx) ; if let Some (b) = val . try_to_bits_for_ty (tcx , typing_env , ty) { trace ! ("discriminants: {} ({:?})" , b , repr_type) ; Ok (Discr { val : b , ty }) } else { info ! ("invalid enum discriminant: {:#?}" , val) ; let guar = tcx . dcx () . emit_err (crate :: error :: ConstEvalNonIntError { span : tcx . def_span (expr_did) , }) ; Err (guar) } } Err (err) => { let guar = match err { ErrorHandled :: Reported (info , _) => info . into () , ErrorHandled :: TooGeneric (..) => tcx . dcx () . span_delayed_bug (tcx . def_span (expr_did) , "enum discriminant depends on generics" ,) , } ; Err (guar) } } } # [inline] pub fn discriminants (self , tcx : TyCtxt < 'tcx > ,) -> impl Iterator < Item = (VariantIdx , Discr < 'tcx >) > { assert ! (self . is_enum ()) ; let repr_type = self . repr () . discr_type () ; let initial = repr_type . initial_discriminant (tcx) ; let mut prev_discr = None :: < Discr < 'tcx > > ; self . variants () . iter_enumerated () . map (move | (i , v) | { let mut discr = prev_discr . map_or (initial , | d | d . wrap_incr (tcx)) ; if let VariantDiscr :: Explicit (expr_did) = v . discr && let Ok (new_discr) = self . eval_explicit_discr (tcx , expr_did) { discr = new_discr ; } prev_discr = Some (discr) ; (i , discr) }) } # [inline] pub fn variant_range (self) -> Range < VariantIdx > { FIRST_VARIANT .. self . variants () . next_index () } # [doc = " Computes the discriminant value used by a specific variant."] # [doc = " Unlike `discriminants`, this is (amortized) constant-time,"] # [doc = " only doing at most one query for evaluating an explicit"] # [doc = " discriminant (the last one before the requested variant),"] # [doc = " assuming there are no constant-evaluation errors there."] # [inline] pub fn discriminant_for_variant (self , tcx : TyCtxt < 'tcx > , variant_index : VariantIdx ,) -> Discr < 'tcx > { assert ! (self . is_enum ()) ; let (val , offset) = self . discriminant_def_for_variant (variant_index) ; let explicit_value = if let Some (expr_did) = val && let Ok (val) = self . eval_explicit_discr (tcx , expr_did) { val } else { self . repr () . discr_type () . initial_discriminant (tcx) } ; explicit_value . checked_add (tcx , offset as u128) . 0 } # [doc = " Yields a `DefId` for the discriminant and an offset to add to it"] # [doc = " Alternatively, if there is no explicit discriminant, returns the"] # [doc = " inferred discriminant directly."] pub fn discriminant_def_for_variant (self , variant_index : VariantIdx) -> (Option < DefId > , u32) { assert ! (! self . variants () . is_empty ()) ; let mut explicit_index = variant_index . as_u32 () ; let expr_did ; loop { match self . variant (VariantIdx :: from_u32 (explicit_index)) . discr { ty :: VariantDiscr :: Relative (0) => { expr_did = None ; break ; } ty :: VariantDiscr :: Relative (distance) => { explicit_index -= distance ; } ty :: VariantDiscr :: Explicit (did) => { expr_did = Some (did) ; break ; } } } (expr_did , variant_index . as_u32 () - explicit_index) } pub fn destructor (self , tcx : TyCtxt < 'tcx >) -> Option < Destructor > { tcx . adt_destructor (self . did ()) } pub fn async_destructor (self , tcx : TyCtxt < 'tcx >) -> Option < AsyncDestructor > { tcx . adt_async_destructor (self . did ()) } # [doc = " If this ADT is a struct, returns a type such that `Self: {Meta,Pointee,}Sized` if and only"] # [doc = " if that type is `{Meta,Pointee,}Sized`, or `None` if this ADT is always"] # [doc = " `{Meta,Pointee,}Sized`."] pub fn sizedness_constraint (self , tcx : TyCtxt < 'tcx > , sizedness : ty :: SizedTraitKind ,) -> Option < ty :: EarlyBinder < 'tcx , Ty < 'tcx > > > { if self . is_struct () { tcx . adt_sizedness_constraint ((self . did () , sizedness)) } else { None } } }}}
mkitem!{mkenum!{# [derive (Clone , Copy , Debug , HashStable)] pub enum Representability { Representable , Infinite (ErrorGuaranteed) , }}}