// Generated macro for impl_16 (impl)
macro_rules! Depcrate_errorimpl_16 {
() => {
// Module: crate::error
// Provides: {"impl_16"}
// Dependencies: {}
impl Error { # [doc = " Creates an error from an arbitrary `core::fmt::Arguments`."] # [doc = ""] # [doc = " When `alloc` isn't enabled, then `Arguments::as_str()` is used to"] # [doc = " find an error message. Otherwise, a generic error message is emitted."] pub (crate) fn adhoc_from_args < 'a > (message : core :: fmt :: Arguments < 'a > ,) -> Error { let kind = ErrorKind :: Adhoc (AdhocError :: from_args (message)) ; Error { kind } } }
};
}
