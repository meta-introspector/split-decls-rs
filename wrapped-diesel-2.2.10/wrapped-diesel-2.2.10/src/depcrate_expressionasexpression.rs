// Generated macro for AsExpression (trait)
macro_rules! Depcrate_expressionAsExpression {
() => {
// Module: crate::expression
// Provides: {"AsExpression"}
// Dependencies: {}
# [doc = " Converts a type to its representation for use in Diesel's query builder."] # [doc = ""] # [doc = " This trait is used directly. Apps should typically use [`IntoSql`] instead."] # [doc = ""] # [doc = " Implementations of this trait will generally do one of 3 things:"] # [doc = ""] # [doc = " - Return `self` for types which are already parts of Diesel's query builder"] # [doc = " - Perform some implicit coercion (for example, allowing [`now`] to be used as"] # [doc = "   both [`Timestamp`] and [`Timestamptz`]."] # [doc = " - Indicate that the type has data which will be sent separately from the"] # [doc = "   query. This is generally referred as a \"bind parameter\". Types which"] # [doc = "   implement [`ToSql`] will generally implement `AsExpression` this way."] # [doc = ""] # [doc = "   [`IntoSql`]: crate::IntoSql"] # [doc = "   [`now`]: crate::dsl::now"] # [doc = "   [`Timestamp`]: crate::sql_types::Timestamp"] # [doc = "   [`Timestamptz`]: ../pg/types/sql_types/struct.Timestamptz.html"] # [doc = "   [`ToSql`]: crate::serialize::ToSql"] # [doc = ""] # [doc = "  This trait could be [derived](derive@AsExpression)"] pub trait AsExpression < T > where T : SqlType + TypedExpressionType , { # [doc = " The expression being returned"] type Expression : Expression < SqlType = T > ; # [doc = " Perform the conversion"] # [allow (clippy :: wrong_self_convention)] fn as_expression (self) -> Self :: Expression ; }
};
}
