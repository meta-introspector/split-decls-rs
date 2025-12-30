// Generated macro for UdpSockRef (struct)
macro_rules! DepcrateUdpSockRef {
() => {
// Module: crate
// Provides: {"UdpSockRef"}
// Dependencies: {}
# [doc = " A borrowed UDP socket"] # [doc = ""] # [doc = " On Unix, constructible via `From<T: AsFd>`. On Windows, constructible via `From<T:"] # [doc = " AsSocket>`."] # [cfg (not (wasm_browser))] pub struct UdpSockRef < 'a > (socket2 :: SockRef < 'a >) ;
};
}
