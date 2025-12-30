// Generated macro for impl_659 (impl)
macro_rules! Depcrate_shared_util_errorimpl_659 {
() => {
// Module: crate::shared::util::error
// Provides: {"impl_659"}
// Dependencies: {}
impl Error { pub (crate) fn from_args < 'a > (message : core :: fmt :: Arguments < 'a >) -> Error { # [cfg (feature = "alloc")] { use alloc :: string :: ToString ; let message = message . to_string () . into_boxed_str () ; Error { message } } # [cfg (not (feature = "alloc"))] { let message = message . as_str () . unwrap_or ("unknown Jiff error (better error messages require \
                 enabling the `alloc` feature for the `jiff` crate)" ,) ; Error { message } } } }
};
}
