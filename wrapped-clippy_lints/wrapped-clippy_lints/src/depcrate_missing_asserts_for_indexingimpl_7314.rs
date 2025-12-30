// Generated macro for impl_7314 (impl)
macro_rules! Depcrate_missing_asserts_for_indexingimpl_7314 {
() => {
// Module: crate::missing_asserts_for_indexing
// Provides: {"impl_7314"}
// Dependencies: {}
impl LateLintPass < '_ > for MissingAssertsForIndexing { fn check_body (& mut self , cx : & LateContext < '_ > , body : & Body < '_ >) { let mut map = UnindexMap :: default () ; for_each_expr_without_closures (body . value , | expr | { check_index (cx , expr , & mut map) ; check_assert (cx , expr , & mut map) ; ControlFlow :: < ! , () > :: Continue (()) }) ; report_indexes (cx , & map) ; } }
};
}
