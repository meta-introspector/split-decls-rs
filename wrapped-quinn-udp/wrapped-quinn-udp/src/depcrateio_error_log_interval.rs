// Generated macro for IO_ERROR_LOG_INTERVAL (const)
macro_rules! DepcrateIO_ERROR_LOG_INTERVAL {
() => {
// Module: crate
// Provides: {"IO_ERROR_LOG_INTERVAL"}
// Dependencies: {}
# [doc = " Log at most 1 IO error per minute"] # [cfg (not (wasm_browser))] const IO_ERROR_LOG_INTERVAL : Duration = std :: time :: Duration :: from_secs (60) ;
};
}
