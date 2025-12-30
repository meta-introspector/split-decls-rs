// Generated macro for impl_147 (impl)
macro_rules! Depcrate_common_intrinsic_helpersimpl_147 {
() => {
// Module: crate::common::intrinsic_helpers
// Provides: {"impl_147"}
// Dependencies: {}
impl FromStr for TypeKind { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "bfloat" | "BF16" => Ok (Self :: BFloat) , "float" | "double" | "FP16" | "FP32" | "FP64" => Ok (Self :: Float) , "int" | "long" | "short" | "SI8" | "SI16" | "SI32" | "SI64" => { Ok (Self :: Int (Sign :: Signed)) } "poly" => Ok (Self :: Poly) , "char" => Ok (Self :: Char (Sign :: Signed)) , "uint" | "unsigned" | "UI8" | "UI16" | "UI32" | "UI64" => Ok (Self :: Int (Sign :: Unsigned)) , "void" => Ok (Self :: Void) , "MASK" => Ok (Self :: Mask) , "M128" | "M256" | "M512" => Ok (Self :: Vector) , _ => Err (format ! ("Impossible to parse argument kind {s}")) , } } }
};
}
