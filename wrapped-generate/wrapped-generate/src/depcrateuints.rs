// Generated macro for uints (function)
macro_rules! Depcrateuints {
() => {
// Module: crate
// Provides: {"uints"}
// Dependencies: {}
fn uints () -> impl Iterator < Item = u64 > { let first2 : u32 = (HIGHEST as f64) . log (2.0) . round () as u32 + 1 ; let first10 : u32 = (HIGHEST as f64) . log (10.0) as u32 + 1 ; let other_constants = [3600] ; (0 .. (HIGHEST + 1)) . chain (other_constants) . chain ((first2 .. 64) . flat_map (| i | [2u64 . pow (i) - 1 , 2u64 . pow (i)])) . chain ((first10 .. 20) . map (| i | 10u64 . pow (i))) }
};
}
