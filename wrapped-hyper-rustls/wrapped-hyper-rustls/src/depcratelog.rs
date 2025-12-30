// Generated macro for log (module)
macro_rules! Depcratelog {
() => {
// Module: crate
// Provides: {"log"}
// Dependencies: {}
# [cfg (not (feature = "logging"))] mod log { # [cfg (any (feature = "rustls-native-certs" , feature = "webpki-roots"))] macro_rules ! debug (($ ($ tt : tt) *) => { { } }) ; # [cfg (any (feature = "rustls-native-certs" , feature = "webpki-roots"))] pub (crate) use debug ; # [cfg (feature = "rustls-native-certs")] macro_rules ! warn_ (($ ($ tt : tt) *) => { { } }) ; # [cfg (feature = "rustls-native-certs")] pub (crate) use warn_ as warn ; }
};
}
