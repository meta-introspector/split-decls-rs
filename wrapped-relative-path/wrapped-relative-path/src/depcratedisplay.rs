// Generated macro for Display (struct)
macro_rules! DepcrateDisplay {
() => {
// Module: crate
// Provides: {"Display"}
// Dependencies: {}
# [doc = " Helper struct for printing relative paths."] # [doc = ""] # [doc = " This is not strictly necessary in the same sense as it is for [`Display`],"] # [doc = " because relative paths are guaranteed to be valid UTF-8. But the behavior is"] # [doc = " preserved to simplify the transition between [`Path`] and [`RelativePath`]."] # [doc = ""] # [doc = " [`Path`]: std::path::Path"] # [doc = " [`Display`]: core::fmt::Display"] # [deprecated (note = "RelativePath implements std::fmt::Display directly")] pub struct Display < 'a > { path : & 'a RelativePath , }
};
}
