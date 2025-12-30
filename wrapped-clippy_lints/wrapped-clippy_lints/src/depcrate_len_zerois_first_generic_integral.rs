// Generated macro for is_first_generic_integral (function)
macro_rules! Depcrate_len_zerois_first_generic_integral {
() => {
// Module: crate::len_zero
// Provides: {"is_first_generic_integral"}
// Dependencies: {}
fn is_first_generic_integral < 'tcx > (segment : & 'tcx PathSegment < 'tcx >) -> bool { if let Some (generic_args) = segment . args && let [GenericArg :: Type (ty) , ..] = & generic_args . args && let TyKind :: Path (QPath :: Resolved (_ , path)) = ty . kind && let [segment , ..] = & path . segments && matches ! (segment . res , Res :: PrimTy (PrimTy :: Uint (_) | PrimTy :: Int (_))) { true } else { false } }
};
}
