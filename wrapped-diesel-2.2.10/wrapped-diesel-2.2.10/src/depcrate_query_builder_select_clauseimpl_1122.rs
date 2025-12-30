// Generated macro for impl_1122 (impl)
macro_rules! Depcrate_query_builder_select_clauseimpl_1122 {
() => {
// Module: crate::query_builder::select_clause
// Provides: {"impl_1122"}
// Dependencies: {}
impl < QS > Clone for DefaultSelectClause < QS > where QS : AsQuerySource , < QS :: QuerySource as QuerySource > :: DefaultSelection : Clone , { fn clone (& self) -> Self { Self { default_selection : self . default_selection . clone () , } } }
};
}
