// Generated macro for asm_const_to_str (function)
macro_rules! Depcrate_commonasm_const_to_str {
() => {
// Module: crate::common
// Provides: {"asm_const_to_str"}
// Dependencies: {}
pub fn asm_const_to_str < 'tcx > (tcx : TyCtxt < 'tcx > , sp : Span , const_value : mir :: ConstValue , ty_and_layout : TyAndLayout < 'tcx > ,) -> String { let mir :: ConstValue :: Scalar (scalar) = const_value else { span_bug ! (sp , "expected Scalar for promoted asm const, but got {:#?}" , const_value) } ; let value = scalar . assert_scalar_int () . to_bits (ty_and_layout . size) ; match ty_and_layout . ty . kind () { ty :: Uint (_) => value . to_string () , ty :: Int (int_ty) => match int_ty . normalize (tcx . sess . target . pointer_width) { ty :: IntTy :: I8 => (value as i8) . to_string () , ty :: IntTy :: I16 => (value as i16) . to_string () , ty :: IntTy :: I32 => (value as i32) . to_string () , ty :: IntTy :: I64 => (value as i64) . to_string () , ty :: IntTy :: I128 => (value as i128) . to_string () , ty :: IntTy :: Isize => unreachable ! () , } , _ => span_bug ! (sp , "asm const has bad type {}" , ty_and_layout . ty) , } }
};
}
