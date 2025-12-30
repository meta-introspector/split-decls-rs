// Generated macro for wants_emoji (function)
macro_rules! Depcrate_windows_termwants_emoji {
() => {
// Module: crate::windows_term
// Provides: {"wants_emoji"}
// Dependencies: {}
pub (crate) fn wants_emoji () -> bool { env :: var ("WT_SESSION") . is_ok () }
};
}
