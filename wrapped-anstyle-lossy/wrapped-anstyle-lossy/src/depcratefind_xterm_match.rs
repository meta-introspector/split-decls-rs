// Generated macro for find_xterm_match (function)
macro_rules! Depcratefind_xterm_match {
() => {
// Module: crate
// Provides: {"find_xterm_match"}
// Dependencies: {}
const fn find_xterm_match (color : anstyle :: RgbColor) -> usize { let mut best_index = 16 ; let mut best_distance = distance (color , XTERM_COLORS [best_index]) ; let mut index = best_index + 1 ; while index < XTERM_COLORS . len () { let distance = distance (color , XTERM_COLORS [index]) ; if distance < best_distance { best_index = index ; best_distance = distance ; } index += 1 ; } best_index }
};
}
