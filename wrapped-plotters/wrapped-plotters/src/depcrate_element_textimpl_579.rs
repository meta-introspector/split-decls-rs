// Generated macro for impl_579 (impl)
macro_rules! Depcrate_element_textimpl_579 {
() => {
// Module: crate::element::text
// Provides: {"impl_579"}
// Dependencies: {}
impl < 'a , Coord > MultiLineText < 'a , Coord , & 'a str > { # [doc = " Parse a multi-line text into an multi-line element."] # [doc = ""] # [doc = " `text`: The text that is parsed"] # [doc = " `pos`: The position of the text"] # [doc = " `style`: The style for this text"] # [doc = " `max_width`: The width of the multi-line text element, the line will break"] # [doc = " into two lines if the line is wider than the max_width. If 0 is given, do not"] # [doc = " do any line wrapping"] pub fn from_str < ST : Into < & 'a str > , S : Into < TextStyle < 'a > > > (text : ST , pos : Coord , style : S , max_width : u32 ,) -> Self { let text = text . into () ; let mut ret = MultiLineText :: new (pos , style) ; layout_multiline_text (text , max_width , ret . style . font . clone () , | l | { ret . push_line (l) }) ; ret } }
};
}
