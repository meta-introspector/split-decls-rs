// Generated macro for method_call_await (function)
macro_rules! Depcrate_nonblockmethod_call_await {
() => {
// Module: crate::nonblock
// Provides: {"method_call_await"}
// Dependencies: {}
async fn method_call_await (mra : MRAwait) -> Result < Message , Error > { use futures_util :: future ; let MRAwait { mrouter , token , timeout , timeoutfn } = mra ; if token . is_err () { return Err (Error :: new_failed ("Failed to send message")) } ; let timeout = if let Some (tfn) = timeoutfn { tfn (timeout) } else { Box :: pin (future :: pending ()) } ; match future :: select (mrouter , timeout) . await { future :: Either :: Left ((r , _)) => r , future :: Either :: Right (_) => Err (Error :: new_custom ("org.freedesktop.DBus.Error.Timeout" , "Timeout waiting for reply")) , } }
};
}
