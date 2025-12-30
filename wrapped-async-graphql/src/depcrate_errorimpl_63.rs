// Generated macro for impl_63 (impl)
macro_rules! Depcrate_errorimpl_63 {
() => {
// Module: crate::error
// Provides: {"impl_63"}
// Dependencies: {}
impl From < mime :: FromStrError > for ParseRequestError { fn from (e : mime :: FromStrError) -> Self { Self :: InvalidRequest (Box :: new (e)) } }
};
}
