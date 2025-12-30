// Generated macro for impl_21 (impl)
macro_rules! Depcrate_covfunimpl_21 {
() => {
// Module: crate::covfun
// Provides: {"impl_21"}
// Dependencies: {}
impl MappingKind { fn for_each_term (& self , mut callback : impl FnMut (CovTerm)) { match * self { Self :: Code (term) => callback (term) , Self :: Gap (term) => callback (term) , Self :: Expansion (_id) => { } Self :: Skip => { } Self :: Branch { r#true , r#false } => { callback (r#true) ; callback (r#false) ; } Self :: MCDCBranch { r#true , r#false , condition_id : _ , true_next_id : _ , false_next_id : _ , } => { callback (r#true) ; callback (r#false) ; } Self :: MCDCDecision { bitmap_idx : _ , conditions_num : _ } => { } } } }
};
}
