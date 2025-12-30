// Generated macro for confirm (function)
macro_rules! Depcrateconfirm {
() => {
// Module: crate
// Provides: {"confirm"}
// Dependencies: {}
# [doc = " Calls the confirm function."] # [doc = ""] # [doc = " [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)"] pub fn confirm (message : & str) -> bool { window () . confirm_with_message (message) . unwrap_throw () }
};
}
