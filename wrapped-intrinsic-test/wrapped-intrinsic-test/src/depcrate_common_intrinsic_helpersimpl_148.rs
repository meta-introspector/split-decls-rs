// Generated macro for impl_148 (impl)
macro_rules! Depcrate_common_intrinsic_helpersimpl_148 {
() => {
// Module: crate::common::intrinsic_helpers
// Provides: {"impl_148"}
// Dependencies: {}
impl fmt :: Display for TypeKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , match self { Self :: BFloat => "bfloat" , Self :: Float => "float" , Self :: Int (Sign :: Signed) => "int" , Self :: Int (Sign :: Unsigned) => "uint" , Self :: Poly => "poly" , Self :: Void => "void" , Self :: Char (Sign :: Signed) => "char" , Self :: Char (Sign :: Unsigned) => "unsigned char" , Self :: Mask => "mask" , Self :: Vector => "vector" , }) } }
};
}
