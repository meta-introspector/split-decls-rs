// Generated macro for impl_174 (impl)
macro_rules! Depcrate_encodingimpl_174 {
() => {
// Module: crate::encoding
// Provides: {"impl_174"}
// Dependencies: {}
impl std :: fmt :: Display for EncodingError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Utf8 (e) => write ! (f , "cannot decode input using UTF-8: {}" , e) , # [cfg (feature = "encoding")] Self :: Other (encoding) => write ! (f , "cannot decode input using {}" , encoding . name ()) , } } }
};
}
