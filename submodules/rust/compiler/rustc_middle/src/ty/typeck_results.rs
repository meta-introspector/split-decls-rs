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
mkuse!{use std :: collections :: hash_map :: Entry ;}
mkuse!{use std :: hash :: Hash ;}
mkuse!{use std :: iter ;}
mkuse!{use rustc_abi :: { FieldIdx , VariantIdx } ;}
mkuse!{use rustc_data_structures :: fx :: { FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_data_structures :: unord :: { ExtendUnord , UnordItems , UnordSet } ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId , LocalDefIdMap } ;}
mkuse!{use rustc_hir :: hir_id :: OwnerId ;}
mkuse!{use rustc_hir :: { self as hir , BindingMode , ByRef , HirId , ItemLocalId , ItemLocalMap , ItemLocalSet , Mutability , } ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable , TypeFoldable , TypeVisitable } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use super :: RvalueScopes ;}
mkuse!{use crate :: hir :: place :: Place as HirPlace ;}
mkuse!{use crate :: infer :: canonical :: Canonical ;}
mkuse!{use crate :: mir :: FakeReadCause ;}
mkuse!{use crate :: traits :: ObligationCause ;}
mkuse!{use crate :: ty :: { self , BoundVar , CanonicalPolyFnSig , ClosureSizeProfileData , GenericArgKind , GenericArgs , GenericArgsRef , Ty , UserArgs , tls , } ;}
mkitem!{mkstruct!{#[derive (TyEncodable , TyDecodable , Debug , HashStable)] pub struct TypeckResults < 'tcx > { #[doc = " The `HirId::owner` all `ItemLocalId`s in this table are relative to."] pub hir_owner : OwnerId , #[doc = " Resolved definitions for `<T>::X` associated paths and"] #[doc = " method calls, including those of overloaded operators."] type_dependent_defs : ItemLocalMap < Result < (DefKind , DefId) , ErrorGuaranteed > > , #[doc = " Resolved field indices for field accesses in expressions (`S { field }`, `obj.field`)"] #[doc = " or patterns (`S { field }`). The index is often useful by itself, but to learn more"] #[doc = " about the field you also need definition of the variant to which the field"] #[doc = " belongs, but it may not exist if it's a tuple field (`tuple.0`)."] field_indices : ItemLocalMap < FieldIdx > , #[doc = " Stores the types for various nodes in the AST. Note that this table"] #[doc = " is not guaranteed to be populated outside inference. See"] #[doc = " typeck::check::fn_ctxt for details."] node_types : ItemLocalMap < Ty < 'tcx > > , #[doc = " Stores the type parameters which were instantiated to obtain the type"] #[doc = " of this node. This only applies to nodes that refer to entities"] #[doc = " parameterized by type parameters, such as generic fns, types, or"] #[doc = " other items."] node_args : ItemLocalMap < GenericArgsRef < 'tcx > > , #[doc = " This will either store the canonicalized types provided by the user"] #[doc = " or the generic parameters that the user explicitly gave (if any) attached"] #[doc = " to `id`. These will not include any inferred values. The canonical form"] #[doc = " is used to capture things like `_` or other unspecified values."] #[doc = ""] #[doc = " For example, if the user wrote `foo.collect::<Vec<_>>()`, then the"] #[doc = " canonical generic parameters would include only `for<X> { Vec<X> }`."] #[doc = ""] #[doc = " See also `AscribeUserType` statement in MIR."] user_provided_types : ItemLocalMap < CanonicalUserType < 'tcx > > , #[doc = " Stores the canonicalized types provided by the user. See also"] #[doc = " `AscribeUserType` statement in MIR."] pub user_provided_sigs : LocalDefIdMap < CanonicalPolyFnSig < 'tcx > > , adjustments : ItemLocalMap < Vec < ty :: adjustment :: Adjustment < 'tcx > > > , #[doc = " Stores the actual binding mode for all instances of [`BindingMode`]."] pat_binding_modes : ItemLocalMap < BindingMode > , #[doc = " Top-level patterns incompatible with Rust 2024's match ergonomics. These will be translated"] #[doc = " to a form valid in all Editions, either as a lint diagnostic or hard error."] rust_2024_migration_desugared_pats : ItemLocalMap < Rust2024IncompatiblePatInfo > , #[doc = " Stores the types which were implicitly dereferenced in pattern binding modes or deref"] #[doc = " patterns for later usage in THIR lowering. For example,"] #[doc = ""] #[doc = " ```"] #[doc = " match &&Some(5i32) {"] #[doc = "     Some(n) => {},"] #[doc = "     _ => {},"] #[doc = " }"] #[doc = " ```"] #[doc = " leads to a `vec![&&Option<i32>, &Option<i32>]` and"] #[doc = ""] #[doc = " ```"] #[doc = " #![feature(deref_patterns)]"] #[doc = " match &Box::new(Some(5i32)) {"] #[doc = "     Some(n) => {},"] #[doc = "     _ => {},"] #[doc = " }"] #[doc = " ```"] #[doc = " leads to a `vec![&Box<Option<i32>>, Box<Option<i32>>]`. Empty vectors are not stored."] #[doc = ""] #[doc = " See:"] #[doc = " <https://github.com/rust-lang/rfcs/blob/master/text/2005-match-ergonomics.md#definitions>"] pat_adjustments : ItemLocalMap < Vec < ty :: adjustment :: PatAdjustment < 'tcx > > > , #[doc = " Set of reference patterns that match against a match-ergonomics inserted reference"] #[doc = " (as opposed to against a reference in the scrutinee type)."] skipped_ref_pats : ItemLocalSet , #[doc = " Records the reasons that we picked the kind of each closure;"] #[doc = " not all closures are present in the map."] closure_kind_origins : ItemLocalMap < (Span , HirPlace < 'tcx >) > , #[doc = " For each fn, records the \"liberated\" types of its arguments"] #[doc = " and return type. Liberated means that all bound regions"] #[doc = " (including late-bound regions) are replaced with free"] #[doc = " equivalents. This table is not used in codegen (since regions"] #[doc = " are erased there) and hence is not serialized to metadata."] #[doc = ""] #[doc = " This table also contains the \"revealed\" values for any `impl Trait`"] #[doc = " that appear in the signature and whose values are being inferred"] #[doc = " by this function."] #[doc = ""] #[doc = " # Example"] #[doc = ""] #[doc = " ```rust"] #[doc = " # use std::fmt::Debug;"] #[doc = " fn foo(x: &u32) -> impl Debug { *x }"] #[doc = " ```"] #[doc = ""] #[doc = " The function signature here would be:"] #[doc = ""] #[doc = " ```ignore (illustrative)"] #[doc = " for<'a> fn(&'a u32) -> Foo"] #[doc = " ```"] #[doc = ""] #[doc = " where `Foo` is an opaque type created for this function."] #[doc = ""] #[doc = ""] #[doc = " The *liberated* form of this would be"] #[doc = ""] #[doc = " ```ignore (illustrative)"] #[doc = " fn(&'a u32) -> u32"] #[doc = " ```"] #[doc = ""] #[doc = " Note that `'a` is not bound (it would be an `ReLateParam`) and"] #[doc = " that the `Foo` opaque type is replaced by its hidden type."] liberated_fn_sigs : ItemLocalMap < ty :: FnSig < 'tcx > > , #[doc = " For each FRU expression, record the normalized types of the fields"] #[doc = " of the struct - this is needed because it is non-trivial to"] #[doc = " normalize while preserving regions. This table is used only in"] #[doc = " MIR construction and hence is not serialized to metadata."] fru_field_types : ItemLocalMap < Vec < Ty < 'tcx > > > , #[doc = " For every coercion cast we add the HIR node ID of the cast"] #[doc = " expression to this set."] coercion_casts : ItemLocalSet , #[doc = " Set of trait imports actually used in the method resolution."] #[doc = " This is used for warning unused imports."] pub used_trait_imports : UnordSet < LocalDefId > , #[doc = " If any errors occurred while type-checking this body,"] #[doc = " this field will be set to `Some(ErrorGuaranteed)`."] pub tainted_by_errors : Option < ErrorGuaranteed > , #[doc = " All the opaque types that have hidden types set by this function."] #[doc = " We also store the type here, so that the compiler can use it as a hint"] #[doc = " for figuring out hidden types, even if they are only set in dead code"] #[doc = " (which doesn't show up in MIR)."] pub concrete_opaque_types : FxIndexMap < LocalDefId , ty :: OpaqueHiddenType < 'tcx > > , #[doc = " Tracks the minimum captures required for a closure;"] #[doc = " see `MinCaptureInformationMap` for more details."] pub closure_min_captures : ty :: MinCaptureInformationMap < 'tcx > , #[doc = " Tracks the fake reads required for a closure and the reason for the fake read."] #[doc = " When performing pattern matching for closures, there are times we don't end up"] #[doc = " reading places that are mentioned in a closure (because of _ patterns). However,"] #[doc = " to ensure the places are initialized, we introduce fake reads."] #[doc = " Consider these two examples:"] #[doc = " ```ignore (discriminant matching with only wildcard arm)"] #[doc = " let x: u8;"] #[doc = " let c = || match x { _ => () };"] #[doc = " ```"] #[doc = " In this example, we don't need to actually read/borrow `x` in `c`, and so we don't"] #[doc = " want to capture it. However, we do still want an error here, because `x` should have"] #[doc = " to be initialized at the point where c is created. Therefore, we add a \"fake read\""] #[doc = " instead."] #[doc = " ```ignore (destructured assignments)"] #[doc = " let c = || {"] #[doc = "     let (t1, t2) = t;"] #[doc = " }"] #[doc = " ```"] #[doc = " In the second example, we capture the disjoint fields of `t` (`t.0` & `t.1`), but"] #[doc = " we never capture `t`. This becomes an issue when we build MIR as we require"] #[doc = " information on `t` in order to create place `t.0` and `t.1`. We can solve this"] #[doc = " issue by fake reading `t`."] pub closure_fake_reads : LocalDefIdMap < Vec < (HirPlace < 'tcx > , FakeReadCause , HirId) > > , #[doc = " Tracks the rvalue scoping rules which defines finer scoping for rvalue expressions"] #[doc = " by applying extended parameter rules."] #[doc = " Details may be found in `rustc_hir_analysis::check::rvalue_scopes`."] pub rvalue_scopes : RvalueScopes , #[doc = " Stores the predicates that apply on coroutine witness types."] #[doc = " formatting modified file tests/ui/coroutine/retain-resume-ref.rs"] pub coroutine_stalled_predicates : FxIndexSet < (ty :: Predicate < 'tcx > , ObligationCause < 'tcx >) > , #[doc = " Goals proven during HIR typeck which may be potentially region dependent."] #[doc = ""] #[doc = " Borrowck *uniquifies* regions which may cause these goal to be ambiguous in MIR"] #[doc = " type check. We ICE if goals fail in borrowck to detect bugs during MIR building or"] #[doc = " missed checks in HIR typeck. To avoid ICE due to region dependence we store all"] #[doc = " goals which may be region dependent and reprove them in case borrowck encounters"] #[doc = " an error."] pub potentially_region_dependent_goals : FxIndexSet < (ty :: Predicate < 'tcx > , ObligationCause < 'tcx >) > , #[doc = " Contains the data for evaluating the effect of feature `capture_disjoint_fields`"] #[doc = " on closure size."] pub closure_size_eval : LocalDefIdMap < ClosureSizeProfileData < 'tcx > > , #[doc = " Stores the types involved in calls to `transmute` intrinsic. These are meant to be checked"] #[doc = " outside of typeck and borrowck to avoid cycles with opaque types and coroutine layout"] #[doc = " computation."] pub transmutes_to_check : Vec < (Ty < 'tcx > , Ty < 'tcx > , HirId) > , #[doc = " Container types and field indices of `offset_of!` expressions"] offset_of_data : ItemLocalMap < (Ty < 'tcx > , Vec < (VariantIdx , FieldIdx) >) > , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeckResults < 'tcx > { pub fn new (hir_owner : OwnerId) -> TypeckResults < 'tcx > { TypeckResults { hir_owner , type_dependent_defs : Default :: default () , field_indices : Default :: default () , user_provided_types : Default :: default () , user_provided_sigs : Default :: default () , node_types : Default :: default () , node_args : Default :: default () , adjustments : Default :: default () , pat_binding_modes : Default :: default () , pat_adjustments : Default :: default () , rust_2024_migration_desugared_pats : Default :: default () , skipped_ref_pats : Default :: default () , closure_kind_origins : Default :: default () , liberated_fn_sigs : Default :: default () , fru_field_types : Default :: default () , coercion_casts : Default :: default () , used_trait_imports : Default :: default () , tainted_by_errors : None , concrete_opaque_types : Default :: default () , closure_min_captures : Default :: default () , closure_fake_reads : Default :: default () , rvalue_scopes : Default :: default () , coroutine_stalled_predicates : Default :: default () , potentially_region_dependent_goals : Default :: default () , closure_size_eval : Default :: default () , transmutes_to_check : Default :: default () , offset_of_data : Default :: default () , } } #[doc = " Returns the final resolution of a `QPath` in an `Expr` or `Pat` node."] pub fn qpath_res (& self , qpath : & hir :: QPath < '_ > , id : HirId) -> Res { match * qpath { hir :: QPath :: Resolved (_ , path) => path . res , hir :: QPath :: TypeRelative (..) | hir :: QPath :: LangItem (..) => self . type_dependent_def (id) . map_or (Res :: Err , | (kind , def_id) | Res :: Def (kind , def_id)) , } } pub fn type_dependent_defs (& self ,) -> LocalTableInContext < '_ , Result < (DefKind , DefId) , ErrorGuaranteed > > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . type_dependent_defs } } pub fn type_dependent_def (& self , id : HirId) -> Option < (DefKind , DefId) > { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . type_dependent_defs . get (& id . local_id) . cloned () . and_then (| r | r . ok ()) } pub fn type_dependent_def_id (& self , id : HirId) -> Option < DefId > { self . type_dependent_def (id) . map (| (_ , def_id) | def_id) } pub fn type_dependent_defs_mut (& mut self ,) -> LocalTableInContextMut < '_ , Result < (DefKind , DefId) , ErrorGuaranteed > > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . type_dependent_defs } } pub fn field_indices (& self) -> LocalTableInContext < '_ , FieldIdx > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . field_indices } } pub fn field_indices_mut (& mut self) -> LocalTableInContextMut < '_ , FieldIdx > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . field_indices } } pub fn field_index (& self , id : HirId) -> FieldIdx { self . field_indices () . get (id) . cloned () . expect ("no index for a field") } pub fn opt_field_index (& self , id : HirId) -> Option < FieldIdx > { self . field_indices () . get (id) . cloned () } pub fn user_provided_types (& self) -> LocalTableInContext < '_ , CanonicalUserType < 'tcx > > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . user_provided_types } } pub fn user_provided_types_mut (& mut self ,) -> LocalTableInContextMut < '_ , CanonicalUserType < 'tcx > > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . user_provided_types } } pub fn node_types (& self) -> LocalTableInContext < '_ , Ty < 'tcx > > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . node_types } } pub fn node_types_mut (& mut self) -> LocalTableInContextMut < '_ , Ty < 'tcx > > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . node_types } } pub fn node_type (& self , id : HirId) -> Ty < 'tcx > { self . node_type_opt (id) . unwrap_or_else (| | { bug ! ("node_type: no type for node {}" , tls :: with (| tcx | tcx . hir_id_to_string (id))) }) } pub fn node_type_opt (& self , id : HirId) -> Option < Ty < 'tcx > > { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . node_types . get (& id . local_id) . cloned () } pub fn node_args_mut (& mut self) -> LocalTableInContextMut < '_ , GenericArgsRef < 'tcx > > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . node_args } } pub fn node_args (& self , id : HirId) -> GenericArgsRef < 'tcx > { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . node_args . get (& id . local_id) . cloned () . unwrap_or_else (| | GenericArgs :: empty ()) } pub fn node_args_opt (& self , id : HirId) -> Option < GenericArgsRef < 'tcx > > { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . node_args . get (& id . local_id) . cloned () } #[doc = " Returns the type of a pattern as a monotype. Like [`expr_ty`], this function"] #[doc = " doesn't provide type parameter args."] #[doc = ""] #[doc = " [`expr_ty`]: TypeckResults::expr_ty"] pub fn pat_ty (& self , pat : & hir :: Pat < '_ >) -> Ty < 'tcx > { self . node_type (pat . hir_id) } #[doc = " Returns the type of an expression as a monotype."] #[doc = ""] #[doc = " NB (1): This is the PRE-ADJUSTMENT TYPE for the expression. That is, in"] #[doc = " some cases, we insert `Adjustment` annotations such as auto-deref or"] #[doc = " auto-ref. The type returned by this function does not consider such"] #[doc = " adjustments. See [`Self::expr_ty_adjusted`] instead."] #[doc = ""] #[doc = " NB (2): This type doesn't provide type parameter args; e.g., if you"] #[doc = " ask for the type of `id` in `id(3)`, it will return `fn(&isize) -> isize`"] #[doc = " instead of `fn(ty) -> T with T = isize`."] pub fn expr_ty (& self , expr : & hir :: Expr < '_ >) -> Ty < 'tcx > { self . node_type (expr . hir_id) } pub fn expr_ty_opt (& self , expr : & hir :: Expr < '_ >) -> Option < Ty < 'tcx > > { self . node_type_opt (expr . hir_id) } pub fn adjustments (& self) -> LocalTableInContext < '_ , Vec < ty :: adjustment :: Adjustment < 'tcx > > > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . adjustments } } pub fn adjustments_mut (& mut self ,) -> LocalTableInContextMut < '_ , Vec < ty :: adjustment :: Adjustment < 'tcx > > > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . adjustments } } pub fn expr_adjustments (& self , expr : & hir :: Expr < '_ >) -> & [ty :: adjustment :: Adjustment < 'tcx >] { validate_hir_id_for_typeck_results (self . hir_owner , expr . hir_id) ; self . adjustments . get (& expr . hir_id . local_id) . map_or (& [] , | a | & a [..]) } #[doc = " Returns the type of `expr`, considering any `Adjustment`"] #[doc = " entry recorded for that expression."] pub fn expr_ty_adjusted (& self , expr : & hir :: Expr < '_ >) -> Ty < 'tcx > { self . expr_adjustments (expr) . last () . map_or_else (| | self . expr_ty (expr) , | adj | adj . target) } pub fn expr_ty_adjusted_opt (& self , expr : & hir :: Expr < '_ >) -> Option < Ty < 'tcx > > { self . expr_adjustments (expr) . last () . map (| adj | adj . target) . or_else (| | self . expr_ty_opt (expr)) } pub fn is_method_call (& self , expr : & hir :: Expr < '_ >) -> bool { if let hir :: ExprKind :: Path (_) = expr . kind { return false ; } matches ! (self . type_dependent_defs () . get (expr . hir_id) , Some (Ok ((DefKind :: AssocFn , _)))) } #[doc = " Returns the computed binding mode for a `PatKind::Binding` pattern"] #[doc = " (after match ergonomics adjustments)."] pub fn extract_binding_mode (& self , s : & Session , id : HirId , sp : Span) -> BindingMode { self . pat_binding_modes () . get (id) . copied () . unwrap_or_else (| | { s . dcx () . span_bug (sp , "missing binding mode") ; }) } pub fn pat_binding_modes (& self) -> LocalTableInContext < '_ , BindingMode > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . pat_binding_modes } } pub fn pat_binding_modes_mut (& mut self) -> LocalTableInContextMut < '_ , BindingMode > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . pat_binding_modes } } pub fn pat_adjustments (& self ,) -> LocalTableInContext < '_ , Vec < ty :: adjustment :: PatAdjustment < 'tcx > > > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . pat_adjustments } } pub fn pat_adjustments_mut (& mut self ,) -> LocalTableInContextMut < '_ , Vec < ty :: adjustment :: PatAdjustment < 'tcx > > > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . pat_adjustments } } pub fn rust_2024_migration_desugared_pats (& self ,) -> LocalTableInContext < '_ , Rust2024IncompatiblePatInfo > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . rust_2024_migration_desugared_pats , } } pub fn rust_2024_migration_desugared_pats_mut (& mut self ,) -> LocalTableInContextMut < '_ , Rust2024IncompatiblePatInfo > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . rust_2024_migration_desugared_pats , } } pub fn skipped_ref_pats (& self) -> LocalSetInContext < '_ > { LocalSetInContext { hir_owner : self . hir_owner , data : & self . skipped_ref_pats } } pub fn skipped_ref_pats_mut (& mut self) -> LocalSetInContextMut < '_ > { LocalSetInContextMut { hir_owner : self . hir_owner , data : & mut self . skipped_ref_pats } } #[doc = " Does the pattern recursively contain a `ref mut` binding in it?"] #[doc = ""] #[doc = " This is used to determined whether a `deref` pattern should emit a `Deref`"] #[doc = " or `DerefMut` call for its pattern scrutinee."] #[doc = ""] #[doc = " This is computed from the typeck results since we want to make"] #[doc = " sure to apply any match-ergonomics adjustments, which we cannot"] #[doc = " determine from the HIR alone."] pub fn pat_has_ref_mut_binding (& self , pat : & hir :: Pat < '_ >) -> bool { let mut has_ref_mut = false ; pat . walk (| pat | { if let hir :: PatKind :: Binding (_ , id , _ , _) = pat . kind && let Some (BindingMode (ByRef :: Yes (Mutability :: Mut) , _)) = self . pat_binding_modes () . get (id) { has_ref_mut = true ; false } else { true } }) ; has_ref_mut } #[doc = " How should a deref pattern find the place for its inner pattern to match on?"] #[doc = ""] #[doc = " In most cases, if the pattern recursively contains a `ref mut` binding, we find the inner"] #[doc = " pattern's scrutinee by calling `DerefMut::deref_mut`, and otherwise we call `Deref::deref`."] #[doc = " However, for boxes we can use a built-in deref instead, which doesn't borrow the scrutinee;"] #[doc = " in this case, we return `ByRef::No`."] pub fn deref_pat_borrow_mode (& self , pointer_ty : Ty < '_ > , inner : & hir :: Pat < '_ >) -> ByRef { if pointer_ty . is_box () { ByRef :: No } else { let mutable = self . pat_has_ref_mut_binding (inner) ; ByRef :: Yes (if mutable { Mutability :: Mut } else { Mutability :: Not }) } } #[doc = " For a given closure, returns the iterator of `ty::CapturedPlace`s that are captured"] #[doc = " by the closure."] pub fn closure_min_captures_flattened (& self , closure_def_id : LocalDefId ,) -> impl Iterator < Item = & ty :: CapturedPlace < 'tcx > > { self . closure_min_captures . get (& closure_def_id) . map (| closure_min_captures | closure_min_captures . values () . flat_map (| v | v . iter ())) . into_iter () . flatten () } pub fn closure_kind_origins (& self) -> LocalTableInContext < '_ , (Span , HirPlace < 'tcx >) > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . closure_kind_origins } } pub fn closure_kind_origins_mut (& mut self ,) -> LocalTableInContextMut < '_ , (Span , HirPlace < 'tcx >) > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . closure_kind_origins } } pub fn liberated_fn_sigs (& self) -> LocalTableInContext < '_ , ty :: FnSig < 'tcx > > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . liberated_fn_sigs } } pub fn liberated_fn_sigs_mut (& mut self) -> LocalTableInContextMut < '_ , ty :: FnSig < 'tcx > > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . liberated_fn_sigs } } pub fn fru_field_types (& self) -> LocalTableInContext < '_ , Vec < Ty < 'tcx > > > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . fru_field_types } } pub fn fru_field_types_mut (& mut self) -> LocalTableInContextMut < '_ , Vec < Ty < 'tcx > > > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . fru_field_types } } pub fn is_coercion_cast (& self , hir_id : HirId) -> bool { validate_hir_id_for_typeck_results (self . hir_owner , hir_id) ; self . coercion_casts . contains (& hir_id . local_id) } pub fn set_coercion_cast (& mut self , id : ItemLocalId) { self . coercion_casts . insert (id) ; } pub fn coercion_casts (& self) -> & ItemLocalSet { & self . coercion_casts } pub fn offset_of_data (& self ,) -> LocalTableInContext < '_ , (Ty < 'tcx > , Vec < (VariantIdx , FieldIdx) >) > { LocalTableInContext { hir_owner : self . hir_owner , data : & self . offset_of_data } } pub fn offset_of_data_mut (& mut self ,) -> LocalTableInContextMut < '_ , (Ty < 'tcx > , Vec < (VariantIdx , FieldIdx) >) > { LocalTableInContextMut { hir_owner : self . hir_owner , data : & mut self . offset_of_data } } }}}

macro_rules! validate_hir_id_for_typeck_results_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function validate_hir_id_for_typeck_results in module {}", module_path!());
    };
}

