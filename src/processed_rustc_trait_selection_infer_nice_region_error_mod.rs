/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_USE_0001
/* FP:mod.rs-0002 */ use crate :: rustc_complete :: { Diag , ErrorGuaranteed } ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_USE_0002
/* FP:mod.rs-0004 */ use crate :: rustc_complete :: def_id :: LocalDefId ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_USE_0003
/* FP:mod.rs-0006 */ use crate :: rustc_complete :: ty :: { self , TyCtxt } ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_USE_0004
/* FP:mod.rs-0008 */ use crate :: rustc_complete :: Span ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_USE_0005
/* FP:mod.rs-0010 */ use crate :: error_reporting :: TypeErrCtxt ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_USE_0006
/* FP:mod.rs-0012 */ use crate :: infer :: RegionResolutionError ;
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_MOD_0007
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_MOD_0008
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_MOD_0009
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_MOD_0010
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_MOD_0011
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_MOD_0012
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_MOD_0013
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_MOD_0014
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_MOD_0015
/* FP:mod.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_USE_0016
/* FP:mod.rs-0032 */ pub use different_lifetimes :: suggest_adding_lifetime_params ;
/* FP:mod.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_USE_0017
/* FP:mod.rs-0034 */ pub use find_anon_type :: find_anon_type ;
/* FP:mod.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_USE_0018
/* FP:mod.rs-0036 */ pub use static_impl_trait :: { HirTraitObjectVisitor , TraitObjectVisitor , suggest_new_region_bound } ;
/* FP:mod.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_USE_0019
/* FP:mod.rs-0038 */ pub use util :: find_param_with_region ;
/* FP:mod.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_IMPL_0020
/* FP:mod.rs-0040 */ impl < 'cx , 'tcx > TypeErrCtxt < 'cx , 'tcx > { pub fn try_report_nice_region_error (& 'cx self , generic_param_scope : LocalDefId , error : & RegionResolutionError < 'tcx > ,) -> Option < ErrorGuaranteed > { NiceRegionError :: new (self , generic_param_scope , error . clone ()) . try_report () } }
/* FP:mod.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_STRUCT_0021
/* FP:mod.rs-0042 */ pub struct NiceRegionError < 'cx , 'tcx > { cx : & 'cx TypeErrCtxt < 'cx , 'tcx > , # [doc = " The innermost definition that introduces generic parameters that may be involved in"] # [doc = " the region errors we are dealing with."] generic_param_scope : LocalDefId , error : Option < RegionResolutionError < 'tcx > > , regions : Option < (Span , ty :: Region < 'tcx > , ty :: Region < 'tcx >) > , }
/* FP:mod.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_nice_region_error_mod_IMPL_0022
/* FP:mod.rs-0044 */ impl < 'cx , 'tcx > NiceRegionError < 'cx , 'tcx > { pub fn new (cx : & 'cx TypeErrCtxt < 'cx , 'tcx > , generic_param_scope : LocalDefId , error : RegionResolutionError < 'tcx > ,) -> Self { Self { cx , error : Some (error) , regions : None , generic_param_scope } } pub fn new_from_span (cx : & 'cx TypeErrCtxt < 'cx , 'tcx > , generic_param_scope : LocalDefId , span : Span , sub : ty :: Region < 'tcx > , sup : ty :: Region < 'tcx > ,) -> Self { Self { cx , error : None , regions : Some ((span , sub , sup)) , generic_param_scope } } fn tcx (& self) -> TyCtxt < 'tcx > { self . cx . tcx } pub fn try_report_from_nll (& self) -> Option < Diag < 'tcx > > { self . try_report_named_anon_conflict () . or_else (| | self . try_report_placeholder_conflict ()) . or_else (| | self . try_report_placeholder_relation ()) } pub fn try_report (& self) -> Option < ErrorGuaranteed > { self . try_report_from_nll () . map (| diag | diag . emit ()) . or_else (| | self . try_report_impl_not_conforming_to_trait ()) . or_else (| | self . try_report_anon_anon_conflict ()) . or_else (| | self . try_report_static_impl_trait ()) . or_else (| | self . try_report_mismatched_static_lifetime ()) } pub (super) fn regions (& self) -> Option < (Span , ty :: Region < 'tcx > , ty :: Region < 'tcx >) > { match (& self . error , self . regions) { (Some (RegionResolutionError :: ConcreteFailure (origin , sub , sup)) , None) => { Some ((origin . span () , * sub , * sup)) } (Some (RegionResolutionError :: SubSupConflict (_ , _ , origin , sub , _ , sup , _)) , None) => { Some ((origin . span () , * sub , * sup)) } (None , Some ((span , sub , sup))) => Some ((span , sub , sup)) , _ => None , } } }