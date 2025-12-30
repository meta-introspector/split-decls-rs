// Generated macro for impl_285 (impl)
macro_rules! Depcrate_errorimpl_285 {
() => {
// Module: crate::error
// Provides: {"impl_285"}
// Dependencies: {}
impl AdhocError { # [cfg (feature = "alloc")] fn from_display < 'a > (message : impl core :: fmt :: Display + 'a) -> AdhocError { use alloc :: string :: ToString ; let message = message . to_string () . into_boxed_str () ; AdhocError { message } } fn from_args < 'a > (message : core :: fmt :: Arguments < 'a >) -> AdhocError { # [cfg (feature = "alloc")] { AdhocError :: from_display (message) } # [cfg (not (feature = "alloc"))] { let message = message . as_str () . unwrap_or ("unknown Jiff error (better error messages require \
                 enabling the `alloc` feature for the `jiff` crate)" ,) ; AdhocError :: from_static_str (message) } } fn from_static_str (message : & 'static str) -> AdhocError { # [cfg (feature = "alloc")] { AdhocError :: from_display (message) } # [cfg (not (feature = "alloc"))] { AdhocError { message } } } }
};
}