mkfn!{
    validate_hir_id_for_typeck_results_introspect!();
    #[doc = " Validate that the given HirId (respectively its `local_id` part) can be"] #[doc = " safely used as a key in the maps of a TypeckResults. For that to be"] #[doc = " the case, the HirId must have the same `owner` as all the other IDs in"] #[doc = " this table (signified by `hir_owner`). Otherwise the HirId"] #[doc = " would be in a different frame of reference and using its `local_id`"] #[doc = " would result in lookup errors, or worse, in silently wrong data being"] #[doc = " stored/returned."] #[inline] fn validate_hir_id_for_typeck_results (hir_owner : OwnerId , hir_id : HirId) { if hir_id . owner != hir_owner { invalid_hir_id_for_typeck_results (hir_owner , hir_id) ; } }
}

macro_rules! invalid_hir_id_for_typeck_results_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invalid_hir_id_for_typeck_results in module {}", module_path!());
    };
}

mkfn!{
    invalid_hir_id_for_typeck_results_introspect!();
    #[cold] #[inline (never)] fn invalid_hir_id_for_typeck_results (hir_owner : OwnerId , hir_id : HirId) { ty :: tls :: with (| tcx | { bug ! ("node {} cannot be placed in TypeckResults with hir_owner {:?}" , tcx . hir_id_to_string (hir_id) , hir_owner) }) ; }
}
mkitem!{mkstruct!{pub struct LocalTableInContext < 'a , V > { hir_owner : OwnerId , data : & 'a ItemLocalMap < V > , }}}
mkitem!{mkimpl!{impl < 'a , V > LocalTableInContext < 'a , V > { pub fn contains_key (& self , id : HirId) -> bool { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . data . contains_key (& id . local_id) } pub fn get (& self , id : HirId) -> Option < & 'a V > { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . data . get (& id . local_id) } pub fn items (& self ,) -> UnordItems < (hir :: ItemLocalId , & 'a V) , impl Iterator < Item = (hir :: ItemLocalId , & 'a V) > > { self . data . items () . map (| (id , value) | (* id , value)) } pub fn items_in_stable_order (& self) -> Vec < (ItemLocalId , & 'a V) > { self . data . items () . map (| (& k , v) | (k , v)) . into_sorted_stable_ord_by_key (| (k , _) | k) } }}}
mkitem!{mkimpl!{impl < 'a , V > :: std :: ops :: Index < HirId > for LocalTableInContext < 'a , V > { type Output = V ; fn index (& self , key : HirId) -> & V { self . get (key) . unwrap_or_else (| | { bug ! ("LocalTableInContext({:?}): key {:?} not found" , self . hir_owner , key) }) } }}}
mkitem!{mkstruct!{pub struct LocalTableInContextMut < 'a , V > { hir_owner : OwnerId , data : & 'a mut ItemLocalMap < V > , }}}
mkitem!{mkimpl!{impl < 'a , V > LocalTableInContextMut < 'a , V > { pub fn get_mut (& mut self , id : HirId) -> Option < & mut V > { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . data . get_mut (& id . local_id) } pub fn get (& mut self , id : HirId) -> Option < & V > { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . data . get (& id . local_id) } pub fn entry (& mut self , id : HirId) -> Entry < '_ , hir :: ItemLocalId , V > { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . data . entry (id . local_id) } pub fn insert (& mut self , id : HirId , val : V) -> Option < V > { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . data . insert (id . local_id , val) } pub fn remove (& mut self , id : HirId) -> Option < V > { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . data . remove (& id . local_id) } pub fn extend (& mut self , items : UnordItems < (HirId , V) , impl Iterator < Item = (HirId , V) > >) { self . data . extend_unord (items . map (| (id , value) | { validate_hir_id_for_typeck_results (self . hir_owner , id) ; (id . local_id , value) })) } }}}
mkitem!{mkstruct!{#[derive (Clone , Copy , Debug)] pub struct LocalSetInContext < 'a > { hir_owner : OwnerId , data : & 'a ItemLocalSet , }}}
mkitem!{mkimpl!{impl < 'a > LocalSetInContext < 'a > { pub fn is_empty (& self) -> bool { self . data . is_empty () } pub fn contains (& self , id : hir :: HirId) -> bool { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . data . contains (& id . local_id) } }}}
mkitem!{mkstruct!{#[derive (Debug)] pub struct LocalSetInContextMut < 'a > { hir_owner : OwnerId , data : & 'a mut ItemLocalSet , }}}
mkitem!{mkimpl!{impl < 'a > LocalSetInContextMut < 'a > { pub fn is_empty (& self) -> bool { self . data . is_empty () } pub fn contains (& self , id : hir :: HirId) -> bool { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . data . contains (& id . local_id) } pub fn insert (& mut self , id : hir :: HirId) -> bool { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . data . insert (id . local_id) } pub fn remove (& mut self , id : hir :: HirId) -> bool { validate_hir_id_for_typeck_results (self . hir_owner , id) ; self . data . remove (& id . local_id) } }}}
mkitem!{rustc_index :: newtype_index ! { #[derive (HashStable)] #[encodable] #[debug_format = "UserType({})"] pub struct UserTypeAnnotationIndex { const START_INDEX = 0 ; } }}
mkitem!{#[doc = " Mapping of type annotation indices to canonical user type annotations."] pub type CanonicalUserTypeAnnotations < 'tcx > = IndexVec < UserTypeAnnotationIndex , CanonicalUserTypeAnnotation < 'tcx > > ;}
mkitem!{mkstruct!{#[derive (Clone , Debug , TyEncodable , TyDecodable , HashStable , TypeFoldable , TypeVisitable)] pub struct CanonicalUserTypeAnnotation < 'tcx > { #[type_foldable (identity)] #[type_visitable (ignore)] pub user_ty : Box < CanonicalUserType < 'tcx > > , pub span : Span , pub inferred_ty : Ty < 'tcx > , }}}
mkitem!{#[doc = " Canonical user type annotation."] pub type CanonicalUserType < 'tcx > = Canonical < 'tcx , UserType < 'tcx > > ;}
mkitem!{mkstruct!{#[derive (Copy , Clone , Debug , PartialEq , TyEncodable , TyDecodable)] #[derive (Eq , Hash , HashStable , TypeFoldable , TypeVisitable)] pub struct UserType < 'tcx > { pub kind : UserTypeKind < 'tcx > , pub bounds : ty :: Clauses < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > UserType < 'tcx > { pub fn new (kind : UserTypeKind < 'tcx >) -> UserType < 'tcx > { UserType { kind , bounds : ty :: ListWithCachedTypeInfo :: empty () } } #[doc = " A user type annotation with additional bounds that need to be enforced."] #[doc = " These bounds are lowered from `impl Trait` in bindings."] pub fn new_with_bounds (kind : UserTypeKind < 'tcx > , bounds : ty :: Clauses < 'tcx >) -> UserType < 'tcx > { UserType { kind , bounds } } }}}
mkitem!{mkenum!{#[doc = " A user-given type annotation attached to a constant. These arise"] #[doc = " from constants that are named via paths, like `Foo::<A>::new` and"] #[doc = " so forth."] #[derive (Copy , Clone , Debug , PartialEq , TyEncodable , TyDecodable)] #[derive (Eq , Hash , HashStable , TypeFoldable , TypeVisitable)] pub enum UserTypeKind < 'tcx > { Ty (Ty < 'tcx >) , #[doc = " The canonical type is the result of `type_of(def_id)` with the"] #[doc = " given generic parameters applied."] TypeOf (DefId , UserArgs < 'tcx >) , }}}
mkitem!{mktrait!{pub trait IsIdentity { fn is_identity (& self) -> bool ; }}}
mkitem!{mkimpl!{impl < 'tcx > IsIdentity for CanonicalUserType < 'tcx > { #[doc = " Returns `true` if this represents the generic parameters of the form `[?0, ?1, ?2]`,"] #[doc = " i.e., each thing is mapped to a canonical variable with the same index."] fn is_identity (& self) -> bool { if ! self . value . bounds . is_empty () { return false ; } match self . value . kind { UserTypeKind :: Ty (_) => false , UserTypeKind :: TypeOf (_ , user_args) => { if user_args . user_self_ty . is_some () { return false ; } iter :: zip (user_args . args , BoundVar :: ZERO ..) . all (| (arg , cvar) | { match arg . kind () { GenericArgKind :: Type (ty) => match ty . kind () { ty :: Bound (debruijn , b) => { assert_eq ! (* debruijn , ty :: INNERMOST) ; cvar == b . var } _ => false , } , GenericArgKind :: Lifetime (r) => match r . kind () { ty :: ReBound (debruijn , b) => { assert_eq ! (debruijn , ty :: INNERMOST) ; cvar == b . var } _ => false , } , GenericArgKind :: Const (ct) => match ct . kind () { ty :: ConstKind :: Bound (debruijn , b) => { assert_eq ! (debruijn , ty :: INNERMOST) ; cvar == b . var } _ => false , } , } }) } } } }}}
mkitem!{mkimpl!{impl < 'tcx > std :: fmt :: Display for UserType < 'tcx > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if self . bounds . is_empty () { self . kind . fmt (f) } else { self . kind . fmt (f) ? ; write ! (f , " + ") ? ; std :: fmt :: Debug :: fmt (& self . bounds , f) } } }}}
mkitem!{mkimpl!{impl < 'tcx > std :: fmt :: Display for UserTypeKind < 'tcx > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Ty (arg0) => { ty :: print :: with_no_trimmed_paths ! (write ! (f , "Ty({})" , arg0)) } Self :: TypeOf (arg0 , arg1) => write ! (f , "TypeOf({:?}, {:?})" , arg0 , arg1) , } } }}}
mkitem!{mkstruct!{#[doc = " Information on a pattern incompatible with Rust 2024, for use by the error/migration diagnostic"] #[doc = " emitted during THIR construction."] #[derive (TyEncodable , TyDecodable , Debug , HashStable)] pub struct Rust2024IncompatiblePatInfo { #[doc = " Labeled spans for `&`s, `&mut`s, and binding modifiers incompatible with Rust 2024."] pub primary_labels : Vec < (Span , String) > , #[doc = " Whether any binding modifiers occur under a non-`move` default binding mode."] pub bad_modifiers : bool , #[doc = " Whether any `&` or `&mut` patterns occur under a non-`move` default binding mode."] pub bad_ref_pats : bool , #[doc = " If `true`, we can give a simpler suggestion solely by eliding explicit binding modifiers."] pub suggest_eliding_modes : bool , }}}