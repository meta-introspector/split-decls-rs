// Generated macro for write_identical_level (function)
macro_rules! Depcrate_comparisonwrite_identical_level {
() => {
// Module: crate::comparison
// Provides: {"write_identical_level"}
// Dependencies: {}
fn write_identical_level < I , S > (iter : I , sink : & mut S , state : & mut S :: State) -> Result < () , S :: Error > where I : Iterator < Item = char > , S : CollationKeySink + ? Sized , { let mut prev = 0i32 ; for c in iter { if ! (0x4e00 ..= 0xa000) . contains (& prev) { prev = (prev & ! 0x7f) - SLOPE_REACH_NEG_1 ; } else { prev = 0x9fff - SLOPE_REACH_POS_2 ; } if c == MERGE_SEPARATOR { sink . write_byte (state , MERGE_SEPARATOR_BYTE) ? ; prev = 0 ; } else { let c = c as i32 ; write_diff (c - prev , sink , state) ? ; prev = c ; } } Ok (()) }
};
}
