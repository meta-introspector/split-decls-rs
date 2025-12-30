// Generated macro for MultiLineText (struct)
macro_rules! Depcrate_element_textMultiLineText {
() => {
// Module: crate::element::text
// Provides: {"MultiLineText"}
// Dependencies: {}
# [doc = " An multi-line text element. The `Text` element allows only single line text"] # [doc = " and the `MultiLineText` supports drawing multiple lines"] pub struct MultiLineText < 'a , Coord , T : Borrow < str > > { lines : Vec < T > , coord : Coord , style : TextStyle < 'a > , line_height : f64 , }
};
}
