// Generated macro for name (function)
macro_rules! Depcrate_genname {
() => {
// Module: crate::gen
// Provides: {"name"}
// Dependencies: {}
# [doc = " Generate a name."] pub fn name (rng : & mut SmallRng , len_lo : usize , len_hi : usize) -> String { const UPPER : & [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ" ; const LOWER : & [u8] = b"abcdefghijklmnopqrstuvwxyz" ; let len = rng . gen_range (len_lo .. len_hi) ; let mut ret = String :: new () ; ret . push (UPPER [rng . gen_range (0 .. UPPER . len ())] as char) ; ret . push_str (string_from_set (rng , len , len + 1 , LOWER) . as_str ()) ; ret }
};
}
