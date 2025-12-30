// Generated macro for SelectAll (struct)
macro_rules! Depcrate_select_allSelectAll {
() => {
// Module: crate::select_all
// Provides: {"SelectAll"}
// Dependencies: {}
# [doc = " Future for the `select_all` combinator, waiting for one of any of a list of"] # [doc = " futures to complete."] # [doc = ""] # [doc = " This is created by this `select_all` function."] pub struct SelectAll < A > where A : Future { inner : Vec < SelectAllNext < A > > , }
};
}
