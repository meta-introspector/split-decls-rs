// Generated macro for checked_div (function)
macro_rules! Depcratechecked_div {
() => {
// Module: crate
// Provides: {"checked_div"}
// Dependencies: {}
# [inline (always)] fn checked_div (opt : Option < usize > , num : usize) -> Option < usize > { if let Some (n) = opt { n . checked_div (num) } else { None } }
};
}
