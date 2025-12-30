// Generated macro for SelectNext (struct)
macro_rules! Depcrate_selectSelectNext {
() => {
// Module: crate::select
// Provides: {"SelectNext"}
// Dependencies: {}
# [doc = " Future yielded as the second result in a `Select` future."] # [doc = ""] # [doc = " This sentinel future represents the completion of the second future to a"] # [doc = " `select` which finished second."] pub struct SelectNext < A , B > where A : Future , B : Future < Item = A :: Item , Error = A :: Error > { inner : OneOf < A , B > , }
};
}
