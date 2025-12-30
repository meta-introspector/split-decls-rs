// Generated macro for alert (function)
macro_rules! Depcratealert {
() => {
// Module: crate
// Provides: {"alert"}
// Dependencies: {}
# [doc = " Calls the alert function."] # [doc = ""] # [doc = " [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)"] pub fn alert (message : & str) { window () . alert_with_message (message) . unwrap_throw () }
};
}
