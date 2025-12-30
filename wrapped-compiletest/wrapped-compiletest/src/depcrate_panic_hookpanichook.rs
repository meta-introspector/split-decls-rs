// Generated macro for PanicHook (type)
macro_rules! Depcrate_panic_hookPanicHook {
() => {
// Module: crate::panic_hook
// Provides: {"PanicHook"}
// Dependencies: {}
type PanicHook = Box < dyn Fn (& PanicHookInfo < '_ >) + Sync + Send + 'static > ;
};
}
