/* FP:mappings.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_USE_0001
/* FP:mappings.rs-0002 */ use crate :: rustc_index :: IndexVec ;
/* FP:mappings.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_USE_0002
/* FP:mappings.rs-0004 */ use crate :: rustc_complete :: mir :: coverage :: { BlockMarkerId , BranchSpan , CoverageInfoHi , CoverageKind , Mapping , MappingKind , } ;
/* FP:mappings.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_USE_0003
/* FP:mappings.rs-0006 */ use crate :: rustc_complete :: mir :: { self , BasicBlock , StatementKind } ;
/* FP:mappings.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_USE_0004
/* FP:mappings.rs-0008 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:mappings.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_USE_0005
/* FP:mappings.rs-0010 */ use crate :: coverage :: graph :: CoverageGraph ;
/* FP:mappings.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_USE_0006
/* FP:mappings.rs-0012 */ use crate :: coverage :: hir_info :: ExtractedHirInfo ;
/* FP:mappings.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_USE_0007
/* FP:mappings.rs-0014 */ use crate :: coverage :: spans :: extract_refined_covspans ;
/* FP:mappings.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_USE_0008
/* FP:mappings.rs-0016 */ use crate :: coverage :: unexpand :: unexpand_into_body_span ;
/* FP:mappings.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_STRUCT_0009
/* FP:mappings.rs-0018 */ # [derive (Default)] pub (crate) struct ExtractedMappings { pub (crate) mappings : Vec < Mapping > , }
/* FP:mappings.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_FN_0010
/* FP:mappings.rs-0020 */ # [doc = " Extracts coverage-relevant spans from MIR, and uses them to create"] # [doc = " coverage mapping data for inclusion in MIR."] pub (crate) fn extract_mappings_from_mir < 'tcx > (tcx : TyCtxt < 'tcx > , mir_body : & mir :: Body < 'tcx > , hir_info : & ExtractedHirInfo , graph : & CoverageGraph ,) -> ExtractedMappings { let mut mappings = vec ! [] ; extract_refined_covspans (tcx , mir_body , hir_info , graph , & mut mappings) ; extract_branch_mappings (mir_body , hir_info , graph , & mut mappings) ; ExtractedMappings { mappings } }
/* FP:mappings.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_FN_0011
/* FP:mappings.rs-0022 */ fn resolve_block_markers (coverage_info_hi : & CoverageInfoHi , mir_body : & mir :: Body < '_ > ,) -> IndexVec < BlockMarkerId , Option < BasicBlock > > { let mut block_markers = IndexVec :: < BlockMarkerId , Option < BasicBlock > > :: from_elem_n (None , coverage_info_hi . num_block_markers ,) ; for (bb , data) in mir_body . basic_blocks . iter_enumerated () { for statement in & data . statements { if let StatementKind :: Coverage (CoverageKind :: BlockMarker { id }) = statement . kind { block_markers [id] = Some (bb) ; } } } block_markers }
/* FP:mappings.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_mappings_FN_0012
/* FP:mappings.rs-0024 */ pub (super) fn extract_branch_mappings (mir_body : & mir :: Body < '_ > , hir_info : & ExtractedHirInfo , graph : & CoverageGraph , mappings : & mut Vec < Mapping > ,) { let Some (coverage_info_hi) = mir_body . coverage_info_hi . as_deref () else { return } ; let block_markers = resolve_block_markers (coverage_info_hi , mir_body) ; mappings . extend (coverage_info_hi . branch_spans . iter () . filter_map (| & BranchSpan { span : raw_span , true_marker , false_marker } | try { if ! raw_span . ctxt () . outer_expn_data () . is_root () { return None ; } let span = unexpand_into_body_span (raw_span , hir_info . body_span) ? ; let bcb_from_marker = | marker : BlockMarkerId | graph . bcb_from_bb (block_markers [marker] ?) ; let true_bcb = bcb_from_marker (true_marker) ? ; let false_bcb = bcb_from_marker (false_marker) ? ; Mapping { span , kind : MappingKind :: Branch { true_bcb , false_bcb } } } ,)) ; }