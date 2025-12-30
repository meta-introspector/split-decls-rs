// Generated macro for ErrnoSentinel (trait)
macro_rules! Depcrate_errnoErrnoSentinel {
() => {
// Module: crate::errno
// Provides: {"ErrnoSentinel"}
// Dependencies: {}
# [doc = " The sentinel value indicates that a function failed and more detailed"] # [doc = " information about the error can be found in `errno`"] pub trait ErrnoSentinel : Sized { fn sentinel () -> Self ; }
};
}
