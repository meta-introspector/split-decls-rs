// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl Config { # [doc = " Set timeout"] # [doc = ""] # [doc = " Timeout is the amount of time after which a debounced event is emitted or a continuous event is send, if there still are events incoming for the specific path."] pub fn with_timeout (mut self , timeout : Duration) -> Self { self . timeout = timeout ; self } # [doc = " Set batch mode"] # [doc = ""] # [doc = " When `batch_mode` is enabled, events may be delayed (at most 2x the specified timeout) and delivered with others."] # [doc = " If disabled, all events are delivered immediately when their debounce timeout is reached."] pub fn with_batch_mode (mut self , batch_mode : bool) -> Self { self . batch_mode = batch_mode ; self } # [doc = " Set [`notify::Config`] for the backend"] pub fn with_notify_config (mut self , notify_config : notify :: Config) -> Self { self . notify_config = notify_config ; self } }
};
}
