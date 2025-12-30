// Generated macro for install_panic_hook (function)
macro_rules! Depcrate_panic_hookinstall_panic_hook {
() => {
// Module: crate::panic_hook
// Provides: {"install_panic_hook"}
// Dependencies: {}
# [doc = " Installs a custom panic hook that will divert panic output to a thread-local"] # [doc = " capture buffer, but only for threads that have a capture buffer set."] # [doc = ""] # [doc = " Otherwise, the custom hook delegates to a copy of the default panic hook."] pub (crate) fn install_panic_hook () { let default_hook = panic :: take_hook () ; panic :: set_hook (Box :: new (move | info | custom_panic_hook (& default_hook , info))) ; }
};
}
