// Generated macro for text (function)
macro_rules! Depcrate_gentext {
() => {
// Module: crate::gen
// Provides: {"text"}
// Dependencies: {}
# [doc = " Generate a lipsum text."] # [doc = ""] # [doc = " Texts are composed of some paragraphs and empty lines between them."] pub fn text (rng : & mut SmallRng , paragraphs_lo : usize , paragraphs_hi : usize , lines_lo : usize , lines_hi : usize , wps_lo : usize , wps_hi : usize , line_maxcol : usize ,) -> Vec < String > { let mut ret = Vec :: new () ; let mut first = true ; for _ in 0 .. rng . gen_range (paragraphs_lo .. paragraphs_hi) { if first { first = false ; } else { ret . push (String :: new ()) ; } ret . extend (paragraph (rng , lines_lo , lines_hi , wps_lo , wps_hi , line_maxcol) . into_iter ()) ; } ret }
};
}
