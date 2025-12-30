// Generated macro for impl_17 (impl)
macro_rules! Depcrate_decoderimpl_17 {
() => {
// Module: crate::decoder
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a > Encapsulation < 'a > { # [doc = " Parse the type label and encapsulated text from between the"] # [doc = " pre/post-encapsulation boundaries."] pub fn parse (data : & 'a [u8]) -> Result < Self > { let data = grammar :: strip_preamble (data) ? ; let data = data . strip_prefix (PRE_ENCAPSULATION_BOUNDARY) . ok_or (Error :: PreEncapsulationBoundary) ? ; let (label , body) = grammar :: split_label (data) . ok_or (Error :: Label) ? ; let mut body = match grammar :: strip_trailing_eol (body) . unwrap_or (body) { [head @ .. , b'-' , b'-' , b'-' , b'-' , b'-'] => head , _ => return Err (Error :: PreEncapsulationBoundary) , } ; for & slice in [POST_ENCAPSULATION_BOUNDARY , label . as_bytes ()] . iter () . rev () { if ! body . ends_with (slice) { return Err (Error :: PostEncapsulationBoundary) ; } let len = body . len () . checked_sub (slice . len ()) . ok_or (Error :: Length) ? ; body = body . get (.. len) . ok_or (Error :: PostEncapsulationBoundary) ? ; } let encapsulated_text = grammar :: strip_trailing_eol (body) . ok_or (Error :: PostEncapsulationBoundary) ? ; Ok (Self { label , encapsulated_text , }) } # [doc = " Get the label parsed from the encapsulation boundaries."] pub fn label (self) -> & 'a str { self . label } # [doc = " Detect the line width of the encapsulated text by looking for the position of the first EOL."] pub fn encapsulated_text_line_width (self) -> usize { self . encapsulated_text . iter () . copied () . position (| c | matches ! (c , grammar :: CHAR_CR | grammar :: CHAR_LF)) . unwrap_or (self . encapsulated_text . len ()) } }
};
}
