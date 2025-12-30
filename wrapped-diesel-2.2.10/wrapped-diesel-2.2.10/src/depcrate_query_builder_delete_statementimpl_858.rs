// Generated macro for impl_858 (impl)
macro_rules! Depcrate_query_builder_delete_statementimpl_858 {
() => {
// Module: crate::query_builder::delete_statement
// Provides: {"impl_858"}
// Dependencies: {}
impl < T , U , Ret > std :: fmt :: Debug for DeleteStatement < T , U , Ret > where T : QuerySource , FromClause < T > : std :: fmt :: Debug , U : std :: fmt :: Debug , Ret : std :: fmt :: Debug , { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("DeleteStatement") . field ("from_clause" , & self . from_clause) . field ("where_clause" , & self . where_clause) . field ("returning" , & self . returning) . finish () } }
};
}
