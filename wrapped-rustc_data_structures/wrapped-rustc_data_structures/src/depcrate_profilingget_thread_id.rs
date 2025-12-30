// Generated macro for get_thread_id (function)
macro_rules! Depcrate_profilingget_thread_id {
() => {
// Module: crate::profiling
// Provides: {"get_thread_id"}
// Dependencies: {}
fn get_thread_id () -> u32 { std :: thread :: current () . id () . as_u64 () . get () as u32 }
};
}
