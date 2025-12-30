// Generated macro for NormalizedInputsAndOutput (type)
macro_rules! Depcrate_type_check_free_region_relationsNormalizedInputsAndOutput {
() => {
// Module: crate::type_check::free_region_relations
// Provides: {"NormalizedInputsAndOutput"}
// Dependencies: {}
# [doc = " As part of computing the free region relations, we also have to"] # [doc = " normalize the input-output types, which we then need later. So we"] # [doc = " return those. This vector consists of first the input types and"] # [doc = " then the output type as the last element."] type NormalizedInputsAndOutput < 'tcx > = Vec < Ty < 'tcx > > ;
};
}
