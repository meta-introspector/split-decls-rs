// Generated macro for scalar_to_clif_type (function)
macro_rules! Depcrate_commonscalar_to_clif_type {
() => {
// Module: crate::common
// Provides: {"scalar_to_clif_type"}
// Dependencies: {}
pub (crate) fn scalar_to_clif_type (tcx : TyCtxt < '_ > , scalar : Scalar) -> Type { match scalar . primitive () { Primitive :: Int (int , _sign) => match int { Integer :: I8 => types :: I8 , Integer :: I16 => types :: I16 , Integer :: I32 => types :: I32 , Integer :: I64 => types :: I64 , Integer :: I128 => types :: I128 , } , Primitive :: Float (float) => match float { Float :: F16 => types :: F16 , Float :: F32 => types :: F32 , Float :: F64 => types :: F64 , Float :: F128 => types :: F128 , } , Primitive :: Pointer (_) => pointer_ty (tcx) , } }
};
}
