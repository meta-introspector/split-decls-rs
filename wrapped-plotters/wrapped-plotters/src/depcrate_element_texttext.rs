// Generated macro for Text (struct)
macro_rules! Depcrate_element_textText {
() => {
// Module: crate::element::text
// Provides: {"Text"}
// Dependencies: {}
# [doc = " A single line text element. This can be owned or borrowed string, dependents on"] # [doc = " `String` or `str` moved into."] pub struct Text < 'a , Coord , T : Borrow < str > > { text : T , coord : Coord , style : TextStyle < 'a > , }
};
}
