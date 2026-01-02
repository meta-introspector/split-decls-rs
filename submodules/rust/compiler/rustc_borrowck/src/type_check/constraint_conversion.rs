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
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_infer :: infer :: SubregionOrigin ;}
mkuse!{use rustc_infer :: infer :: canonical :: QueryRegionConstraints ;}
mkuse!{use rustc_infer :: infer :: outlives :: env :: RegionBoundPairs ;}
mkuse!{use rustc_infer :: infer :: outlives :: obligations :: { TypeOutlives , TypeOutlivesDelegate } ;}
mkuse!{use rustc_infer :: infer :: region_constraints :: { GenericKind , VerifyBound } ;}
mkuse!{use rustc_infer :: traits :: query :: type_op :: DeeplyNormalize ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgKind , Ty , TyCtxt , TypeFoldable , TypeVisitableExt , elaborate , fold_regions , } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_trait_selection :: traits :: query :: type_op :: { TypeOp , TypeOpOutput } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: constraints :: OutlivesConstraint ;}
mkuse!{use crate :: region_infer :: TypeTest ;}
mkuse!{use crate :: type_check :: { Locations , MirTypeckRegionConstraints } ;}
mkuse!{use crate :: universal_regions :: UniversalRegions ;}
mkuse!{use crate :: { BorrowckInferCtxt , ClosureOutlivesSubject , ClosureRegionRequirements , ConstraintCategory , } ;}
mkitem!{mkstruct!{pub (crate) struct ConstraintConversion < 'a , 'tcx > { infcx : & 'a BorrowckInferCtxt < 'tcx > , universal_regions : & 'a UniversalRegions < 'tcx > , # [doc = " Each RBP `GK: 'a` is assumed to be true. These encode"] # [doc = " relationships like `T: 'a` that are added via implicit bounds"] # [doc = " or the `param_env`."] # [doc = ""] # [doc = " Each region here is guaranteed to be a key in the `indices`"] # [doc = " map. We use the \"original\" regions (i.e., the keys from the"] # [doc = " map, and not the values) because the code in"] # [doc = " `process_registered_region_obligations` has some special-cased"] # [doc = " logic expecting to see (e.g.) `ReStatic`, and if we supplied"] # [doc = " our special inference variable there, we would mess that up."] region_bound_pairs : & 'a RegionBoundPairs < 'tcx > , known_type_outlives_obligations : & 'a [ty :: PolyTypeOutlivesPredicate < 'tcx >] , locations : Locations , span : Span , category : ConstraintCategory < 'tcx > , from_closure : bool , constraints : & 'a mut MirTypeckRegionConstraints < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > ConstraintConversion < 'a , 'tcx > { pub (crate) fn new (infcx : & 'a BorrowckInferCtxt < 'tcx > , universal_regions : & 'a UniversalRegions < 'tcx > , region_bound_pairs : & 'a RegionBoundPairs < 'tcx > , known_type_outlives_obligations : & 'a [ty :: PolyTypeOutlivesPredicate < 'tcx >] , locations : Locations , span : Span , category : ConstraintCategory < 'tcx > , constraints : & 'a mut MirTypeckRegionConstraints < 'tcx > ,) -> Self { Self { infcx , universal_regions , region_bound_pairs , known_type_outlives_obligations , locations , span , category , constraints , from_closure : false , } } # [instrument (skip (self) , level = "debug")] pub (super) fn convert_all (& mut self , query_constraints : & QueryRegionConstraints < 'tcx >) { let QueryRegionConstraints { outlives , assumptions } = query_constraints ; let assumptions = elaborate :: elaborate_outlives_assumptions (self . infcx . tcx , assumptions . iter () . copied ()) ; for & (predicate , constraint_category) in outlives { self . convert (predicate , constraint_category , & assumptions) ; } } # [doc = " Given an instance of the closure type, this method instantiates the \"extra\" requirements"] # [doc = " that we computed for the closure. This has the effect of adding new outlives obligations"] # [doc = " to existing region variables in `closure_args`."] # [instrument (skip (self) , level = "debug")] pub (crate) fn apply_closure_requirements (& mut self , closure_requirements : & ClosureRegionRequirements < 'tcx > , closure_def_id : LocalDefId , closure_args : ty :: GenericArgsRef < 'tcx > ,) { let closure_mapping = & UniversalRegions :: closure_mapping (self . infcx . tcx , closure_args , closure_requirements . num_external_vids , closure_def_id ,) ; debug ! (? closure_mapping) ; let backup = (self . category , self . span , self . from_closure) ; self . from_closure = true ; for outlives_requirement in & closure_requirements . outlives_requirements { let outlived_region = closure_mapping [outlives_requirement . outlived_free_region] ; let subject = match outlives_requirement . subject { ClosureOutlivesSubject :: Region (re) => closure_mapping [re] . into () , ClosureOutlivesSubject :: Ty (subject_ty) => { subject_ty . instantiate (self . infcx . tcx , | vid | closure_mapping [vid]) . into () } } ; self . category = outlives_requirement . category ; self . span = outlives_requirement . blame_span ; self . convert (ty :: OutlivesPredicate (subject , outlived_region) , self . category , & Default :: default () ,) ; } (self . category , self . span , self . from_closure) = backup ; } fn convert (& mut self , predicate : ty :: ArgOutlivesPredicate < 'tcx > , constraint_category : ConstraintCategory < 'tcx > , higher_ranked_assumptions : & FxHashSet < ty :: ArgOutlivesPredicate < 'tcx > > ,) { let tcx = self . infcx . tcx ; debug ! ("generate: constraints at: {:#?}" , self . locations) ; let ConstraintConversion { infcx , universal_regions , region_bound_pairs , known_type_outlives_obligations , .. } = * self ; let mut outlives_predicates = vec ! [(predicate , constraint_category)] ; for iteration in 0 .. { if outlives_predicates . is_empty () { break ; } if ! tcx . recursion_limit () . value_within_limit (iteration) { bug ! ("unexpected overflowed when processing region obligations: {outlives_predicates:#?}") ; } let mut next_outlives_predicates = vec ! [] ; for (pred , constraint_category) in outlives_predicates { if self . infcx . tcx . sess . opts . unstable_opts . higher_ranked_assumptions && higher_ranked_assumptions . contains (& pred) { continue ; } let ty :: OutlivesPredicate (k1 , r2) = pred ; match k1 . kind () { GenericArgKind :: Lifetime (r1) => { let r1_vid = self . to_region_vid (r1) ; let r2_vid = self . to_region_vid (r2) ; self . add_outlives (r1_vid , r2_vid , constraint_category) ; } GenericArgKind :: Type (mut t1) => { t1 = self . infcx . resolve_vars_if_possible (t1) ; if infcx . next_trait_solver () { t1 = self . normalize_and_add_type_outlives_constraints (t1 , & mut next_outlives_predicates ,) ; } let implicit_region_bound = ty :: Region :: new_var (tcx , universal_regions . implicit_region_bound ()) ; let origin = SubregionOrigin :: RelateParamBound (self . span , t1 , None) ; TypeOutlives :: new (& mut * self , tcx , region_bound_pairs , Some (implicit_region_bound) , known_type_outlives_obligations ,) . type_must_outlive (origin , t1 , r2 , constraint_category ,) ; } GenericArgKind :: Const (_) => unreachable ! () , } } outlives_predicates = next_outlives_predicates ; } } # [doc = " Placeholder regions need to be converted eagerly because it may"] # [doc = " create new region variables, which we must not do when verifying"] # [doc = " our region bounds."] # [doc = ""] # [doc = " FIXME: This should get removed once higher ranked region obligations"] # [doc = " are dealt with during trait solving."] fn replace_placeholders_with_nll < T : TypeFoldable < TyCtxt < 'tcx > > > (& mut self , value : T) -> T { if value . has_placeholders () { fold_regions (self . infcx . tcx , value , | r , _ | match r . kind () { ty :: RePlaceholder (placeholder) => { self . constraints . placeholder_region (self . infcx , placeholder) } _ => r , }) } else { value } } fn verify_to_type_test (& mut self , generic_kind : GenericKind < 'tcx > , region : ty :: Region < 'tcx > , verify_bound : VerifyBound < 'tcx > ,) -> TypeTest < 'tcx > { let lower_bound = self . to_region_vid (region) ; TypeTest { generic_kind , lower_bound , span : self . span , verify_bound } } fn to_region_vid (& mut self , r : ty :: Region < 'tcx >) -> ty :: RegionVid { if let ty :: RePlaceholder (placeholder) = r . kind () { self . constraints . placeholder_region (self . infcx , placeholder) . as_var () } else { self . universal_regions . to_region_vid (r) } } fn add_outlives (& mut self , sup : ty :: RegionVid , sub : ty :: RegionVid , category : ConstraintCategory < 'tcx > ,) { let category = match self . category { ConstraintCategory :: Boring | ConstraintCategory :: BoringNoLocation => category , _ => self . category , } ; self . constraints . outlives_constraints . push (OutlivesConstraint { locations : self . locations , category , span : self . span , sub , sup , variance_info : ty :: VarianceDiagInfo :: default () , from_closure : self . from_closure , }) ; } fn add_type_test (& mut self , type_test : TypeTest < 'tcx >) { debug ! ("add_type_test(type_test={:?})" , type_test) ; self . constraints . type_tests . push (type_test) ; } fn normalize_and_add_type_outlives_constraints (& self , ty : Ty < 'tcx > , next_outlives_predicates : & mut Vec < (ty :: ArgOutlivesPredicate < 'tcx > , ConstraintCategory < 'tcx > ,) > ,) -> Ty < 'tcx > { match self . infcx . param_env . and (DeeplyNormalize { value : ty }) . fully_perform (self . infcx , self . infcx . root_def_id , self . span ,) { Ok (TypeOpOutput { output : ty , constraints , .. }) => { if let Some (QueryRegionConstraints { outlives , assumptions : _ }) = constraints { next_outlives_predicates . extend (outlives . iter () . copied ()) ; } ty } Err (_) => ty , } } }}}
mkitem!{mkimpl!{impl < 'a , 'b , 'tcx > TypeOutlivesDelegate < 'tcx > for & 'a mut ConstraintConversion < 'b , 'tcx > { fn push_sub_region_constraint (& mut self , _origin : SubregionOrigin < 'tcx > , a : ty :: Region < 'tcx > , b : ty :: Region < 'tcx > , constraint_category : ConstraintCategory < 'tcx > ,) { let b = self . to_region_vid (b) ; let a = self . to_region_vid (a) ; self . add_outlives (b , a , constraint_category) ; } fn push_verify (& mut self , _origin : SubregionOrigin < 'tcx > , kind : GenericKind < 'tcx > , a : ty :: Region < 'tcx > , bound : VerifyBound < 'tcx > ,) { let kind = self . replace_placeholders_with_nll (kind) ; let bound = self . replace_placeholders_with_nll (bound) ; let type_test = self . verify_to_type_test (kind , a , bound) ; self . add_type_test (type_test) ; } }}}