// Generated macro for IsNot (type)
macro_rules! Depcrate_sqlite_expression_helper_typesIsNot {
() => {
// Module: crate::sqlite::expression::helper_types
// Provides: {"IsNot"}
// Dependencies: {}
# [doc = " The return type of `lhs.is_not(rhs)`."] pub type IsNot < Lhs , Rhs > = Grouped < super :: operators :: IsNot < Lhs , AsExpr < Rhs , Lhs > > > ;
};
}
