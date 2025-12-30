// Generated macro for impl_150 (impl)
macro_rules! Depcrate_common_intrinsic_helpersimpl_150 {
() => {
// Module: crate::common::intrinsic_helpers
// Provides: {"impl_150"}
// Dependencies: {}
impl TypeKind { # [doc = " Gets the type part of a c typedef for a type that's in the form of {type}{size}_t."] pub fn c_prefix (& self) -> & str { match self { Self :: Float => "float" , Self :: Int (Sign :: Signed) => "int" , Self :: Int (Sign :: Unsigned) => "uint" , Self :: Poly => "poly" , Self :: Char (Sign :: Signed) => "char" , _ => unreachable ! ("Not used: {:#?}" , self) , } } # [doc = " Gets the rust prefix for the type kind i.e. i, u, f."] pub fn rust_prefix (& self) -> & str { match self { Self :: BFloat => "bf" , Self :: Float => "f" , Self :: Int (Sign :: Signed) => "i" , Self :: Int (Sign :: Unsigned) => "u" , Self :: Poly => "u" , Self :: Char (Sign :: Unsigned) => "u" , Self :: Char (Sign :: Signed) => "i" , _ => unreachable ! ("Unused type kind: {:#?}" , self) , } } }
};
}
