// Generated macro for impl_10 (impl)
macro_rules! Depcrate_codec_errorimpl_10 {
() => {
// Module: crate::codec::error
// Provides: {"impl_10"}
// Dependencies: {}
impl fmt :: Display for SendError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { match * self { Self :: Connection (ref e) => e . fmt (fmt) , Self :: User (ref e) => e . fmt (fmt) , } } }
};
}
