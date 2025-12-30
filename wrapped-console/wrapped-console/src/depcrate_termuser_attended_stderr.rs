// Generated macro for user_attended_stderr (function)
macro_rules! Depcrate_termuser_attended_stderr {
() => {
// Module: crate::term
// Provides: {"user_attended_stderr"}
// Dependencies: {}
# [doc = " A fast way to check if the application has a user attended for stderr."] # [doc = ""] # [doc = " This means that stderr is connected to a terminal instead of a"] # [doc = " file or redirected by other means. This is a shortcut for"] # [doc = " checking the `is_attended` feature on the stderr terminal."] # [inline] pub fn user_attended_stderr () -> bool { Term :: stderr () . features () . is_attended () }
};
}
