// Generated macro for UnsafeCell (struct)
macro_rules! Depcrate_cell_unsafe_cellUnsafeCell {
() => {
// Module: crate::cell::unsafe_cell
// Provides: {"UnsafeCell"}
// Dependencies: {}
# [doc = " A checked version of `std::cell::UnsafeCell`."] # [doc = ""] # [doc = " Instead of providing a `get()` API, this version of `UnsafeCell` provides"] # [doc = " `with` and `with_mut`. Both functions take a closure in order to track the"] # [doc = " start and end of the access to the underlying cell."] # [derive (Debug)] pub struct UnsafeCell < T : ? Sized > { # [doc = " Causality associated with the cell"] state : rt :: Cell , data : std :: cell :: UnsafeCell < T > , }
};
}
