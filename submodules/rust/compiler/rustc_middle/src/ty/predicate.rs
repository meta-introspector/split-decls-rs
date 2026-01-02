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
mkuse!{use std :: cmp :: Ordering ;}
mkuse!{use rustc_data_structures :: intern :: Interned ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_macros :: { HashStable , extension } ;}
mkuse!{use rustc_type_ir as ir ;}
mkuse!{use crate :: ty :: { self , DebruijnIndex , EarlyBinder , Ty , TyCtxt , TypeFlags , Upcast , UpcastFrom , WithCachedTypeInfo , } ;}
mkitem!{pub type TraitRef < 'tcx > = ir :: TraitRef < TyCtxt < 'tcx > > ;}
mkitem!{pub type AliasTerm < 'tcx > = ir :: AliasTerm < TyCtxt < 'tcx > > ;}
mkitem!{pub type ProjectionPredicate < 'tcx > = ir :: ProjectionPredicate < TyCtxt < 'tcx > > ;}
mkitem!{pub type ExistentialPredicate < 'tcx > = ir :: ExistentialPredicate < TyCtxt < 'tcx > > ;}
mkitem!{pub type ExistentialTraitRef < 'tcx > = ir :: ExistentialTraitRef < TyCtxt < 'tcx > > ;}
mkitem!{pub type ExistentialProjection < 'tcx > = ir :: ExistentialProjection < TyCtxt < 'tcx > > ;}
mkitem!{pub type TraitPredicate < 'tcx > = ir :: TraitPredicate < TyCtxt < 'tcx > > ;}
mkitem!{pub type HostEffectPredicate < 'tcx > = ir :: HostEffectPredicate < TyCtxt < 'tcx > > ;}
mkitem!{pub type ClauseKind < 'tcx > = ir :: ClauseKind < TyCtxt < 'tcx > > ;}
mkitem!{pub type PredicateKind < 'tcx > = ir :: PredicateKind < TyCtxt < 'tcx > > ;}
mkitem!{pub type NormalizesTo < 'tcx > = ir :: NormalizesTo < TyCtxt < 'tcx > > ;}
mkitem!{pub type CoercePredicate < 'tcx > = ir :: CoercePredicate < TyCtxt < 'tcx > > ;}
mkitem!{pub type SubtypePredicate < 'tcx > = ir :: SubtypePredicate < TyCtxt < 'tcx > > ;}
mkitem!{pub type OutlivesPredicate < 'tcx , T > = ir :: OutlivesPredicate < TyCtxt < 'tcx > , T > ;}
mkitem!{pub type RegionOutlivesPredicate < 'tcx > = OutlivesPredicate < 'tcx , ty :: Region < 'tcx > > ;}
mkitem!{pub type TypeOutlivesPredicate < 'tcx > = OutlivesPredicate < 'tcx , Ty < 'tcx > > ;}
mkitem!{pub type ArgOutlivesPredicate < 'tcx > = OutlivesPredicate < 'tcx , ty :: GenericArg < 'tcx > > ;}
mkitem!{pub type PolyTraitPredicate < 'tcx > = ty :: Binder < 'tcx , TraitPredicate < 'tcx > > ;}
mkitem!{pub type PolyRegionOutlivesPredicate < 'tcx > = ty :: Binder < 'tcx , RegionOutlivesPredicate < 'tcx > > ;}
mkitem!{pub type PolyTypeOutlivesPredicate < 'tcx > = ty :: Binder < 'tcx , TypeOutlivesPredicate < 'tcx > > ;}
mkitem!{pub type PolySubtypePredicate < 'tcx > = ty :: Binder < 'tcx , SubtypePredicate < 'tcx > > ;}
mkitem!{pub type PolyCoercePredicate < 'tcx > = ty :: Binder < 'tcx , CoercePredicate < 'tcx > > ;}
mkitem!{pub type PolyProjectionPredicate < 'tcx > = ty :: Binder < 'tcx , ProjectionPredicate < 'tcx > > ;}
mkitem!{mkstruct!{# [doc = " A statement that can be proven by a trait solver. This includes things that may"] # [doc = " show up in where clauses, such as trait predicates and projection predicates,"] # [doc = " and also things that are emitted as part of type checking such as `DynCompatible`"] # [doc = " predicate which is emitted when a type is coerced to a trait object."] # [doc = ""] # [doc = " Use this rather than `PredicateKind`, whenever possible."] # [derive (Clone , Copy , PartialEq , Eq , Hash , HashStable)] # [rustc_pass_by_value] pub struct Predicate < 'tcx > (pub (super) Interned < 'tcx , WithCachedTypeInfo < ty :: Binder < 'tcx , PredicateKind < 'tcx > > > > ,) ;}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: Predicate < TyCtxt < 'tcx > > for Predicate < 'tcx > { fn as_clause (self) -> Option < ty :: Clause < 'tcx > > { self . as_clause () } fn allow_normalization (self) -> bool { self . allow_normalization () } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: IntoKind for Predicate < 'tcx > { type Kind = ty :: Binder < 'tcx , ty :: PredicateKind < 'tcx > > ; fn kind (self) -> Self :: Kind { self . kind () } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: Flags for Predicate < 'tcx > { fn flags (& self) -> TypeFlags { self . 0 . flags } fn outer_exclusive_binder (& self) -> ty :: DebruijnIndex { self . 0 . outer_exclusive_binder } }}}
mkitem!{mkimpl!{impl < 'tcx > Predicate < 'tcx > { # [doc = " Gets the inner `ty::Binder<'tcx, PredicateKind<'tcx>>`."] # [inline] pub fn kind (self) -> ty :: Binder < 'tcx , PredicateKind < 'tcx > > { self . 0 . internee } # [inline (always)] pub fn flags (self) -> TypeFlags { self . 0 . flags } # [inline (always)] pub fn outer_exclusive_binder (self) -> DebruijnIndex { self . 0 . outer_exclusive_binder } # [doc = " Flips the polarity of a Predicate."] # [doc = ""] # [doc = " Given `T: Trait` predicate it returns `T: !Trait` and given `T: !Trait` returns `T: Trait`."] pub fn flip_polarity (self , tcx : TyCtxt < 'tcx >) -> Option < Predicate < 'tcx > > { let kind = self . kind () . map_bound (| kind | match kind { PredicateKind :: Clause (ClauseKind :: Trait (TraitPredicate { trait_ref , polarity , })) => Some (PredicateKind :: Clause (ClauseKind :: Trait (TraitPredicate { trait_ref , polarity : polarity . flip () , }))) , _ => None , }) . transpose () ? ; Some (tcx . mk_predicate (kind)) } # [doc = " Whether this projection can be soundly normalized."] # [doc = ""] # [doc = " Wf predicates must not be normalized, as normalization"] # [doc = " can remove required bounds which would cause us to"] # [doc = " unsoundly accept some programs. See #91068."] # [inline] pub fn allow_normalization (self) -> bool { match self . kind () . skip_binder () { PredicateKind :: Clause (ClauseKind :: WellFormed (_)) | PredicateKind :: AliasRelate (..) => { false } PredicateKind :: Clause (ClauseKind :: Trait (_)) | PredicateKind :: Clause (ClauseKind :: HostEffect (..)) | PredicateKind :: Clause (ClauseKind :: RegionOutlives (_)) | PredicateKind :: Clause (ClauseKind :: TypeOutlives (_)) | PredicateKind :: Clause (ClauseKind :: Projection (_)) | PredicateKind :: Clause (ClauseKind :: ConstArgHasType (..)) | PredicateKind :: Clause (ClauseKind :: UnstableFeature (_)) | PredicateKind :: DynCompatible (_) | PredicateKind :: Subtype (_) | PredicateKind :: Coerce (_) | PredicateKind :: Clause (ClauseKind :: ConstEvaluatable (_)) | PredicateKind :: ConstEquate (_ , _) | PredicateKind :: NormalizesTo (..) | PredicateKind :: Ambiguous => true , } } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_errors :: IntoDiagArg for Predicate < 'tcx > { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> rustc_errors :: DiagArgValue { ty :: tls :: with (| tcx | { let pred = tcx . short_string (self , path) ; rustc_errors :: DiagArgValue :: Str (std :: borrow :: Cow :: Owned (pred)) }) } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_errors :: IntoDiagArg for Clause < 'tcx > { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> rustc_errors :: DiagArgValue { ty :: tls :: with (| tcx | { let clause = tcx . short_string (self , path) ; rustc_errors :: DiagArgValue :: Str (std :: borrow :: Cow :: Owned (clause)) }) } }}}
mkitem!{mkstruct!{# [doc = " A subset of predicates which can be assumed by the trait solver. They show up in"] # [doc = " an item's where clauses, hence the name `Clause`, and may either be user-written"] # [doc = " (such as traits) or may be inserted during lowering."] # [derive (Clone , Copy , PartialEq , Eq , Hash , HashStable)] # [rustc_pass_by_value] pub struct Clause < 'tcx > (pub (super) Interned < 'tcx , WithCachedTypeInfo < ty :: Binder < 'tcx , PredicateKind < 'tcx > > > > ,) ;}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: Clause < TyCtxt < 'tcx > > for Clause < 'tcx > { fn as_predicate (self) -> Predicate < 'tcx > { self . as_predicate () } fn instantiate_supertrait (self , tcx : TyCtxt < 'tcx > , trait_ref : ty :: PolyTraitRef < 'tcx >) -> Self { self . instantiate_supertrait (tcx , trait_ref) } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: IntoKind for Clause < 'tcx > { type Kind = ty :: Binder < 'tcx , ClauseKind < 'tcx > > ; fn kind (self) -> Self :: Kind { self . kind () } }}}
mkitem!{mkimpl!{impl < 'tcx > Clause < 'tcx > { pub fn as_predicate (self) -> Predicate < 'tcx > { Predicate (self . 0) } pub fn kind (self) -> ty :: Binder < 'tcx , ClauseKind < 'tcx > > { self . 0 . internee . map_bound (| kind | match kind { PredicateKind :: Clause (clause) => clause , _ => unreachable ! () , }) } pub fn as_trait_clause (self) -> Option < ty :: Binder < 'tcx , TraitPredicate < 'tcx > > > { let clause = self . kind () ; if let ty :: ClauseKind :: Trait (trait_clause) = clause . skip_binder () { Some (clause . rebind (trait_clause)) } else { None } } pub fn as_projection_clause (self) -> Option < ty :: Binder < 'tcx , ProjectionPredicate < 'tcx > > > { let clause = self . kind () ; if let ty :: ClauseKind :: Projection (projection_clause) = clause . skip_binder () { Some (clause . rebind (projection_clause)) } else { None } } pub fn as_type_outlives_clause (self) -> Option < ty :: Binder < 'tcx , TypeOutlivesPredicate < 'tcx > > > { let clause = self . kind () ; if let ty :: ClauseKind :: TypeOutlives (o) = clause . skip_binder () { Some (clause . rebind (o)) } else { None } } pub fn as_region_outlives_clause (self ,) -> Option < ty :: Binder < 'tcx , RegionOutlivesPredicate < 'tcx > > > { let clause = self . kind () ; if let ty :: ClauseKind :: RegionOutlives (o) = clause . skip_binder () { Some (clause . rebind (o)) } else { None } } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: Clauses < TyCtxt < 'tcx > > for ty :: Clauses < 'tcx > { }}}
mkitem!{mkimpl!{# [extension (pub trait ExistentialPredicateStableCmpExt <'tcx >)] impl < 'tcx > ExistentialPredicate < 'tcx > { # [doc = " Compares via an ordering that will not change if modules are reordered or other changes are"] # [doc = " made to the tree. In particular, this ordering is preserved across incremental compilations."] fn stable_cmp (& self , tcx : TyCtxt < 'tcx > , other : & Self) -> Ordering { match (* self , * other) { (ExistentialPredicate :: Trait (_) , ExistentialPredicate :: Trait (_)) => Ordering :: Equal , (ExistentialPredicate :: Projection (ref a) , ExistentialPredicate :: Projection (ref b)) => { tcx . def_path_hash (a . def_id) . cmp (& tcx . def_path_hash (b . def_id)) } (ExistentialPredicate :: AutoTrait (ref a) , ExistentialPredicate :: AutoTrait (ref b)) => { tcx . def_path_hash (* a) . cmp (& tcx . def_path_hash (* b)) } (ExistentialPredicate :: Trait (_) , _) => Ordering :: Less , (ExistentialPredicate :: Projection (_) , ExistentialPredicate :: Trait (_)) => { Ordering :: Greater } (ExistentialPredicate :: Projection (_) , _) => Ordering :: Less , (ExistentialPredicate :: AutoTrait (_) , _) => Ordering :: Greater , } } }}}
mkitem!{pub type PolyExistentialPredicate < 'tcx > = ty :: Binder < 'tcx , ExistentialPredicate < 'tcx > > ;}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: BoundExistentialPredicates < TyCtxt < 'tcx > > for & 'tcx ty :: List < ty :: PolyExistentialPredicate < 'tcx > > { fn principal_def_id (self) -> Option < DefId > { self . principal_def_id () } fn principal (self) -> Option < ty :: PolyExistentialTraitRef < 'tcx > > { self . principal () } fn auto_traits (self) -> impl IntoIterator < Item = DefId > { self . auto_traits () } fn projection_bounds (self ,) -> impl IntoIterator < Item = ty :: Binder < 'tcx , ExistentialProjection < 'tcx > > > { self . projection_bounds () } }}}
mkitem!{mkimpl!{impl < 'tcx > ty :: List < ty :: PolyExistentialPredicate < 'tcx > > { # [doc = " Returns the \"principal `DefId`\" of this set of existential predicates."] # [doc = ""] # [doc = " A Rust trait object type consists (in addition to a lifetime bound)"] # [doc = " of a set of trait bounds, which are separated into any number"] # [doc = " of auto-trait bounds, and at most one non-auto-trait bound. The"] # [doc = " non-auto-trait bound is called the \"principal\" of the trait"] # [doc = " object."] # [doc = ""] # [doc = " Only the principal can have methods or type parameters (because"] # [doc = " auto traits can have neither of them). This is important, because"] # [doc = " it means the auto traits can be treated as an unordered set (methods"] # [doc = " would force an order for the vtable, while relating traits with"] # [doc = " type parameters without knowing the order to relate them in is"] # [doc = " a rather non-trivial task)."] # [doc = ""] # [doc = " For example, in the trait object `dyn std::fmt::Debug + Sync`, the"] # [doc = " principal bound is `Some(std::fmt::Debug)`, while the auto-trait bounds"] # [doc = " are the set `{Sync}`."] # [doc = ""] # [doc = " It is also possible to have a \"trivial\" trait object that"] # [doc = " consists only of auto traits, with no principal - for example,"] # [doc = " `dyn Send + Sync`. In that case, the set of auto-trait bounds"] # [doc = " is `{Send, Sync}`, while there is no principal. These trait objects"] # [doc = " have a \"trivial\" vtable consisting of just the size, alignment,"] # [doc = " and destructor."] pub fn principal (& self) -> Option < ty :: Binder < 'tcx , ExistentialTraitRef < 'tcx > > > { self [0] . map_bound (| this | match this { ExistentialPredicate :: Trait (tr) => Some (tr) , _ => None , }) . transpose () } pub fn principal_def_id (& self) -> Option < DefId > { self . principal () . map (| trait_ref | trait_ref . skip_binder () . def_id) } # [inline] pub fn projection_bounds (& self ,) -> impl Iterator < Item = ty :: Binder < 'tcx , ExistentialProjection < 'tcx > > > { self . iter () . filter_map (| predicate | { predicate . map_bound (| pred | match pred { ExistentialPredicate :: Projection (projection) => Some (projection) , _ => None , }) . transpose () }) } # [inline] pub fn auto_traits (& self) -> impl Iterator < Item = DefId > { self . iter () . filter_map (| predicate | match predicate . skip_binder () { ExistentialPredicate :: AutoTrait (did) => Some (did) , _ => None , }) } pub fn without_auto_traits (& self) -> impl Iterator < Item = ty :: PolyExistentialPredicate < 'tcx > > { self . iter () . filter (| predicate | { ! matches ! (predicate . as_ref () . skip_binder () , ExistentialPredicate :: AutoTrait (_)) }) } }}}
mkitem!{pub type PolyTraitRef < 'tcx > = ty :: Binder < 'tcx , TraitRef < 'tcx > > ;}
mkitem!{pub type PolyExistentialTraitRef < 'tcx > = ty :: Binder < 'tcx , ExistentialTraitRef < 'tcx > > ;}
mkitem!{pub type PolyExistentialProjection < 'tcx > = ty :: Binder < 'tcx , ExistentialProjection < 'tcx > > ;}
mkitem!{mkimpl!{impl < 'tcx > Clause < 'tcx > { # [doc = " Performs a instantiation suitable for going from a"] # [doc = " poly-trait-ref to supertraits that must hold if that"] # [doc = " poly-trait-ref holds. This is slightly different from a normal"] # [doc = " instantiation in terms of what happens with bound regions. See"] # [doc = " lengthy comment below for details."] pub fn instantiate_supertrait (self , tcx : TyCtxt < 'tcx > , trait_ref : ty :: PolyTraitRef < 'tcx > ,) -> Clause < 'tcx > { let bound_pred = self . kind () ; let pred_bound_vars = bound_pred . bound_vars () ; let trait_bound_vars = trait_ref . bound_vars () ; let shifted_pred = tcx . shift_bound_var_indices (trait_bound_vars . len () , bound_pred . skip_binder ()) ; let new = EarlyBinder :: bind (shifted_pred) . instantiate (tcx , trait_ref . skip_binder () . args) ; let bound_vars = tcx . mk_bound_variable_kinds_from_iter (trait_bound_vars . iter () . chain (pred_bound_vars)) ; tcx . reuse_or_mk_predicate (self . as_predicate () , ty :: Binder :: bind_with_vars (PredicateKind :: Clause (new) , bound_vars) ,) . expect_clause () } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , PredicateKind < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : PredicateKind < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { ty :: Binder :: dummy (from) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , ty :: Binder < 'tcx , PredicateKind < 'tcx > > > for Predicate < 'tcx > { fn upcast_from (from : ty :: Binder < 'tcx , PredicateKind < 'tcx > > , tcx : TyCtxt < 'tcx >) -> Self { tcx . mk_predicate (from) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , ClauseKind < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : ClauseKind < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { tcx . mk_predicate (ty :: Binder :: dummy (PredicateKind :: Clause (from))) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , ty :: Binder < 'tcx , ClauseKind < 'tcx > > > for Predicate < 'tcx > { fn upcast_from (from : ty :: Binder < 'tcx , ClauseKind < 'tcx > > , tcx : TyCtxt < 'tcx >) -> Self { tcx . mk_predicate (from . map_bound (PredicateKind :: Clause)) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , Clause < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : Clause < 'tcx > , _tcx : TyCtxt < 'tcx >) -> Self { from . as_predicate () } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , ClauseKind < 'tcx > > for Clause < 'tcx > { fn upcast_from (from : ClauseKind < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { tcx . mk_predicate (ty :: Binder :: dummy (PredicateKind :: Clause (from))) . expect_clause () } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , ty :: Binder < 'tcx , ClauseKind < 'tcx > > > for Clause < 'tcx > { fn upcast_from (from : ty :: Binder < 'tcx , ClauseKind < 'tcx > > , tcx : TyCtxt < 'tcx >) -> Self { tcx . mk_predicate (from . map_bound (| clause | PredicateKind :: Clause (clause))) . expect_clause () } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , TraitRef < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : TraitRef < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { ty :: Binder :: dummy (from) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , TraitRef < 'tcx > > for Clause < 'tcx > { fn upcast_from (from : TraitRef < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { let p : Predicate < 'tcx > = from . upcast (tcx) ; p . expect_clause () } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , ty :: Binder < 'tcx , TraitRef < 'tcx > > > for Predicate < 'tcx > { fn upcast_from (from : ty :: Binder < 'tcx , TraitRef < 'tcx > > , tcx : TyCtxt < 'tcx >) -> Self { let pred : PolyTraitPredicate < 'tcx > = from . upcast (tcx) ; pred . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , ty :: Binder < 'tcx , TraitRef < 'tcx > > > for Clause < 'tcx > { fn upcast_from (from : ty :: Binder < 'tcx , TraitRef < 'tcx > > , tcx : TyCtxt < 'tcx >) -> Self { let pred : PolyTraitPredicate < 'tcx > = from . upcast (tcx) ; pred . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , TraitPredicate < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : TraitPredicate < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { PredicateKind :: Clause (ClauseKind :: Trait (from)) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , PolyTraitPredicate < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : PolyTraitPredicate < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { from . map_bound (| p | PredicateKind :: Clause (ClauseKind :: Trait (p))) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , TraitPredicate < 'tcx > > for Clause < 'tcx > { fn upcast_from (from : TraitPredicate < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { let p : Predicate < 'tcx > = from . upcast (tcx) ; p . expect_clause () } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , PolyTraitPredicate < 'tcx > > for Clause < 'tcx > { fn upcast_from (from : PolyTraitPredicate < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { let p : Predicate < 'tcx > = from . upcast (tcx) ; p . expect_clause () } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , RegionOutlivesPredicate < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : RegionOutlivesPredicate < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { ty :: Binder :: dummy (PredicateKind :: Clause (ClauseKind :: RegionOutlives (from))) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , PolyRegionOutlivesPredicate < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : PolyRegionOutlivesPredicate < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { from . map_bound (| p | PredicateKind :: Clause (ClauseKind :: RegionOutlives (p))) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , TypeOutlivesPredicate < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : TypeOutlivesPredicate < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { ty :: Binder :: dummy (PredicateKind :: Clause (ClauseKind :: TypeOutlives (from))) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , ProjectionPredicate < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : ProjectionPredicate < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { ty :: Binder :: dummy (PredicateKind :: Clause (ClauseKind :: Projection (from))) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , PolyProjectionPredicate < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : PolyProjectionPredicate < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { from . map_bound (| p | PredicateKind :: Clause (ClauseKind :: Projection (p))) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , ProjectionPredicate < 'tcx > > for Clause < 'tcx > { fn upcast_from (from : ProjectionPredicate < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { let p : Predicate < 'tcx > = from . upcast (tcx) ; p . expect_clause () } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , PolyProjectionPredicate < 'tcx > > for Clause < 'tcx > { fn upcast_from (from : PolyProjectionPredicate < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { let p : Predicate < 'tcx > = from . upcast (tcx) ; p . expect_clause () } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , ty :: Binder < 'tcx , ty :: HostEffectPredicate < 'tcx > > > for Predicate < 'tcx > { fn upcast_from (from : ty :: Binder < 'tcx , ty :: HostEffectPredicate < 'tcx > > , tcx : TyCtxt < 'tcx > ,) -> Self { from . map_bound (ty :: ClauseKind :: HostEffect) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , ty :: Binder < 'tcx , ty :: HostEffectPredicate < 'tcx > > > for Clause < 'tcx > { fn upcast_from (from : ty :: Binder < 'tcx , ty :: HostEffectPredicate < 'tcx > > , tcx : TyCtxt < 'tcx > ,) -> Self { from . map_bound (ty :: ClauseKind :: HostEffect) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > UpcastFrom < TyCtxt < 'tcx > , NormalizesTo < 'tcx > > for Predicate < 'tcx > { fn upcast_from (from : NormalizesTo < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { PredicateKind :: NormalizesTo (from) . upcast (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > Predicate < 'tcx > { pub fn as_trait_clause (self) -> Option < PolyTraitPredicate < 'tcx > > { let predicate = self . kind () ; match predicate . skip_binder () { PredicateKind :: Clause (ClauseKind :: Trait (t)) => Some (predicate . rebind (t)) , _ => None , } } pub fn as_projection_clause (self) -> Option < PolyProjectionPredicate < 'tcx > > { let predicate = self . kind () ; match predicate . skip_binder () { PredicateKind :: Clause (ClauseKind :: Projection (t)) => Some (predicate . rebind (t)) , _ => None , } } # [doc = " Matches a `PredicateKind::Clause` and turns it into a `Clause`, otherwise returns `None`."] pub fn as_clause (self) -> Option < Clause < 'tcx > > { match self . kind () . skip_binder () { PredicateKind :: Clause (..) => Some (self . expect_clause ()) , _ => None , } } # [doc = " Assert that the predicate is a clause."] pub fn expect_clause (self) -> Clause < 'tcx > { match self . kind () . skip_binder () { PredicateKind :: Clause (..) => Clause (self . 0) , _ => bug ! ("{self} is not a clause") , } } }}}
mkmod!{size_asserts, { 
                getname!(size_asserts);
                getsrc!(size_asserts);
                getpath!(size_asserts);
                get_deps!(size_asserts);
                get_crates!(size_asserts);
                mkinclude!(size_asserts);
                mkuse!{use rustc_data_structures :: static_assert_size ;}
mkuse!{use super :: * ;}
mkitem!{static_assert_size ! (PredicateKind <'_ >, 32) ;}
mkitem!{static_assert_size ! (WithCachedTypeInfo < PredicateKind <'_ >>, 56) ;} 
            }}