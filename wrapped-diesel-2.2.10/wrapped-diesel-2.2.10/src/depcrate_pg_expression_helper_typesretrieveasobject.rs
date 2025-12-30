// Generated macro for RetrieveAsObject (type)
macro_rules! Depcrate_pg_expression_helper_typesRetrieveAsObject {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"RetrieveAsObject"}
// Dependencies: {}
# [doc (hidden)] pub type RetrieveAsObject < Lhs , Rhs > = RetrieveAsObjectJson < Lhs , < Rhs as JsonIndex > :: Expression , < < Rhs as JsonIndex > :: Expression as Expression > :: SqlType , > ;
};
}
