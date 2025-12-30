// Generated macro for DisplayWidth (struct)
macro_rules! Depcrate_output_cellDisplayWidth {
() => {
// Module: crate::output::cell
// Provides: {"DisplayWidth"}
// Dependencies: {}
# [doc = " The Unicode “display width” of a string."] # [doc = ""] # [doc = " This is related to the number of *graphemes* of a string, rather than the"] # [doc = " number of *characters*, or *bytes*: although most characters are one"] # [doc = " column wide, a few can be two columns wide, and this is important to note"] # [doc = " when calculating widths for displaying tables in a terminal."] # [doc = ""] # [doc = " This type is used to ensure that the width, rather than the length, is"] # [doc = " used when constructing a `TextCell` — it’s too easy to write something"] # [doc = " like `file_name.len()` and assume it will work!"] # [doc = ""] # [doc = " It has `From` impls that convert an input string or fixed with to values"] # [doc = " of this type, and will `Deref` to the contained `usize` value."] # [derive (PartialEq , Eq , Debug , Clone , Copy , Default)] pub struct DisplayWidth (usize) ;
};
}
