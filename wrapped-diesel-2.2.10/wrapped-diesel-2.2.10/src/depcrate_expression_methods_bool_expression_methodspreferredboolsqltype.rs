// Generated macro for PreferredBoolSqlType (trait)
macro_rules! Depcrate_expression_methods_bool_expression_methodsPreferredBoolSqlType {
() => {
// Module: crate::expression_methods::bool_expression_methods
// Provides: {"PreferredBoolSqlType"}
// Dependencies: {}
# [doc = " Allow ~type inference on [And](crate::helper_types::And) and [Or](crate::helper_types::Or)"] # [doc = " helper types"] # [doc = ""] # [doc = " This is used to be statistically correct as last generic parameter of `dsl::And` and `dsl::Or`"] # [doc = " without having to specify an additional type parameter."] # [doc = ""] # [doc = " It works with types that are [Expression]s and have a [`SqlType`](Expression::SqlType) that is"] # [doc = " either [`Bool`](sql_types::Bool) or [`Nullable<Bool>`](sql_types::Nullable), and with [`bool`]"] # [doc = " (and `Option<bool>` and references to those)."] # [doc = ""] # [doc = " Cases where an additional type parameter would still have to be specified in the helper type"] # [doc = " generic parameters are:"] # [doc = " - If this trait isn't implemented for the `other` parameter of the expression"] # [doc = "   (in that case the user (you?) probably wants to implement it)"] # [doc = " - If the user actually was using the not-preferred implementation of `AsExpression`"] # [doc = "   (e.g. towards `Nullable<Bool>` instead of `Bool`)"] pub trait PreferredBoolSqlType { # [doc = " The preferred `Bool` SQL type for this AsExpression implementation."] # [doc = ""] # [doc = " That should be either `Bool` or `Nullable<Bool>`."] type PreferredSqlType ; }
};
}
