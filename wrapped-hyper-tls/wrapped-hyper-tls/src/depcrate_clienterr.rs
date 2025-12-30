// Generated macro for err (function)
macro_rules! Depcrate_clienterr {
() => {
// Module: crate::client
// Provides: {"err"}
// Dependencies: {}
fn err < T > (e : BoxError) -> HttpsConnecting < T > { HttpsConnecting (Box :: pin (async { Err (e) })) }
};
}
