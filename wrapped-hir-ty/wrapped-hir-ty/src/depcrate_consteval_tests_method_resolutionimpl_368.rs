// Generated macro for impl_368 (impl)
macro_rules! Depcrate_consteval_tests_method_resolutionimpl_368 {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"impl_368"}
// Dependencies: {}
impl CandidateId { fn container (self , db : & dyn HirDatabase) -> ItemContainerId { match self { CandidateId :: FunctionId (id) => id . loc (db) . container , CandidateId :: ConstId (id) => id . loc (db) . container , } } }
};
}
