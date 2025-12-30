// Generated macro for trim_offset (function)
macro_rules! Depcrate_reflowtrim_offset {
() => {
// Module: crate::reflow
// Provides: {"trim_offset"}
// Dependencies: {}
# [doc = " This function will return a str slice which start at specified offset."] # [doc = " As src is a unicode str, start offset has to be calculated with each character."] fn trim_offset (src : & str , mut offset : usize) -> & str { let mut start = 0 ; for c in UnicodeSegmentation :: graphemes (src , true) { let w = c . width () ; if w <= offset { offset -= w ; start += c . len () ; } else { break ; } } # [expect (clippy :: string_slice)] & src [start ..] }
};
}
