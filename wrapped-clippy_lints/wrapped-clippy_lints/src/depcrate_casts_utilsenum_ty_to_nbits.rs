// Generated macro for enum_ty_to_nbits (function)
macro_rules! Depcrate_casts_utilsenum_ty_to_nbits {
() => {
// Module: crate::casts::utils
// Provides: {"enum_ty_to_nbits"}
// Dependencies: {}
pub (super) fn enum_ty_to_nbits (adt : AdtDef < '_ > , tcx : TyCtxt < '_ >) -> u64 { let mut explicit = 0i128 ; let (start , end) = adt . variants () . iter () . fold ((0 , i128 :: MIN) , | (start , end) , variant | match variant . discr { VariantDiscr :: Relative (x) => match explicit . checked_add (i128 :: from (x)) { Some (x) => (start , end . max (x)) , None => (i128 :: MIN , end) , } , VariantDiscr :: Explicit (id) => match read_explicit_enum_value (tcx , id) { Some (EnumValue :: Signed (x)) => { explicit = x ; (start . min (x) , end . max (x)) } , Some (EnumValue :: Unsigned (x)) => match i128 :: try_from (x) { Ok (x) => { explicit = x ; (start , end . max (x)) } , Err (_) => (i128 :: MIN , end) , } , None => (start , end) , } , }) ; if start > end { 0 } else { let neg_bits = if start < 0 { 128 - (- (start + 1)) . leading_zeros () + 1 } else { 0 } ; let pos_bits = if end > 0 { 128 - end . leading_zeros () } else { 0 } ; neg_bits . max (pos_bits) . into () } }
};
}
