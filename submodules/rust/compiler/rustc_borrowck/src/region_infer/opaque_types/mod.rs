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
mkuse!{use std :: iter ;}
mkuse!{use std :: rc :: Rc ;}
mkuse!{use rustc_data_structures :: frozen :: Frozen ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_infer :: infer :: outlives :: env :: RegionBoundPairs ;}
mkuse!{use rustc_infer :: infer :: { InferCtxt , NllRegionVariableOrigin , OpaqueTypeStorageEntries } ;}
mkuse!{use rustc_infer :: traits :: ObligationCause ;}
mkuse!{use rustc_macros :: extension ;}
mkuse!{use rustc_middle :: mir :: { Body , ConcreteOpaqueTypes , ConstraintCategory } ;}
mkuse!{use rustc_middle :: ty :: { self , DefiningScopeKind , EarlyBinder , FallibleTypeFolder , GenericArg , GenericArgsRef , OpaqueHiddenType , OpaqueTypeKey , Region , RegionVid , Ty , TyCtxt , TypeFoldable , TypeSuperFoldable , TypeVisitableExt , fold_regions , } ;}
mkuse!{use rustc_mir_dataflow :: points :: DenseLocationMap ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_trait_selection :: opaque_types :: { NonDefiningUseReason , opaque_type_has_defining_use_args , } ;}
mkuse!{use rustc_trait_selection :: solve :: NoSolution ;}
mkuse!{use rustc_trait_selection :: traits :: query :: type_op :: custom :: CustomTypeOp ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: reverse_sccs :: ReverseSccGraph ;}
mkuse!{use crate :: BorrowckInferCtxt ;}
mkuse!{use crate :: consumers :: RegionInferenceContext ;}
mkuse!{use crate :: session_diagnostics :: LifetimeMismatchOpaqueParam ;}
mkuse!{use crate :: type_check :: canonical :: fully_perform_op_raw ;}
mkuse!{use crate :: type_check :: free_region_relations :: UniversalRegionRelations ;}
mkuse!{use crate :: type_check :: { Locations , MirTypeckRegionConstraints } ;}
mkuse!{use crate :: universal_regions :: { RegionClassification , UniversalRegions } ;}
mkmod!{member_constraints, { 
                getname!(member_constraints);
                getsrc!(member_constraints);
                getpath!(member_constraints);
                get_deps!(member_constraints);
                get_crates!(member_constraints);
                mkinclude!(member_constraints);
                 
            }}
mkmod!{region_ctxt, { 
                getname!(region_ctxt);
                getsrc!(region_ctxt);
                getpath!(region_ctxt);
                get_deps!(region_ctxt);
                get_crates!(region_ctxt);
                mkinclude!(region_ctxt);
                 
            }}
mkuse!{use member_constraints :: apply_member_constraints ;}
mkuse!{use region_ctxt :: RegionCtxt ;}
mkitem!{mkenum!{# [doc = " We defer errors from [fn handle_opaque_type_uses] and only report them"] # [doc = " if there are no `RegionErrors`. If there are region errors, it's likely"] # [doc = " that errors here are caused by them and don't need to be handled separately."] pub (crate) enum DeferredOpaqueTypeError < 'tcx > { InvalidOpaqueTypeArgs (NonDefiningUseReason < 'tcx >) , LifetimeMismatchOpaqueParam (LifetimeMismatchOpaqueParam < 'tcx >) , UnexpectedHiddenRegion { # [doc = " The opaque type."] opaque_type_key : OpaqueTypeKey < 'tcx > , # [doc = " The hidden type containing the member region."] hidden_type : OpaqueHiddenType < 'tcx > , # [doc = " The unexpected region."] member_region : Region < 'tcx > , } , NonDefiningUseInDefiningScope { span : Span , opaque_type_key : OpaqueTypeKey < 'tcx > , } , }}}

macro_rules! clone_and_resolve_opaque_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clone_and_resolve_opaque_types in module {}", module_path!());
    };
}

