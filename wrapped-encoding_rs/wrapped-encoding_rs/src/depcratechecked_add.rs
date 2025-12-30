// Generated macro for checked_add (function)
macro_rules! Depcratechecked_add {
() => {
// Module: crate
// Provides: {"checked_add"}
// Dependencies: {}
# [inline (always)] fn checked_add (num : usize , opt : Option < usize >) -> Option < usize > { if let Some (n) = opt { n . checked_add (num) } else { None } }
};
}
