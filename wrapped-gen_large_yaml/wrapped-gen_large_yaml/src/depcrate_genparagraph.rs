// Generated macro for paragraph (function)
macro_rules! Depcrate_genparagraph {
() => {
// Module: crate::gen
// Provides: {"paragraph"}
// Dependencies: {}
# [doc = " Generate a lipsum paragraph."] pub fn paragraph (rng : & mut SmallRng , lines_lo : usize , lines_hi : usize , wps_lo : usize , wps_hi : usize , line_maxcol : usize ,) -> Vec < String > { let mut ret = Vec :: new () ; let nlines = rng . gen_range (lines_lo .. lines_hi) ; while ret . len () < nlines { let words_in_sentence = rng . gen_range (wps_lo .. wps_hi) ; let mut sentence = lipsum :: lipsum_words_with_rng (rng . clone () , words_in_sentence) ; if let Some (last_line) = ret . pop () { sentence = format ! ("{last_line} {sentence}") ; } while sentence . len () > line_maxcol { let last_space_idx = line_maxcol - sentence [0 .. line_maxcol] . chars () . rev () . position (char :: is_whitespace) . unwrap () ; ret . push (sentence [0 .. last_space_idx] . to_string ()) ; sentence = sentence [last_space_idx + 1 ..] . to_string () ; } if ! sentence . is_empty () { ret . push (sentence) ; } } ret }
};
}
