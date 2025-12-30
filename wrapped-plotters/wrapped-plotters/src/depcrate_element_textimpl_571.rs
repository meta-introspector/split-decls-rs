// Generated macro for impl_571 (impl)
macro_rules! Depcrate_element_textimpl_571 {
() => {
// Module: crate::element::text
// Provides: {"impl_571"}
// Dependencies: {}
impl < 'a , Coord , T : Borrow < str > > Text < 'a , Coord , T > { # [doc = " Create a new text element"] # [doc = " - `text`: The text for the element"] # [doc = " - `points`: The upper left conner for the text element"] # [doc = " - `style`: The text style"] # [doc = " - Return the newly created text element"] pub fn new < S : Into < TextStyle < 'a > > > (text : T , points : Coord , style : S) -> Self { Self { text , coord : points , style : style . into () , } } }
};
}
