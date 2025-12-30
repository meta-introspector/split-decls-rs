// Generated macro for impl_40 (impl)
macro_rules! Depcrate_errorimpl_40 {
() => {
// Module: crate::error
// Provides: {"impl_40"}
// Dependencies: {}
impl fmt :: Display for ImageError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match self { ImageError :: IoError (err) => err . fmt (fmt) , ImageError :: Decoding (err) => err . fmt (fmt) , ImageError :: Encoding (err) => err . fmt (fmt) , ImageError :: Parameter (err) => err . fmt (fmt) , ImageError :: Limits (err) => err . fmt (fmt) , ImageError :: Unsupported (err) => err . fmt (fmt) , } } }
};
}
