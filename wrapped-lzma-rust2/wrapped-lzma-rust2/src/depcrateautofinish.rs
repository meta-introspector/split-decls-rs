// Generated macro for AutoFinish (trait)
macro_rules! DepcrateAutoFinish {
() => {
// Module: crate
// Provides: {"AutoFinish"}
// Dependencies: {}
# [doc = " A trait for writers that finishes the stream on drop."] trait AutoFinish { # [doc = " Finish writing the stream without error handling."] fn finish_ignore_error (self) ; }
};
}
