// Generated macro for set_panic_hook (function)
macro_rules! Depcrate_initset_panic_hook {
() => {
// Module: crate::init
// Provides: {"set_panic_hook"}
// Dependencies: {}
# [doc = " Sets a panic hook that restores the terminal before panicking."] # [doc = ""] # [doc = " Replaces the panic hook with a one that will restore the terminal state before calling the"] # [doc = " original panic hook. This ensures that the terminal is left in a good state when a panic occurs."] fn set_panic_hook () { let hook = std :: panic :: take_hook () ; std :: panic :: set_hook (alloc :: boxed :: Box :: new (move | info | { restore () ; hook (info) ; })) ; }
};
}