mkfn!{
    clone_and_resolve_opaque_types_introspect!();
    # [doc = " We eagerly map all regions to NLL vars here, as we need to make sure we've"] # [doc = " introduced nll vars for all used placeholders."] # [doc = ""] # [doc = " We need to resolve inference vars as even though we're in MIR typeck, we may still"] # [doc = " encounter inference variables, e.g. when checking user types."] pub (crate) fn clone_and_resolve_opaque_types < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , universal_region_relations : & Frozen < UniversalRegionRelations < 'tcx > > , constraints : & mut MirTypeckRegionConstraints < 'tcx > ,) -> (OpaqueTypeStorageEntries , Vec < (OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >) >) { let opaque_types = infcx . clone_opaque_types () ; let opaque_types_storage_num_entries = infcx . inner . borrow_mut () . opaque_types () . num_entries () ; let opaque_types = opaque_types . into_iter () . map (| entry | { fold_regions (infcx . tcx , infcx . resolve_vars_if_possible (entry) , | r , _ | { let vid = if let ty :: RePlaceholder (placeholder) = r . kind () { constraints . placeholder_region (infcx , placeholder) . as_var () } else { universal_region_relations . universal_regions . to_region_vid (r) } ; Region :: new_var (infcx . tcx , vid) }) }) . collect :: < Vec < _ > > () ; (opaque_types_storage_num_entries , opaque_types) }
}

macro_rules! nll_var_to_universal_region_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nll_var_to_universal_region in module {}", module_path!());
    };
}

mkfn!{
    nll_var_to_universal_region_introspect!();
    # [doc = " Maps an NLL var to a deterministically chosen equal universal region."] # [doc = ""] # [doc = " See the corresponding [rustc-dev-guide chapter] for more details. This"] # [doc = " ignores changes to the region values due to member constraints. Applying"] # [doc = " member constraints does not impact the result of this function."] # [doc = ""] # [doc = " [rustc-dev-guide chapter]: https://rustc-dev-guide.rust-lang.org/borrow_check/opaque-types-region-inference-restrictions.html"] fn nll_var_to_universal_region < 'tcx > (rcx : & RegionCtxt < '_ , 'tcx > , r : RegionVid ,) -> Option < Region < 'tcx > > { let vid = rcx . representative (r) . rvid () ; match rcx . definitions [vid] . origin { NllRegionVariableOrigin :: FreeRegion => rcx . universal_regions () . universal_regions_iter () . filter (| & ur | { ! matches ! (rcx . universal_regions () . region_classification (ur) , Some (RegionClassification :: External)) }) . find (| & ur | rcx . universal_region_relations . equal (vid , ur)) . map (| ur | rcx . definitions [ur] . external_name . unwrap ()) , NllRegionVariableOrigin :: Placeholder (placeholder) => { Some (ty :: Region :: new_placeholder (rcx . infcx . tcx , placeholder)) } NllRegionVariableOrigin :: Existential { .. } => None , } }
}

macro_rules! add_concrete_opaque_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_concrete_opaque_type in module {}", module_path!());
    };
}

mkfn!{
    add_concrete_opaque_type_introspect!();
    # [doc = " Collect all defining uses of opaque types inside of this typeck root. This"] # [doc = " expects the hidden type to be mapped to the definition parameters of the opaque"] # [doc = " and errors if we end up with distinct hidden types."] fn add_concrete_opaque_type < 'tcx > (tcx : TyCtxt < 'tcx > , concrete_opaque_types : & mut ConcreteOpaqueTypes < 'tcx > , def_id : LocalDefId , hidden_ty : OpaqueHiddenType < 'tcx > ,) { if let Some (prev) = concrete_opaque_types . 0 . get_mut (& def_id) { if prev . ty != hidden_ty . ty { let guar = hidden_ty . ty . error_reported () . err () . unwrap_or_else (| | { let (Ok (e) | Err (e)) = prev . build_mismatch_error (& hidden_ty , tcx) . map (| d | d . emit ()) ; e }) ; prev . ty = Ty :: new_error (tcx , guar) ; } prev . span = prev . span . substitute_dummy (hidden_ty . span) ; } else { concrete_opaque_types . 0 . insert (def_id , hidden_ty) ; } }
}

macro_rules! get_concrete_opaque_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_concrete_opaque_type in module {}", module_path!());
    };
}

