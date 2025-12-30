// Generated macro for get_iter_index (function)
macro_rules! Depcrateget_iter_index {
() => {
// Module: crate
// Provides: {"get_iter_index"}
// Dependencies: {}
fn get_iter_index (z : Complex , c : Complex) -> u32 { let mut iter_index : u32 = 0 ; let mut z = z ; while iter_index < 900 { if z . norm () > 2.0 { break ; } z = z . square () + c ; iter_index += 1 ; } iter_index }
};
}
