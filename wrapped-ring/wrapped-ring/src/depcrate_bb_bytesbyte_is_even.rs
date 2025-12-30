// Generated macro for byte_is_even (function)
macro_rules! Depcrate_bb_bytesbyte_is_even {
() => {
// Module: crate::bb::bytes
// Provides: {"byte_is_even"}
// Dependencies: {}
pub fn byte_is_even (a : & u8) -> BoolMask { const _1 : LeakyWord = 1 ; let a = Word :: from (* a) ; (a & Word :: from (_1)) . is_zero () }
};
}
