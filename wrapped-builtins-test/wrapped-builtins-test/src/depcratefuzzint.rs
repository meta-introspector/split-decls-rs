// Generated macro for FuzzInt (trait)
macro_rules! DepcrateFuzzInt {
() => {
// Module: crate
// Provides: {"FuzzInt"}
// Dependencies: {}
# [doc = " Additional constants that determine how the integer gets fuzzed."] trait FuzzInt : MinInt { # [doc = " LUT used for maximizing the space covered and minimizing the computational cost of fuzzing"] # [doc = " in `builtins-test`. For example, Self = u128 produces [0,1,2,7,8,15,16,31,32,63,64,95,96,"] # [doc = " 111,112,119,120,125,126,127]."] const FUZZ_LENGTHS : [u8 ; 20] = make_fuzz_lengths (Self :: BITS) ; # [doc = " The number of entries of `FUZZ_LENGTHS` actually used. The maximum is 20 for u128."] const FUZZ_NUM : usize = { let log2 = Self :: BITS . ilog2 () as usize ; if log2 == 3 { 6 } else { 8 + (4 * (log2 - 4)) } } ; }
};
}
