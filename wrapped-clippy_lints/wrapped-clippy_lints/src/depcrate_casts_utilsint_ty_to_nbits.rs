// Generated macro for int_ty_to_nbits (function)
macro_rules! Depcrate_casts_utilsint_ty_to_nbits {
() => {
// Module: crate::casts::utils
// Provides: {"int_ty_to_nbits"}
// Dependencies: {}
# [doc = " Returns the size in bits of an integral type, or `None` if `ty` is not an"] # [doc = " integral type."] pub (super) fn int_ty_to_nbits (tcx : TyCtxt < '_ > , ty : Ty < '_ >) -> Option < u64 > { match ty . kind () { ty :: Int (IntTy :: Isize) | ty :: Uint (UintTy :: Usize) => Some (tcx . data_layout . pointer_size () . bits ()) , ty :: Int (i) => i . bit_width () , ty :: Uint (i) => i . bit_width () , _ => None , } }
};
}
