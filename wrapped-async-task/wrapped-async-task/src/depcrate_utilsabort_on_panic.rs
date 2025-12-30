// Generated macro for abort_on_panic (function)
macro_rules! Depcrate_utilsabort_on_panic {
() => {
// Module: crate::utils
// Provides: {"abort_on_panic"}
// Dependencies: {}
# [doc = " Calls a function and aborts if it panics."] # [doc = ""] # [doc = " This is useful in unsafe code where we can't recover from panics."] # [inline] pub (crate) fn abort_on_panic < T > (f : impl FnOnce () -> T) -> T { let bomb = Bomb ; let t = f () ; mem :: forget (bomb) ; t }
};
}
