// Generated macro for impl_269 (impl)
macro_rules! Depcrate_stringimpl_269 {
() => {
// Module: crate::string
// Provides: {"impl_269"}
// Dependencies: {}
impl fmt :: Display for FromUtf16Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Capacity (err) => write ! (f , "{err}") , Self :: DecodeUtf16 (err) => write ! (f , "invalid UTF-16: {err}") , } } }
};
}
