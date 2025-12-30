// Generated macro for impl_1121 (impl)
macro_rules! Depcrate_query_builder_select_clauseimpl_1121 {
() => {
// Module: crate::query_builder::select_clause
// Provides: {"impl_1121"}
// Dependencies: {}
impl < QS > std :: fmt :: Debug for DefaultSelectClause < QS > where QS : AsQuerySource , < QS :: QuerySource as QuerySource > :: DefaultSelection : std :: fmt :: Debug , { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("DefaultSelectClause") . field ("default_selection" , & self . default_selection) . finish () } }
};
}
