// Generated macro for impl_93 (impl)
macro_rules! Depcrate_shared_util_errorimpl_93 {
() => {
// Module: crate::shared::util::error
// Provides: {"impl_93"}
// Dependencies: {}
impl Error { pub (crate) fn from_args < 'a > (message : core :: fmt :: Arguments < 'a >) -> Error { { use alloc :: string :: ToString ; let message = message . to_string () . into_boxed_str () ; Error { message } } } }
};
}
