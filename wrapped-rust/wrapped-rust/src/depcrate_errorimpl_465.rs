// Generated macro for impl_465 (impl)
macro_rules! Depcrate_errorimpl_465 {
() => {
// Module: crate::error
// Provides: {"impl_465"}
// Dependencies: {}
impl From < pyo3 :: CastError < '_ , '_ > > for CryptographyError { fn from (e : pyo3 :: CastError < '_ , '_ >) -> CryptographyError { CryptographyError :: Py (e . into ()) } }
};
}
