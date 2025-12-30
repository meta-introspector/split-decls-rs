// Generated macro for convert_char (macro)
macro_rules! Depcrate_rangeconvert_char {
() => {
// Module: crate::range
// Provides: {"convert_char"}
// Dependencies: {}
macro_rules ! convert_char { ($ self : ident . $ method : ident ($ ($ arg : expr) ,*)) => { { let start = $ self . range . start as u32 ; let end = $ self . range . end as u32 ; if start < 0xD800 && 0xE000 < end { (start .. 0xD800) . into_par_iter () . chain (0xE000 .. end) . map (| codepoint | unsafe { char :: from_u32_unchecked (codepoint) }) .$ method ($ ($ arg) ,*) } else { (start .. end) . into_par_iter () . map (| codepoint | unsafe { char :: from_u32_unchecked (codepoint) }) .$ method ($ ($ arg) ,*) } } } ; }
};
}
