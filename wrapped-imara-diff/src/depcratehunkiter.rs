// Generated macro for HunkIter (struct)
macro_rules! DepcrateHunkIter {
() => {
// Module: crate
// Provides: {"HunkIter"}
// Dependencies: {}
# [doc = " Yields all [`Hunk`]s in a file in monotonically increasing order."] # [doc = " Monotonically increasing means here that the following holds for any two"] # [doc = " consecutive [`Hunk`]s `x` and `y`:"] # [doc = ""] # [doc = " ``` no_compile"] # [doc = " assert!(x.before.end < y.before.start);"] # [doc = " assert!(x.after.end < y.after.start);"] # [doc = " ```"] # [doc = ""] pub struct HunkIter < 'diff > { removed : slice :: Iter < 'diff , bool > , added : slice :: Iter < 'diff , bool > , pos_before : u32 , pos_after : u32 , }
};
}
