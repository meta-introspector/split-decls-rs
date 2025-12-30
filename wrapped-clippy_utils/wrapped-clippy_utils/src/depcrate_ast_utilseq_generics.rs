// Generated macro for eq_generics (function)
macro_rules! Depcrate_ast_utilseq_generics {
() => {
// Module: crate::ast_utils
// Provides: {"eq_generics"}
// Dependencies: {}
pub fn eq_generics (l : & Generics , r : & Generics) -> bool { over (& l . params , & r . params , eq_generic_param) && over (& l . where_clause . predicates , & r . where_clause . predicates , | l , r | { eq_where_predicate (l , r) }) }
};
}
