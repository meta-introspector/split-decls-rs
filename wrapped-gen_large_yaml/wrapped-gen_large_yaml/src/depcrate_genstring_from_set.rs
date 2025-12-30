// Generated macro for string_from_set (function)
macro_rules! Depcrate_genstring_from_set {
() => {
// Module: crate::gen
// Provides: {"string_from_set"}
// Dependencies: {}
# [doc = " Generate a string with hexadecimal digits of the specified length."] pub fn string_from_set (rng : & mut SmallRng , len_lo : usize , len_hi : usize , set : & [u8]) -> String { (0 .. rng . gen_range (len_lo .. len_hi)) . map (| _ | set [rng . gen_range (0 .. set . len ())] as char) . collect () }
};
}
