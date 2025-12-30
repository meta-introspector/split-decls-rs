// Generated macro for co_set_para (function)
macro_rules! Depcrate_yield_co_set_para {
() => {
// Module: crate::yield_
// Provides: {"co_set_para"}
// Dependencies: {}
# [doc = " set current coroutine para in user space"] pub fn co_set_para < A : Any > (para : A) { if let Some (ctx) = ContextStack :: current () . co_ctx () { ctx . co_set_para (para) } }
};
}
