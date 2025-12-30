// Generated macro for impl_15 (impl)
macro_rules! Depcrate_forgiving_base64impl_15 {
() => {
// Module: crate::forgiving_base64
// Provides: {"impl_15"}
// Dependencies: {}
impl < E : fmt :: Display > fmt :: Display for DecodeError < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: InvalidBase64 (inner) => write ! (f , "base64 not valid: {}" , inner) , Self :: WriteError (err) => write ! (f , "write error: {}" , err) , } } }
};
}
