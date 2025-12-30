// Generated macro for co_get_yield (function)
macro_rules! Depcrate_yield_co_get_yield {
() => {
// Module: crate::yield_
// Provides: {"co_get_yield"}
// Dependencies: {}
# [doc = " coroutine get passed in yield para"] pub fn co_get_yield < A : Any > () -> Option < A > { ContextStack :: current () . co_ctx () . and_then (| ctx | ctx . co_get_para ()) }
};
}
