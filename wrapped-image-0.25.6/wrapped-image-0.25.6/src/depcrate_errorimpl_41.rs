// Generated macro for impl_41 (impl)
macro_rules! Depcrate_errorimpl_41 {
() => {
// Module: crate::error
// Provides: {"impl_41"}
// Dependencies: {}
impl Error for ImageError { fn source (& self) -> Option < & (dyn Error + 'static) > { match self { ImageError :: IoError (err) => err . source () , ImageError :: Decoding (err) => err . source () , ImageError :: Encoding (err) => err . source () , ImageError :: Parameter (err) => err . source () , ImageError :: Limits (err) => err . source () , ImageError :: Unsupported (err) => err . source () , } } }
};
}
