// Generated macro for AsyncTimer (trait)
macro_rules! Depcrate_runtimeAsyncTimer {
() => {
// Module: crate::runtime
// Provides: {"AsyncTimer"}
// Dependencies: {}
# [doc = " Abstract implementation of an async timer for runtime independence"] pub trait AsyncTimer : Send + Debug + 'static { # [doc = " Update the timer to expire at `i`"] fn reset (self : Pin < & mut Self > , i : Instant) ; # [doc = " Check whether the timer has expired, and register to be woken if not"] fn poll (self : Pin < & mut Self > , cx : & mut Context) -> Poll < () > ; }
};
}