mkfn!{
    get_concrete_opaque_type_introspect!();
    fn get_concrete_opaque_type < 'tcx > (concrete_opaque_types : & ConcreteOpaqueTypes < 'tcx > , def_id : LocalDefId ,) -> Option < EarlyBinder < 'tcx , OpaqueHiddenType < 'tcx > > > { concrete_opaque_types . 0 . get (& def_id) . map (| ty | EarlyBinder :: bind (* ty)) }
}
mkitem!{mkstruct!{# [derive (Debug)] struct DefiningUse < 'tcx > { # [doc = " The opaque type using non NLL vars. This uses the actual"] # [doc = " free regions and placeholders. This is necessary"] # [doc = " to interact with code outside of `rustc_borrowck`."] opaque_type_key : OpaqueTypeKey < 'tcx > , arg_regions : Vec < RegionVid > , hidden_type : OpaqueHiddenType < 'tcx > , }}}

macro_rules! compute_concrete_opaque_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_concrete_opaque_types in module {}", module_path!());
    };
}

mkfn!{
    compute_concrete_opaque_types_introspect!();
    # [doc = " This computes the actual hidden types of the opaque types and maps them to their"] # [doc = " definition sites. Outside of registering the computed concrete types this function"] # [doc = " does not mutate the current borrowck state."] # [doc = ""] # [doc = " While it may fail to infer the hidden type and return errors, we always apply"] # [doc = " the computed concrete hidden type to all opaque type uses to check whether they"] # [doc = " are correct. This is necessary to support non-defining uses of opaques in their"] # [doc = " defining scope."] # [doc = ""] # [doc = " It also means that this whole function is not really soundness critical as we"] # [doc = " recheck all uses of the opaques regardless."] pub (crate) fn compute_concrete_opaque_types < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , universal_region_relations : & Frozen < UniversalRegionRelations < 'tcx > > , constraints : & MirTypeckRegionConstraints < 'tcx > , location_map : Rc < DenseLocationMap > , concrete_opaque_types : & mut ConcreteOpaqueTypes < 'tcx > , opaque_types : & [(OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >)] ,) -> Vec < DeferredOpaqueTypeError < 'tcx > > { let mut errors = Vec :: new () ; let mut rcx = RegionCtxt :: new (infcx , universal_region_relations , location_map , constraints) ; let defining_uses = collect_defining_uses (& mut rcx , concrete_opaque_types , opaque_types , & mut errors) ; apply_member_constraints (& mut rcx , & defining_uses) ; compute_concrete_types_from_defining_uses (& rcx , concrete_opaque_types , & defining_uses , & mut errors ,) ; errors }
}

macro_rules! collect_defining_uses_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_defining_uses in module {}", module_path!());
    };
}

mkfn!{
    collect_defining_uses_introspect!();
    # [instrument (level = "debug" , skip_all , ret)] fn collect_defining_uses < 'tcx > (rcx : & mut RegionCtxt < '_ , 'tcx > , concrete_opaque_types : & mut ConcreteOpaqueTypes < 'tcx > , opaque_types : & [(OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >)] , errors : & mut Vec < DeferredOpaqueTypeError < 'tcx > > ,) -> Vec < DefiningUse < 'tcx > > { let infcx = rcx . infcx ; let mut defining_uses = vec ! [] ; for & (opaque_type_key , hidden_type) in opaque_types { let non_nll_opaque_type_key = opaque_type_key . fold_captured_lifetime_args (infcx . tcx , | r | { nll_var_to_universal_region (& rcx , r . as_var ()) . unwrap_or (r) }) ; if let Err (err) = opaque_type_has_defining_use_args (infcx , non_nll_opaque_type_key , hidden_type . span , DefiningScopeKind :: MirBorrowck ,) { if infcx . tcx . use_typing_mode_borrowck () { match err { NonDefiningUseReason :: Tainted (guar) => add_concrete_opaque_type (infcx . tcx , concrete_opaque_types , opaque_type_key . def_id , OpaqueHiddenType :: new_error (infcx . tcx , guar) ,) , _ => debug ! (? non_nll_opaque_type_key , ? err , "ignoring non-defining use") , } } else { errors . push (DeferredOpaqueTypeError :: InvalidOpaqueTypeArgs (err)) ; debug ! ("collect_defining_uses: InvalidOpaqueTypeArgs for {:?} := {:?}" , non_nll_opaque_type_key , hidden_type) ; } continue ; } let arg_regions = iter :: once (rcx . universal_regions () . fr_static) . chain (opaque_type_key . iter_captured_args (infcx . tcx) . filter_map (| (_ , arg) | arg . as_region ()) . map (Region :: as_var) ,) . collect () ; defining_uses . push (DefiningUse { opaque_type_key : non_nll_opaque_type_key , arg_regions , hidden_type , }) ; } defining_uses }
}

