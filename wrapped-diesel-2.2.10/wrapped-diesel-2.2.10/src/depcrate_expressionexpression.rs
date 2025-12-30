// Generated macro for Expression (trait)
macro_rules! Depcrate_expressionExpression {
() => {
// Module: crate::expression
// Provides: {"Expression"}
// Dependencies: {}
# [doc = " Represents a typed fragment of SQL."] # [doc = ""] # [doc = " Apps should not need to implement this type directly, but it may be common"] # [doc = " to use this in where clauses. Libraries should consider using"] # [doc = " [`infix_operator!`](crate::infix_operator!) or"] # [doc = " [`postfix_operator!`](crate::postfix_operator!) instead of"] # [doc = " implementing this directly."] pub trait Expression { # [doc = " The type that this expression represents in SQL"] type SqlType : TypedExpressionType ; }
};
}
