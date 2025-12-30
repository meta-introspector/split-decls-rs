// Generated macro for get_local_data (function)
macro_rules! Depcrate_rtget_local_data {
() => {
// Module: crate::rt
// Provides: {"get_local_data"}
// Dependencies: {}
# [doc = " get the current context local data"] # [doc = " only coroutine support local data"] # [inline] pub fn get_local_data () -> * mut u8 { let env = ContextStack :: current () ; env . co_ctx () . map_or (ptr :: null_mut () , | ctx | ctx . local_data) }
};
}
