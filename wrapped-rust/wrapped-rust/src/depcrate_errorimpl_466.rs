// Generated macro for impl_466 (impl)
macro_rules! Depcrate_errorimpl_466 {
() => {
// Module: crate::error
// Provides: {"impl_466"}
// Dependencies: {}
impl From < pyo3 :: CastIntoError < '_ > > for CryptographyError { fn from (e : pyo3 :: CastIntoError < '_ >) -> CryptographyError { CryptographyError :: Py (e . into ()) } }
};
}
