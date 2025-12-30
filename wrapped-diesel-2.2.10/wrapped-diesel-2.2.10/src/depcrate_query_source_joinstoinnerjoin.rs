// Generated macro for ToInnerJoin (trait)
macro_rules! Depcrate_query_source_joinsToInnerJoin {
() => {
// Module: crate::query_source::joins
// Provides: {"ToInnerJoin"}
// Dependencies: {}
# [doc (hidden)] # [doc = " Convert any joins in a `FROM` clause into an inner join."] # [doc = ""] # [doc = " This trait is used to determine whether"] # [doc = " `Nullable<T>: SelectableExpression<SomeJoin>`. We consider it to be"] # [doc = " selectable if `T: SelectableExpression<InnerJoin>`. Since `SomeJoin`"] # [doc = " may be deeply nested, we need to recursively change any appearances of"] # [doc = " `LeftOuter` to `Inner` in order to perform this check."] pub trait ToInnerJoin { type InnerJoin ; }
};
}