macro_rules! compute_concrete_types_from_defining_uses_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_concrete_types_from_defining_uses in module {}", module_path!());
    };
}

mkfn!{
    compute_concrete_types_from_defining_uses_introspect!();
    # [instrument (level = "debug" , skip (rcx , concrete_opaque_types , defining_uses , errors))] fn compute_concrete_types_from_defining_uses < 'tcx > (rcx : & RegionCtxt < '_ , 'tcx > , concrete_opaque_types : & mut ConcreteOpaqueTypes < 'tcx > , defining_uses : & [DefiningUse < 'tcx >] , errors : & mut Vec < DeferredOpaqueTypeError < 'tcx > > ,) { let infcx = rcx . infcx ; let tcx = infcx . tcx ; let mut decls_modulo_regions : FxIndexMap < OpaqueTypeKey < 'tcx > , (OpaqueTypeKey < 'tcx > , Span) > = FxIndexMap :: default () ; for & DefiningUse { opaque_type_key , ref arg_regions , hidden_type } in defining_uses { debug ! (? opaque_type_key , ? arg_regions , ? hidden_type) ; let hidden_type = match hidden_type . try_fold_with (& mut ToArgRegionsFolder :: new (rcx , arg_regions)) { Ok (hidden_type) => hidden_type , Err (r) => { debug ! ("UnexpectedHiddenRegion: {:?}" , r) ; errors . push (DeferredOpaqueTypeError :: UnexpectedHiddenRegion { hidden_type , opaque_type_key , member_region : ty :: Region :: new_var (tcx , r) , }) ; let guar = tcx . dcx () . span_delayed_bug (hidden_type . span , "opaque type with non-universal region args" ,) ; ty :: OpaqueHiddenType :: new_error (tcx , guar) } } ; let ty = infcx . infer_opaque_definition_from_instantiation (opaque_type_key , hidden_type) . unwrap_or_else (| _ | { Ty :: new_error_with_message (rcx . infcx . tcx , hidden_type . span , "deferred invalid opaque type args" ,) }) ; if ! rcx . infcx . tcx . use_typing_mode_borrowck () { if let ty :: Alias (ty :: Opaque , alias_ty) = ty . kind () && alias_ty . def_id == opaque_type_key . def_id . to_def_id () && alias_ty . args == opaque_type_key . args { continue ; } } if let Some ((prev_decl_key , prev_span)) = decls_modulo_regions . insert (rcx . infcx . tcx . erase_and_anonymize_regions (opaque_type_key) , (opaque_type_key , hidden_type . span) ,) && let Some ((arg1 , arg2)) = std :: iter :: zip (prev_decl_key . iter_captured_args (infcx . tcx) . map (| (_ , arg) | arg) , opaque_type_key . iter_captured_args (infcx . tcx) . map (| (_ , arg) | arg) ,) . find (| (arg1 , arg2) | arg1 != arg2) { errors . push (DeferredOpaqueTypeError :: LifetimeMismatchOpaqueParam (LifetimeMismatchOpaqueParam { arg : arg1 , prev : arg2 , span : prev_span , prev_span : hidden_type . span , } ,)) ; } add_concrete_opaque_type (tcx , concrete_opaque_types , opaque_type_key . def_id , OpaqueHiddenType { span : hidden_type . span , ty } ,) ; } }
}
mkitem!{mkstruct!{# [doc = " A folder to map the regions in the hidden type to their corresponding `arg_regions`."] # [doc = ""] # [doc = " This folder has to differentiate between member regions and other regions in the hidden"] # [doc = " type. Member regions have to be equal to one of the `arg_regions` while other regions simply"] # [doc = " get treated as an existential region in the opaque if they are not. Existential"] # [doc = " regions are currently represented using `'erased`."] struct ToArgRegionsFolder < 'a , 'tcx > { rcx : & 'a RegionCtxt < 'a , 'tcx > , erase_unknown_regions : bool , arg_regions : & 'a [RegionVid] , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > ToArgRegionsFolder < 'a , 'tcx > { fn new (rcx : & 'a RegionCtxt < 'a , 'tcx > , arg_regions : & 'a [RegionVid] ,) -> ToArgRegionsFolder < 'a , 'tcx > { ToArgRegionsFolder { rcx , erase_unknown_regions : false , arg_regions } } fn fold_non_member_arg (& mut self , arg : GenericArg < 'tcx >) -> GenericArg < 'tcx > { let prev = self . erase_unknown_regions ; self . erase_unknown_regions = true ; let res = arg . try_fold_with (self) . unwrap () ; self . erase_unknown_regions = prev ; res } fn fold_closure_args (& mut self , def_id : DefId , args : GenericArgsRef < 'tcx > ,) -> Result < GenericArgsRef < 'tcx > , RegionVid > { let generics = self . cx () . generics_of (def_id) ; self . cx () . mk_args_from_iter (args . iter () . enumerate () . map (| (index , arg) | { if index < generics . parent_count { Ok (self . fold_non_member_arg (arg)) } else { arg . try_fold_with (self) } })) } }}}
mkitem!{mkimpl!{impl < 'tcx > FallibleTypeFolder < TyCtxt < 'tcx > > for ToArgRegionsFolder < '_ , 'tcx > { type Error = RegionVid ; fn cx (& self) -> TyCtxt < 'tcx > { self . rcx . infcx . tcx } fn try_fold_region (& mut self , r : Region < 'tcx >) -> Result < Region < 'tcx > , RegionVid > { match r . kind () { ty :: ReBound (_ , _) => Ok (r) , _ => { let r = r . as_var () ; if let Some (arg_region) = self . arg_regions . iter () . copied () . find (| & arg_vid | self . rcx . eval_equal (r , arg_vid)) . and_then (| r | nll_var_to_universal_region (self . rcx , r)) { Ok (arg_region) } else if self . erase_unknown_regions { Ok (self . cx () . lifetimes . re_erased) } else { Err (r) } } } } fn try_fold_ty (& mut self , ty : Ty < 'tcx >) -> Result < Ty < 'tcx > , RegionVid > { if ! ty . flags () . intersects (ty :: TypeFlags :: HAS_FREE_REGIONS) { return Ok (ty) ; } let tcx = self . cx () ; Ok (match * ty . kind () { ty :: Closure (def_id , args) => { Ty :: new_closure (tcx , def_id , self . fold_closure_args (def_id , args) ?) } ty :: CoroutineClosure (def_id , args) => { Ty :: new_coroutine_closure (tcx , def_id , self . fold_closure_args (def_id , args) ?) } ty :: Coroutine (def_id , args) => { Ty :: new_coroutine (tcx , def_id , self . fold_closure_args (def_id , args) ?) } ty :: Alias (kind , ty :: AliasTy { def_id , args , .. }) if let Some (variances) = tcx . opt_alias_variances (kind , def_id) => { let args = tcx . mk_args_from_iter (std :: iter :: zip (variances , args . iter ()) . map (| (& v , s) | { if v == ty :: Bivariant { Ok (self . fold_non_member_arg (s)) } else { s . try_fold_with (self) } } ,)) ? ; ty :: AliasTy :: new_from_args (tcx , def_id , args) . to_ty (tcx) } _ => ty . try_super_fold_with (self) ? , }) } }}}

macro_rules! apply_computed_concrete_opaque_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function apply_computed_concrete_opaque_types in module {}", module_path!());
    };
}

mkfn!{
    apply_computed_concrete_opaque_types_introspect!();
    # [doc = " This function is what actually applies member constraints to the borrowck"] # [doc = " state. It is also responsible to check all uses of the opaques in their"] # [doc = " defining scope."] # [doc = ""] # [doc = " It does this by equating the hidden type of each use with the instantiated final"] # [doc = " hidden type of the opaque."] pub (crate) fn apply_computed_concrete_opaque_types < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , body : & Body < 'tcx > , universal_regions : & UniversalRegions < 'tcx > , region_bound_pairs : & RegionBoundPairs < 'tcx > , known_type_outlives_obligations : & [ty :: PolyTypeOutlivesPredicate < 'tcx >] , constraints : & mut MirTypeckRegionConstraints < 'tcx > , concrete_opaque_types : & mut ConcreteOpaqueTypes < 'tcx > , opaque_types : & [(OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >)] ,) -> Vec < DeferredOpaqueTypeError < 'tcx > > { let tcx = infcx . tcx ; let mut errors = Vec :: new () ; for & (key , hidden_type) in opaque_types { let Some (expected) = get_concrete_opaque_type (concrete_opaque_types , key . def_id) else { if ! tcx . use_typing_mode_borrowck () { if let ty :: Alias (ty :: Opaque , alias_ty) = hidden_type . ty . kind () && alias_ty . def_id == key . def_id . to_def_id () && alias_ty . args == key . args { continue ; } else { unreachable ! ("non-defining use in defining scope") ; } } errors . push (DeferredOpaqueTypeError :: NonDefiningUseInDefiningScope { span : hidden_type . span , opaque_type_key : key , }) ; let guar = tcx . dcx () . span_delayed_bug (hidden_type . span , "non-defining use in the defining scope with no defining uses" ,) ; add_concrete_opaque_type (tcx , concrete_opaque_types , key . def_id , OpaqueHiddenType :: new_error (tcx , guar) ,) ; continue ; } ; let expected = ty :: fold_regions (tcx , expected . instantiate (tcx , key . args) , | re , _dbi | { match re . kind () { ty :: ReErased => infcx . next_nll_region_var (NllRegionVariableOrigin :: Existential { name : None } , | | crate :: RegionCtxt :: Existential (None) ,) , _ => re , } }) ; let locations = Locations :: All (hidden_type . span) ; if let Err (guar) = fully_perform_op_raw (infcx , body , universal_regions , region_bound_pairs , known_type_outlives_obligations , constraints , locations , ConstraintCategory :: OpaqueType , CustomTypeOp :: new (| ocx | { let cause = ObligationCause :: misc (hidden_type . span , body . source . def_id () . expect_local () ,) ; let actual_ty = ocx . normalize (& cause , infcx . param_env , hidden_type . ty) ; let expected_ty = ocx . normalize (& cause , infcx . param_env , expected . ty) ; ocx . eq (& cause , infcx . param_env , actual_ty , expected_ty) . map_err (| _ | NoSolution) } , "equating opaque types" ,) ,) { add_concrete_opaque_type (tcx , concrete_opaque_types , key . def_id , OpaqueHiddenType :: new_error (tcx , guar) ,) ; } } errors }
}

macro_rules! detect_opaque_types_added_while_handling_opaque_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_opaque_types_added_while_handling_opaque_types in module {}", module_path!());
    };
}

