// Generated macro for raw_yield_now (function)
macro_rules! Depcrate_yield_raw_yield_now {
() => {
// Module: crate::yield_
// Provides: {"raw_yield_now"}
// Dependencies: {}
# [inline] pub fn raw_yield_now (env : & ContextStack , cur : & mut Context) { let parent = env . pop_context (cur as * mut _) ; RegContext :: swap (& mut cur . regs , & parent . regs) ; }
};
}
