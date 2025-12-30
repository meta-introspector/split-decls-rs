// Generated macro for Cell (struct)
macro_rules! Depcrate_cell_cellCell {
() => {
// Module: crate::cell::cell
// Provides: {"Cell"}
// Dependencies: {}
# [doc = " A checked version of [`std::cell::Cell`], implemented on top of"] # [doc = " [`loom::cell::UnsafeCell`][unsafecell]."] # [doc = ""] # [doc = " Unlike [`loom::cell::UnsafeCell`][unsafecell], this provides an API that's"] # [doc = " largely compatible with the standard counterpart."] # [doc = ""] # [doc = " [unsafecell]: crate::cell::UnsafeCell"] # [derive (Debug)] pub struct Cell < T > { cell : UnsafeCell < T > , }
};
}
