/* FP:reverse_sccs.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_region_infer_reverse_sccs_USE_0001
/* FP:reverse_sccs.rs-0002 */ use std :: ops :: Range ;
/* FP:reverse_sccs.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_region_infer_reverse_sccs_USE_0002
/* FP:reverse_sccs.rs-0004 */ use crate :: rustc_data_structures :: fx :: { FxIndexMap , FxIndexSet } ;
/* FP:reverse_sccs.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_region_infer_reverse_sccs_USE_0003
/* FP:reverse_sccs.rs-0006 */ use crate :: rustc_data_structures :: graph ;
/* FP:reverse_sccs.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_region_infer_reverse_sccs_USE_0004
/* FP:reverse_sccs.rs-0008 */ use crate :: rustc_data_structures :: graph :: vec_graph :: VecGraph ;
/* FP:reverse_sccs.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_region_infer_reverse_sccs_USE_0005
/* FP:reverse_sccs.rs-0010 */ use crate :: rustc_complete :: ty :: RegionVid ;
/* FP:reverse_sccs.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_region_infer_reverse_sccs_USE_0006
/* FP:reverse_sccs.rs-0012 */ use crate :: constraints :: ConstraintSccIndex ;
/* FP:reverse_sccs.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_region_infer_reverse_sccs_USE_0007
/* FP:reverse_sccs.rs-0014 */ use crate :: region_infer :: ConstraintSccs ;
/* FP:reverse_sccs.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_region_infer_reverse_sccs_USE_0008
/* FP:reverse_sccs.rs-0016 */ use crate :: universal_regions :: UniversalRegions ;
/* FP:reverse_sccs.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_region_infer_reverse_sccs_STRUCT_0009
/* FP:reverse_sccs.rs-0018 */ pub (crate) struct ReverseSccGraph { graph : VecGraph < ConstraintSccIndex > , # [doc = " For each SCC, the range of `universal_regions` that use that SCC as"] # [doc = " their value."] scc_regions : FxIndexMap < ConstraintSccIndex , Range < usize > > , # [doc = " All of the universal regions, in grouped so that `scc_regions` can"] # [doc = " index into here."] universal_regions : Vec < RegionVid > , }
/* FP:reverse_sccs.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_region_infer_reverse_sccs_IMPL_0010
/* FP:reverse_sccs.rs-0020 */ impl ReverseSccGraph { pub (super) fn compute (constraint_sccs : & ConstraintSccs , universal_regions : & UniversalRegions < '_ > ,) -> Self { let graph = constraint_sccs . reverse () ; let mut paired_scc_regions = universal_regions . universal_regions_iter () . map (| region | (constraint_sccs . scc (region) , region)) . collect :: < Vec < _ > > () ; paired_scc_regions . sort () ; let universal_regions = paired_scc_regions . iter () . map (| & (_ , region) | region) . collect () ; let mut scc_regions = FxIndexMap :: default () ; let mut start = 0 ; for chunk in paired_scc_regions . chunk_by (| & (scc1 , _) , & (scc2 , _) | scc1 == scc2) { let (scc , _) = chunk [0] ; scc_regions . insert (scc , start .. start + chunk . len ()) ; start += chunk . len () ; } ReverseSccGraph { graph , scc_regions , universal_regions } } # [doc = " Find all universal regions that are required to outlive the given SCC."] pub (super) fn upper_bounds (& self , scc0 : ConstraintSccIndex) -> impl Iterator < Item = RegionVid > { let mut duplicates = FxIndexSet :: default () ; graph :: depth_first_search (& self . graph , scc0) . flat_map (move | scc1 | { self . scc_regions . get (& scc1) . map_or (& [] [..] , | range | & self . universal_regions [range . clone ()]) }) . copied () . filter (move | r | duplicates . insert (* r)) } }