// Generated macro for arms_contain_ref_bindings (function)
macro_rules! Depcrate__matcharms_contain_ref_bindings {
() => {
// Module: crate::_match
// Provides: {"arms_contain_ref_bindings"}
// Dependencies: {}
fn arms_contain_ref_bindings < 'tcx > (arms : & 'tcx [hir :: Arm < 'tcx >]) -> Option < hir :: Mutability > { arms . iter () . filter_map (| a | a . pat . contains_explicit_ref_binding ()) . max () }
};
}
