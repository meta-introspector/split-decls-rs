// Generated macro for new_random_cnonce (function)
macro_rules! Depcrate_digestnew_random_cnonce {
() => {
// Module: crate::digest
// Provides: {"new_random_cnonce"}
// Dependencies: {}
fn new_random_cnonce () -> String { let raw : [u8 ; 16] = rand :: random () ; hex :: encode (& raw [..]) }
};
}
