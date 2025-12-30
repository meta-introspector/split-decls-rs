// Generated macro for impl_95 (impl)
macro_rules! Depcrate_devimpl_95 {
() => {
// Module: crate::dev
// Provides: {"impl_95"}
// Dependencies: {}
impl TryFrom < U256 > for Scalar { type Error = Error ; fn try_from (w : U256) -> Result < Self > { ScalarValue :: new (w) . into_option () . map (Self) . ok_or (Error) } }
};
}
