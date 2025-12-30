// Generated macro for impl_161 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_161 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_161"}
// Dependencies: {}
impl ConnectingTcp < '_ > { async fn connect (mut self) -> Result < TcpStream , ConnectError > { match self . fallback { None => self . preferred . connect (self . config) . await , Some (mut fallback) => { let preferred_fut = self . preferred . connect (self . config) ; futures_util :: pin_mut ! (preferred_fut) ; let fallback_fut = fallback . remote . connect (self . config) ; futures_util :: pin_mut ! (fallback_fut) ; let fallback_delay = fallback . delay ; futures_util :: pin_mut ! (fallback_delay) ; let (result , future) = match futures_util :: future :: select (preferred_fut , fallback_delay) . await { Either :: Left ((result , _fallback_delay)) => { (result , Either :: Right (fallback_fut)) } Either :: Right ((() , preferred_fut)) => { futures_util :: future :: select (preferred_fut , fallback_fut) . await . factor_first () } } ; if result . is_err () { future . await } else { result } } } } }
};
}
