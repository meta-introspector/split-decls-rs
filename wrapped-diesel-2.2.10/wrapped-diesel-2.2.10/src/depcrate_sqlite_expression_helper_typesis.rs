// Generated macro for Is (type)
macro_rules! Depcrate_sqlite_expression_helper_typesIs {
() => {
// Module: crate::sqlite::expression::helper_types
// Provides: {"Is"}
// Dependencies: {}
# [doc = " The return type of `lhs.is(rhs)`."] pub type Is < Lhs , Rhs > = Grouped < super :: operators :: Is < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
