// Generated macro for impl_930 (impl)
macro_rules! Depcrate_http_websocketimpl_930 {
() => {
// Module: crate::http::websocket
// Provides: {"impl_930"}
// Dependencies: {}
impl Timer { # [inline] fn new (interval : Duration) -> Self { Self { interval , delay : Delay :: new (interval) , } } # [inline] fn reset (& mut self) { self . delay . reset (self . interval) ; } }
};
}
