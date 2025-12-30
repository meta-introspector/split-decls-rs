// Generated macro for private (module)
macro_rules! Depcrate_query_sourceprivate {
() => {
// Module: crate::query_source
// Provides: {"private"}
// Dependencies: {}
pub (crate) mod private { use super :: { Never , Once } ; # [doc = " Used to determine which of two from clauses contains a given table."] # [doc = ""] # [doc = " This trait can be used to emulate \"or\" conditions in where clauses when"] # [doc = " we want a trait to be implemented with one of two type parameters."] # [doc = ""] # [doc = " For example, if we wanted to write:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " where"] # [doc = "     T: SelectableExpression<Left> | SelectableExpression<Right>,"] # [doc = " ```"] # [doc = ""] # [doc = " we can emulate this by writing:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " where"] # [doc = "     Left: AppearsInFromClause<T::Table>,"] # [doc = "     Right: AppearsInFromClause<T::Table>,"] # [doc = "     (Left::Count, Right::Count): Pick<Left, Right>,"] # [doc = "     T: SelectableExpression<"] # [doc = "         <(Left::Count, Right::Count) as Pick<Left, Right>>::Selection,"] # [doc = "     >,"] # [doc = " ```"] # [doc = ""] # [doc = " In order to acquire the counts in the first place, we must already know"] # [doc = " the table we're searching for."] # [doc (hidden)] pub trait Pick < Left , Right > { # [doc = " The selected type."] # [doc = ""] # [doc = " For `(Once, Never)` this type will be `Left`. For `(Never, Once)`, this type will be"] # [doc = " `Right`"] type Selection ; } impl < Left , Right > Pick < Left , Right > for (Once , Never) { type Selection = Left ; } impl < Left , Right > Pick < Left , Right > for (Never , Once) { type Selection = Right ; } }
};
}
