// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorimpl_22 {
() => {
// Module: crate::error
// Provides: {"impl_22"}
// Dependencies: {}
impl AdhocError { fn from_args < 'a > (message : core :: fmt :: Arguments < 'a >) -> AdhocError { # [cfg (feature = "alloc")] { AdhocError :: from_display (message) } # [cfg (not (feature = "alloc"))] { let message = message . as_str () . unwrap_or ("unknown `jiff-icu` error (better error messages require \
                 enabling the `alloc` feature for the `jiff-icu` crate)" ,) ; AdhocError :: from_static_str (message) } } # [cfg (feature = "alloc")] fn from_display < 'a > (message : impl core :: fmt :: Display + 'a) -> AdhocError { use alloc :: string :: ToString ; let message = message . to_string () . into_boxed_str () ; AdhocError { message } } # [cfg (not (feature = "alloc"))] fn from_static_str (message : & 'static str) -> AdhocError { AdhocError { message } } }
};
}
