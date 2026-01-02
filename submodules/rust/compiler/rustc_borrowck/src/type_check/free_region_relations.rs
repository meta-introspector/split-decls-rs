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
mkuse!{use rustc_data_structures :: frozen :: Frozen ;}
mkuse!{use rustc_data_structures :: transitive_relation :: { TransitiveRelation , TransitiveRelationBuilder } ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_infer :: infer :: canonical :: QueryRegionConstraints ;}
mkuse!{use rustc_infer :: infer :: outlives ;}
mkuse!{use rustc_infer :: infer :: outlives :: env :: RegionBoundPairs ;}
mkuse!{use rustc_infer :: infer :: region_constraints :: GenericKind ;}
mkuse!{use rustc_infer :: traits :: query :: type_op :: DeeplyNormalize ;}
mkuse!{use rustc_middle :: mir :: ConstraintCategory ;}
mkuse!{use rustc_middle :: traits :: query :: OutlivesBound ;}
mkuse!{use rustc_middle :: ty :: { self , RegionVid , Ty , TypeVisitableExt } ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , Span } ;}
mkuse!{use rustc_trait_selection :: traits :: query :: type_op :: { self , TypeOp } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use type_op :: TypeOpOutput ;}
mkuse!{use crate :: BorrowckInferCtxt ;}
mkuse!{use crate :: type_check :: { Locations , MirTypeckRegionConstraints , constraint_conversion } ;}
mkuse!{use crate :: universal_regions :: UniversalRegions ;}
mkitem!{mkstruct!{# [derive (Debug)] # [derive (Clone)] pub (crate) struct UniversalRegionRelations < 'tcx > { pub (crate) universal_regions : UniversalRegions < 'tcx > , # [doc = " Stores the outlives relations that are known to hold from the"] # [doc = " implied bounds, in-scope where-clauses, and that sort of"] # [doc = " thing."] outlives : TransitiveRelation < RegionVid > , # [doc = " This is the `<=` relation; that is, if `a: b`, then `b <= a`,"] # [doc = " and we store that here. This is useful when figuring out how"] # [doc = " to express some local region in terms of external regions our"] # [doc = " caller will understand."] inverse_outlives : TransitiveRelation < RegionVid > , }}}
mkitem!{# [doc = " As part of computing the free region relations, we also have to"] # [doc = " normalize the input-output types, which we then need later. So we"] # [doc = " return those. This vector consists of first the input types and"] # [doc = " then the output type as the last element."] type NormalizedInputsAndOutput < 'tcx > = Vec < Ty < 'tcx > > ;}
mkitem!{mkstruct!{pub (crate) struct CreateResult < 'tcx > { pub (crate) universal_region_relations : Frozen < UniversalRegionRelations < 'tcx > > , pub (crate) region_bound_pairs : Frozen < RegionBoundPairs < 'tcx > > , pub (crate) known_type_outlives_obligations : Frozen < Vec < ty :: PolyTypeOutlivesPredicate < 'tcx > > > , pub (crate) normalized_inputs_and_output : NormalizedInputsAndOutput < 'tcx > , }}}

macro_rules! create_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create in module {}", module_path!());
    };
}

mkfn!{
    create_introspect!();
    pub (crate) fn create < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , universal_regions : UniversalRegions < 'tcx > , constraints : & mut MirTypeckRegionConstraints < 'tcx > ,) -> CreateResult < 'tcx > { UniversalRegionRelationsBuilder { infcx , constraints , universal_regions , region_bound_pairs : Default :: default () , outlives : Default :: default () , inverse_outlives : Default :: default () , } . create () }
}
mkitem!{mkimpl!{impl UniversalRegionRelations < '_ > { # [doc = " Given two universal regions, returns the postdominating"] # [doc = " upper-bound (effectively the least upper bound)."] # [doc = ""] # [doc = " (See `TransitiveRelation::postdom_upper_bound` for details on"] # [doc = " the postdominating upper bound in general.)"] pub (crate) fn postdom_upper_bound (& self , fr1 : RegionVid , fr2 : RegionVid) -> RegionVid { assert ! (self . universal_regions . is_universal_region (fr1)) ; assert ! (self . universal_regions . is_universal_region (fr2)) ; self . inverse_outlives . postdom_upper_bound (fr1 , fr2) . unwrap_or (self . universal_regions . fr_static) } # [doc = " Finds an \"upper bound\" for `fr` that is not local. In other"] # [doc = " words, returns the smallest (*) known region `fr1` that (a)"] # [doc = " outlives `fr` and (b) is not local."] # [doc = ""] # [doc = " (*) If there are multiple competing choices, we return all of them."] pub (crate) fn non_local_upper_bounds (& self , fr : RegionVid) -> Vec < RegionVid > { debug ! ("non_local_upper_bound(fr={:?})" , fr) ; let res = self . non_local_bounds (& self . inverse_outlives , fr) ; assert ! (! res . is_empty () , "can't find an upper bound!?") ; res } # [doc = " Finds a \"lower bound\" for `fr` that is not local. In other"] # [doc = " words, returns the largest (*) known region `fr1` that (a) is"] # [doc = " outlived by `fr` and (b) is not local."] # [doc = ""] # [doc = " (*) If there are multiple competing choices, we pick the \"postdominating\""] # [doc = " one. See `TransitiveRelation::postdom_upper_bound` for details."] pub (crate) fn non_local_lower_bound (& self , fr : RegionVid) -> Option < RegionVid > { debug ! ("non_local_lower_bound(fr={:?})" , fr) ; let lower_bounds = self . non_local_bounds (& self . outlives , fr) ; let post_dom = self . outlives . mutual_immediate_postdominator (lower_bounds) ; debug ! ("non_local_bound: post_dom={:?}" , post_dom) ; post_dom . and_then (| post_dom | { if ! self . universal_regions . is_local_free_region (post_dom) { Some (post_dom) } else { None } }) } # [doc = " Helper for `non_local_upper_bounds` and `non_local_lower_bounds`."] # [doc = " Repeatedly invokes `postdom_parent` until we find something that is not"] # [doc = " local. Returns `None` if we never do so."] fn non_local_bounds (& self , relation : & TransitiveRelation < RegionVid > , fr0 : RegionVid ,) -> Vec < RegionVid > { assert ! (self . universal_regions . is_universal_region (fr0)) ; let mut external_parents = vec ! [] ; let mut queue = vec ! [relation . minimal_scc_representative (fr0)] ; while let Some (fr) = queue . pop () { if ! self . universal_regions . is_local_free_region (fr) { external_parents . push (fr) ; continue ; } queue . extend (relation . parents (fr)) ; } debug ! ("non_local_bound: external_parents={:?}" , external_parents) ; external_parents } # [doc = " Returns `true` if fr1 is known to outlive fr2."] # [doc = ""] # [doc = " This will only ever be true for universally quantified regions."] pub (crate) fn outlives (& self , fr1 : RegionVid , fr2 : RegionVid) -> bool { self . outlives . contains (fr1 , fr2) } # [doc = " Returns `true` if fr1 is known to equal fr2."] # [doc = ""] # [doc = " This will only ever be true for universally quantified regions."] pub (crate) fn equal (& self , fr1 : RegionVid , fr2 : RegionVid) -> bool { self . outlives . contains (fr1 , fr2) && self . outlives . contains (fr2 , fr1) } # [doc = " Returns a vector of free regions `x` such that `fr1: x` is"] # [doc = " known to hold."] pub (crate) fn regions_outlived_by (& self , fr1 : RegionVid) -> Vec < RegionVid > { self . outlives . reachable_from (fr1) } # [doc = " Returns the _non-transitive_ set of known `outlives` constraints between free regions."] pub (crate) fn known_outlives (& self) -> impl Iterator < Item = (RegionVid , RegionVid) > { self . outlives . base_edges () } }}}
mkitem!{mkstruct!{struct UniversalRegionRelationsBuilder < 'a , 'tcx > { infcx : & 'a BorrowckInferCtxt < 'tcx > , universal_regions : UniversalRegions < 'tcx > , constraints : & 'a mut MirTypeckRegionConstraints < 'tcx > , outlives : TransitiveRelationBuilder < RegionVid > , inverse_outlives : TransitiveRelationBuilder < RegionVid > , region_bound_pairs : RegionBoundPairs < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > UniversalRegionRelationsBuilder < '_ , 'tcx > { # [doc = " Records in the `outlives_relation` (and"] # [doc = " `inverse_outlives_relation`) that `fr_a: fr_b`."] fn relate_universal_regions (& mut self , fr_a : RegionVid , fr_b : RegionVid) { debug ! ("relate_universal_regions: fr_a={:?} outlives fr_b={:?}" , fr_a , fr_b) ; self . outlives . add (fr_a , fr_b) ; self . inverse_outlives . add (fr_b , fr_a) ; } # [instrument (level = "debug" , skip (self))] pub (crate) fn create (mut self) -> CreateResult < 'tcx > { let tcx = self . infcx . tcx ; let defining_ty_def_id = self . universal_regions . defining_ty . def_id () . expect_local () ; let span = tcx . def_span (defining_ty_def_id) ; let param_env = self . infcx . param_env ; self . add_outlives_bounds (outlives :: explicit_outlives_bounds (param_env)) ; let fr_static = self . universal_regions . fr_static ; let fr_fn_body = self . universal_regions . fr_fn_body ; for fr in self . universal_regions . universal_regions_iter () { debug ! ("build: relating free region {:?} to itself and to 'static" , fr) ; self . relate_universal_regions (fr , fr) ; self . relate_universal_regions (fr_static , fr) ; self . relate_universal_regions (fr , fr_fn_body) ; } let mut constraints = vec ! [] ; let mut known_type_outlives_obligations = vec ! [] ; for bound in param_env . caller_bounds () { if let Some (outlives) = bound . as_type_outlives_clause () { self . normalize_and_push_type_outlives_obligation (outlives , span , & mut known_type_outlives_obligations , & mut constraints ,) ; } ; } let unnormalized_input_output_tys = self . universal_regions . unnormalized_input_tys . iter () . cloned () . chain (Some (self . universal_regions . unnormalized_output_ty)) ; let mut normalized_inputs_and_output = Vec :: with_capacity (self . universal_regions . unnormalized_input_tys . len () + 1) ; for ty in unnormalized_input_output_tys { debug ! ("build: input_or_output={:?}" , ty) ; let constraints_unnorm = self . add_implied_bounds (ty , span) ; if let Some (c) = constraints_unnorm { constraints . push (c) } let TypeOpOutput { output : norm_ty , constraints : constraints_normalize , .. } = param_env . and (DeeplyNormalize { value : ty }) . fully_perform (self . infcx , self . infcx . root_def_id , span) . unwrap_or_else (| guar | TypeOpOutput { output : Ty :: new_error (self . infcx . tcx , guar) , constraints : None , error_info : None , }) ; if let Some (c) = constraints_normalize { constraints . push (c) } if ty != norm_ty { let constraints_norm = self . add_implied_bounds (norm_ty , span) ; if let Some (c) = constraints_norm { constraints . push (c) } } normalized_inputs_and_output . push (norm_ty) ; } if matches ! (tcx . def_kind (defining_ty_def_id) , DefKind :: AssocFn | DefKind :: AssocConst) { for & (ty , _) in tcx . assumed_wf_types (tcx . local_parent (defining_ty_def_id)) { let result : Result < _ , ErrorGuaranteed > = param_env . and (DeeplyNormalize { value : ty }) . fully_perform (self . infcx , self . infcx . root_def_id , span) ; let Ok (TypeOpOutput { output : norm_ty , constraints : c , .. }) = result else { continue ; } ; constraints . extend (c) ; let c = self . add_implied_bounds (norm_ty , span) ; constraints . extend (c) ; } } for c in constraints { constraint_conversion :: ConstraintConversion :: new (self . infcx , & self . universal_regions , & self . region_bound_pairs , & known_type_outlives_obligations , Locations :: All (span) , span , ConstraintCategory :: Internal , self . constraints ,) . convert_all (c) ; } CreateResult { universal_region_relations : Frozen :: freeze (UniversalRegionRelations { universal_regions : self . universal_regions , outlives : self . outlives . freeze () , inverse_outlives : self . inverse_outlives . freeze () , }) , known_type_outlives_obligations : Frozen :: freeze (known_type_outlives_obligations) , region_bound_pairs : Frozen :: freeze (self . region_bound_pairs) , normalized_inputs_and_output , } } fn normalize_and_push_type_outlives_obligation (& self , mut outlives : ty :: PolyTypeOutlivesPredicate < 'tcx > , span : Span , known_type_outlives_obligations : & mut Vec < ty :: PolyTypeOutlivesPredicate < 'tcx > > , constraints : & mut Vec < & QueryRegionConstraints < 'tcx > > ,) { if self . infcx . next_trait_solver () { let Ok (TypeOpOutput { output : normalized_outlives , constraints : constraints_normalize , error_info : _ , }) = self . infcx . param_env . and (DeeplyNormalize { value : outlives }) . fully_perform (self . infcx , self . infcx . root_def_id , span ,) else { self . infcx . dcx () . delayed_bug (format ! ("could not normalize {outlives:?}")) ; return ; } ; outlives = normalized_outlives ; if let Some (c) = constraints_normalize { constraints . push (c) ; } } known_type_outlives_obligations . push (outlives) ; } # [doc = " Update the type of a single local, which should represent"] # [doc = " either the return type of the MIR or one of its arguments. At"] # [doc = " the same time, compute and add any implied bounds that come"] # [doc = " from this local."] # [instrument (level = "debug" , skip (self))] fn add_implied_bounds (& mut self , ty : Ty < 'tcx > , span : Span ,) -> Option < & 'tcx QueryRegionConstraints < 'tcx > > { let TypeOpOutput { output : bounds , constraints , .. } = self . infcx . param_env . and (type_op :: ImpliedOutlivesBounds { ty }) . fully_perform (self . infcx , self . infcx . root_def_id , span) . map_err (| _ : ErrorGuaranteed | debug ! ("failed to compute implied bounds {:?}" , ty)) . ok () ? ; debug ! (? bounds , ? constraints) ; let bounds = bounds . into_iter () . filter (| bound | ! bound . has_placeholders ()) ; self . add_outlives_bounds (bounds) ; constraints } # [doc = " Registers the `OutlivesBound` items from `outlives_bounds` in"] # [doc = " the outlives relation as well as the region-bound pairs"] # [doc = " listing."] fn add_outlives_bounds < I > (& mut self , outlives_bounds : I) where I : IntoIterator < Item = OutlivesBound < 'tcx > > , { for outlives_bound in outlives_bounds { debug ! ("add_outlives_bounds(bound={:?})" , outlives_bound) ; match outlives_bound { OutlivesBound :: RegionSubRegion (r1 , r2) => { let r1 = self . universal_regions . to_region_vid (r1) ; let r2 = self . universal_regions . to_region_vid (r2) ; self . relate_universal_regions (r2 , r1) ; } OutlivesBound :: RegionSubParam (r_a , param_b) => { self . region_bound_pairs . insert (ty :: OutlivesPredicate (GenericKind :: Param (param_b) , r_a)) ; } OutlivesBound :: RegionSubAlias (r_a , alias_b) => { self . region_bound_pairs . insert (ty :: OutlivesPredicate (GenericKind :: Alias (alias_b) , r_a)) ; } } } } }}}