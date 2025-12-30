// Generated macro for ensure_sufficient_stack (function)
macro_rules! Depcrate_stackensure_sufficient_stack {
() => {
// Module: crate::stack
// Provides: {"ensure_sufficient_stack"}
// Dependencies: {}
# [doc = " Grows the stack on demand to prevent stack overflow. Call this in strategic locations"] # [doc = " to \"break up\" recursive calls. E.g. almost any call to `visit_expr` or equivalent can benefit"] # [doc = " from this."] # [doc = ""] # [doc = " Should not be sprinkled around carelessly, as it causes a little bit of overhead."] # [inline] pub fn ensure_sufficient_stack < R > (f : impl FnOnce () -> R) -> R { stacker :: maybe_grow (RED_ZONE , STACK_PER_RECURSION , f) }
};
}
