// Generated macro for after_percent_sign (function)
macro_rules! Depcrateafter_percent_sign {
() => {
// Module: crate
// Provides: {"after_percent_sign"}
// Dependencies: {}
fn after_percent_sign (iter : & mut slice :: Iter < '_ , u8 >) -> Option < u8 > { let mut cloned_iter = iter . clone () ; let h = char :: from (* cloned_iter . next () ?) . to_digit (16) ? ; let l = char :: from (* cloned_iter . next () ?) . to_digit (16) ? ; * iter = cloned_iter ; Some (h as u8 * 0x10 + l as u8) }
};
}
