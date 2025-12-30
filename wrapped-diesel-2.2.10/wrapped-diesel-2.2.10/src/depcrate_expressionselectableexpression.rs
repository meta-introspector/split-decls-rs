// Generated macro for SelectableExpression (trait)
macro_rules! Depcrate_expressionSelectableExpression {
() => {
// Module: crate::expression
// Provides: {"SelectableExpression"}
// Dependencies: {}
# [doc = " Indicates that an expression can be selected from a source."] # [doc = ""] # [doc = " Columns will implement this for their table. Certain special types, like"] # [doc = " `CountStar` and `Bound` will implement this for all sources. Most compound"] # [doc = " expressions will implement this if each of their parts implement it."] # [doc = ""] # [doc = " Notably, columns will not implement this trait for the right side of a left"] # [doc = " join. To select a column or expression using a column from the right side of"] # [doc = " a left join, you must call `.nullable()` on it."] # [diagnostic :: on_unimplemented (message = "Cannot select `{Self}` from `{QS}`" , note = "`{Self}` is no valid selection for `{QS}`")] pub trait SelectableExpression < QS : ? Sized > : AppearsOnTable < QS > { }
};
}
