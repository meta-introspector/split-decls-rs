// Generated macro for fp_ty_mantissa_nbits (function)
macro_rules! Depcrate_casts_unnecessary_castfp_ty_mantissa_nbits {
() => {
// Module: crate::casts::unnecessary_cast
// Provides: {"fp_ty_mantissa_nbits"}
// Dependencies: {}
# [doc = " Returns the mantissa bits wide of a fp type."] # [doc = " Will return 0 if the type is not a fp"] fn fp_ty_mantissa_nbits (typ : Ty < '_ >) -> u32 { match typ . kind () { ty :: Float (FloatTy :: F32) => 23 , ty :: Float (FloatTy :: F64) | ty :: Infer (InferTy :: FloatVar (_)) => 52 , _ => 0 , } }
};
}
