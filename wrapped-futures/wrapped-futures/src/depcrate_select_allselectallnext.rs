// Generated macro for SelectAllNext (struct)
macro_rules! Depcrate_select_allSelectAllNext {
() => {
// Module: crate::select_all
// Provides: {"SelectAllNext"}
// Dependencies: {}
# [doc = " Future yielded as the result in a `SelectAll` future."] # [doc = ""] # [doc = " This sentinel future represents the completion of the remaining futures in a"] # [doc = " list of futures."] pub struct SelectAllNext < A > where A : Future { inner : Collapsed < A > , }
};
}
