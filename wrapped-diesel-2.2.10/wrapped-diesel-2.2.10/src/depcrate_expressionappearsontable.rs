// Generated macro for AppearsOnTable (trait)
macro_rules! Depcrate_expressionAppearsOnTable {
() => {
// Module: crate::expression
// Provides: {"AppearsOnTable"}
// Dependencies: {}
# [doc = " Indicates that all elements of an expression are valid given a from clause."] # [doc = ""] # [doc = " This is used to ensure that `users.filter(posts::id.eq(1))` fails to"] # [doc = " compile. This constraint is only used in places where the nullability of a"] # [doc = " SQL type doesn't matter (everything except `select` and `returning`). For"] # [doc = " places where nullability is important, `SelectableExpression` is used"] # [doc = " instead."] pub trait AppearsOnTable < QS : ? Sized > : Expression { }
};
}
