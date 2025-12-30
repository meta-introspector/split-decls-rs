// Generated macro for impl_42 (impl)
macro_rules! Depcrate_signatureimpl_42 {
() => {
// Module: crate::signature
// Provides: {"impl_42"}
// Dependencies: {}
impl fmt :: Display for JavaType { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { JavaType :: Primitive (ref ty) => ty . fmt (f) , JavaType :: Object => write ! (f , "L;") , JavaType :: Array => write ! (f , "[") , } } }
};
}
