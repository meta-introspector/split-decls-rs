// Generated macro for UnwindModule (struct)
macro_rules! Depcrate_unwind_moduleUnwindModule {
() => {
// Module: crate::unwind_module
// Provides: {"UnwindModule"}
// Dependencies: {}
# [doc = " A wrapper around a [Module] which adds any defined function to the [UnwindContext]."] pub (crate) struct UnwindModule < T > { pub (crate) module : T , unwind_context : UnwindContext , }
};
}
