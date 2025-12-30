// Generated macro for impl_896 (impl)
macro_rules! Depcrate_query_builder_from_clauseimpl_896 {
() => {
// Module: crate::query_builder::from_clause
// Provides: {"impl_896"}
// Dependencies: {}
impl < F > std :: fmt :: Debug for FromClause < F > where F : QuerySource , F :: FromClause : std :: fmt :: Debug , { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("FromClause") . field ("from_clause" , & self . from_clause) . finish () } }
};
}
