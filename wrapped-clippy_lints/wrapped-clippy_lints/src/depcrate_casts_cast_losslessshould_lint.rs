// Generated macro for should_lint (function)
macro_rules! Depcrate_casts_cast_losslessshould_lint {
() => {
// Module: crate::casts::cast_lossless
// Provides: {"should_lint"}
// Dependencies: {}
fn should_lint (cx : & LateContext < '_ > , cast_from : Ty < '_ > , cast_to : Ty < '_ > , msrv : Msrv) -> bool { if is_in_const_context (cx) { return false ; } match (utils :: int_ty_to_nbits (cx . tcx , cast_from) , utils :: int_ty_to_nbits (cx . tcx , cast_to) ,) { (Some (from_nbits) , Some (to_nbits)) => { let cast_signed_to_unsigned = cast_from . is_signed () && ! cast_to . is_signed () ; ! is_isize_or_usize (cast_from) && ! is_isize_or_usize (cast_to) && from_nbits < to_nbits && ! cast_signed_to_unsigned } , (Some (from_nbits) , None) => { let to_nbits = if let ty :: Float (FloatTy :: F32) = cast_to . kind () { 32 } else { 64 } ; ! is_isize_or_usize (cast_from) && from_nbits < to_nbits } , (None , Some (_)) if cast_from . is_bool () && msrv . meets (cx , msrvs :: FROM_BOOL) => true , _ => matches ! (cast_from . kind () , ty :: Float (FloatTy :: F32)) && matches ! (cast_to . kind () , ty :: Float (FloatTy :: F64)) , } }
};
}
