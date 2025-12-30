// Generated macro for checked_mul (function)
macro_rules! Depcratechecked_mul {
() => {
// Module: crate
// Provides: {"checked_mul"}
// Dependencies: {}
# [inline (always)] fn checked_mul (num : usize , opt : Option < usize >) -> Option < usize > { if let Some (n) = opt { n . checked_mul (num) } else { None } }
};
}
