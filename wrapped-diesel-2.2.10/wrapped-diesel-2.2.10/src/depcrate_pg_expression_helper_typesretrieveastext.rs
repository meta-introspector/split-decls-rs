// Generated macro for RetrieveAsText (type)
macro_rules! Depcrate_pg_expression_helper_typesRetrieveAsText {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"RetrieveAsText"}
// Dependencies: {}
# [doc (hidden)] pub type RetrieveAsText < Lhs , Rhs > = RetrieveAsTextJson < Lhs , < Rhs as JsonIndex > :: Expression , < < Rhs as JsonIndex > :: Expression as Expression > :: SqlType , > ;
};
}
