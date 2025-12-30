// Generated macro for checked_add_opt (function)
macro_rules! Depcratechecked_add_opt {
() => {
// Module: crate
// Provides: {"checked_add_opt"}
// Dependencies: {}
# [inline (always)] fn checked_add_opt (one : Option < usize > , other : Option < usize >) -> Option < usize > { if let Some (n) = one { checked_add (n , other) } else { None } }
};
}
