// Generated macro for AppendSelection (trait)
macro_rules! Depcrate_query_source_joinsAppendSelection {
() => {
// Module: crate::query_source::joins
// Provides: {"AppendSelection"}
// Dependencies: {}
# [doc (hidden)] # [doc = " Used to ensure the sql type of `left.join(mid).join(right)` is"] # [doc = " `(Left, Mid, Right)` and not `((Left, Mid), Right)`. This needs"] # [doc = " to be separate from `TupleAppend` because we still want to keep"] # [doc = " the column lists (which are tuples) separate."] pub trait AppendSelection < Selection > { type Output ; fn append_selection (& self , selection : Selection) -> Self :: Output ; }
};
}
