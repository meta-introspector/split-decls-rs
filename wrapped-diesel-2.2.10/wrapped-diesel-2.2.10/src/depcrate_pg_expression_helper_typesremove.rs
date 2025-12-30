// Generated macro for Remove (type)
macro_rules! Depcrate_pg_expression_helper_typesRemove {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"Remove"}
// Dependencies: {}
# [doc (hidden)] pub type Remove < Lhs , Rhs > = RemoveFromJsonb < Lhs , < Rhs as JsonRemoveIndex > :: Expression , < < Rhs as JsonRemoveIndex > :: Expression as Expression > :: SqlType , > ;
};
}
