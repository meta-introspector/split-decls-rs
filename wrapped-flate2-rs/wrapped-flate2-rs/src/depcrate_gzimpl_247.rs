// Generated macro for impl_247 (impl)
macro_rules! Depcrate_gzimpl_247 {
() => {
// Module: crate::gz
// Provides: {"impl_247"}
// Dependencies: {}
impl From < GzHeaderParser > for GzHeader { fn from (parser : GzHeaderParser) -> Self { debug_assert ! (matches ! (parser . state , GzHeaderState :: Complete)) ; parser . header } }
};
}
