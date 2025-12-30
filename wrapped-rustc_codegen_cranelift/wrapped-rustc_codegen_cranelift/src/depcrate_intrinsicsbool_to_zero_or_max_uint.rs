// Generated macro for bool_to_zero_or_max_uint (function)
macro_rules! Depcrate_intrinsicsbool_to_zero_or_max_uint {
() => {
// Module: crate::intrinsics
// Provides: {"bool_to_zero_or_max_uint"}
// Dependencies: {}
fn bool_to_zero_or_max_uint < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , ty : Ty < 'tcx > , val : Value ,) -> Value { let ty = fx . clif_type (ty) . unwrap () ; let int_ty = match ty { types :: F16 => types :: I16 , types :: F32 => types :: I32 , types :: F64 => types :: I64 , types :: F128 => types :: I128 , ty => ty , } ; let mut res = fx . bcx . ins () . bmask (int_ty , val) ; if ty . is_float () { res = codegen_bitcast (fx , ty , res) ; } res }
};
}