mkfn!{
    detect_opaque_types_added_while_handling_opaque_types_introspect!();
    # [doc = " In theory `apply_concrete_opaque_types` could introduce new uses of opaque types."] # [doc = " We do not check these new uses so this could be unsound."] # [doc = ""] # [doc = " We detect any new uses and simply delay a bug if they occur. If this results in"] # [doc = " an ICE we can properly handle this, but we haven't encountered any such test yet."] # [doc = ""] # [doc = " See the related comment in `FnCtxt::detect_opaque_types_added_during_writeback`."] pub (crate) fn detect_opaque_types_added_while_handling_opaque_types < 'tcx > (infcx : & InferCtxt < 'tcx > , opaque_types_storage_num_entries : OpaqueTypeStorageEntries ,) { for (key , hidden_type) in infcx . inner . borrow_mut () . opaque_types () . opaque_types_added_since (opaque_types_storage_num_entries) { let opaque_type_string = infcx . tcx . def_path_str (key . def_id) ; let msg = format ! ("unexpected cyclic definition of `{opaque_type_string}`") ; infcx . dcx () . span_delayed_bug (hidden_type . span , msg) ; } let _ = infcx . take_opaque_types () ; }
}
mkitem!{mkimpl!{impl < 'tcx > RegionInferenceContext < 'tcx > { # [doc = " Map the regions in the type to named regions. This is similar to what"] # [doc = " `infer_opaque_types` does, but can infer any universal region, not only"] # [doc = " ones from the args for the opaque type. It also doesn't double check"] # [doc = " that the regions produced are in fact equal to the named region they are"] # [doc = " replaced with. This is fine because this function is only to improve the"] # [doc = " region names in error messages."] # [doc = ""] # [doc = " This differs from `MirBorrowckCtxt::name_regions` since it is particularly"] # [doc = " lax with mapping region vids that are *shorter* than a universal region to"] # [doc = " that universal region. This is useful for member region constraints since"] # [doc = " we want to suggest a universal region name to capture even if it's technically"] # [doc = " not equal to the error region."] pub (crate) fn name_regions_for_member_constraint < T > (& self , tcx : TyCtxt < 'tcx > , ty : T) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { fold_regions (tcx , ty , | region , _ | match region . kind () { ty :: ReVar (vid) => { let scc = self . constraint_sccs . scc (vid) ; if ! self . max_nameable_universe (scc) . is_root () { match self . scc_values . placeholders_contained_in (scc) . enumerate () . last () { Some ((0 , placeholder)) => { return ty :: Region :: new_placeholder (tcx , placeholder) ; } _ => return region , } } let upper_bound = self . approx_universal_upper_bound (vid) ; if let Some (universal_region) = self . definitions [upper_bound] . external_name { return universal_region ; } let scc = self . constraint_sccs . scc (vid) ; let rev_scc_graph = ReverseSccGraph :: compute (& self . constraint_sccs , self . universal_regions ()) ; let upper_bounds : Vec < _ > = rev_scc_graph . upper_bounds (scc) . filter_map (| vid | self . definitions [vid] . external_name) . filter (| r | ! r . is_static ()) . collect () ; match & upper_bounds [..] { [universal_region] => * universal_region , _ => region , } } _ => region , }) } }}}
mkitem!{mkimpl!{# [extension (pub trait InferCtxtExt <'tcx >)] impl < 'tcx > InferCtxt < 'tcx > { # [doc = " Given the fully resolved, instantiated type for an opaque"] # [doc = " type, i.e., the value of an inference variable like C1 or C2"] # [doc = " (*), computes the \"definition type\" for an opaque type"] # [doc = " definition -- that is, the inferred value of `Foo1<'x>` or"] # [doc = " `Foo2<'x>` that we would conceptually use in its definition:"] # [doc = " ```ignore (illustrative)"] # [doc = " type Foo1<'x> = impl Bar<'x> = AAA;  // <-- this type AAA"] # [doc = " type Foo2<'x> = impl Bar<'x> = BBB;  // <-- or this type BBB"] # [doc = " fn foo<'a, 'b>(..) -> (Foo1<'a>, Foo2<'b>) { .. }"] # [doc = " ```"] # [doc = " Note that these values are defined in terms of a distinct set of"] # [doc = " generic parameters (`'x` instead of `'a`) from C1 or C2. The main"] # [doc = " purpose of this function is to do that translation."] # [doc = ""] # [doc = " (*) C1 and C2 were introduced in the comments on"] # [doc = " `register_member_constraints`. Read that comment for more context."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " - `def_id`, the `impl Trait` type"] # [doc = " - `args`, the args used to instantiate this opaque type"] # [doc = " - `instantiated_ty`, the inferred type C1 -- fully resolved, lifted version of"] # [doc = "   `opaque_defn.concrete_ty`"] # [instrument (level = "debug" , skip (self))] fn infer_opaque_definition_from_instantiation (& self , opaque_type_key : OpaqueTypeKey < 'tcx > , instantiated_ty : OpaqueHiddenType < 'tcx > ,) -> Result < Ty < 'tcx > , NonDefiningUseReason < 'tcx > > { opaque_type_has_defining_use_args (self , opaque_type_key , instantiated_ty . span , DefiningScopeKind :: MirBorrowck ,) ? ; let definition_ty = instantiated_ty . remap_generic_params_to_declaration_params (opaque_type_key , self . tcx , DefiningScopeKind :: MirBorrowck ,) . ty ; definition_ty . error_reported () ? ; Ok (definition_ty) } }}}