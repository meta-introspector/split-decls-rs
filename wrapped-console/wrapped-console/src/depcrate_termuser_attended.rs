// Generated macro for user_attended (function)
macro_rules! Depcrate_termuser_attended {
() => {
// Module: crate::term
// Provides: {"user_attended"}
// Dependencies: {}
# [doc = " A fast way to check if the application has a user attended for stdout."] # [doc = ""] # [doc = " This means that stdout is connected to a terminal instead of a"] # [doc = " file or redirected by other means. This is a shortcut for"] # [doc = " checking the `is_attended` feature on the stdout terminal."] # [inline] pub fn user_attended () -> bool { Term :: stdout () . features () . is_attended () }
};
}
