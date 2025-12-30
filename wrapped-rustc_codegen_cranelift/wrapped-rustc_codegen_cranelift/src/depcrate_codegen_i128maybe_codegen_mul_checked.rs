// Generated macro for maybe_codegen_mul_checked (function)
macro_rules! Depcrate_codegen_i128maybe_codegen_mul_checked {
() => {
// Module: crate::codegen_i128
// Provides: {"maybe_codegen_mul_checked"}
// Dependencies: {}
pub (crate) fn maybe_codegen_mul_checked < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , lhs : CValue < 'tcx > , rhs : CValue < 'tcx > ,) -> Option < CValue < 'tcx > > { if lhs . layout () . ty != fx . tcx . types . u128 && lhs . layout () . ty != fx . tcx . types . i128 && rhs . layout () . ty != fx . tcx . types . u128 && rhs . layout () . ty != fx . tcx . types . i128 { return None ; } let is_signed = type_sign (lhs . layout () . ty) ; let oflow_out_place = CPlace :: new_stack_slot (fx , fx . layout_of (fx . tcx . types . i32)) ; let param_types = vec ! [AbiParam :: new (types :: I128) , AbiParam :: new (types :: I128) , AbiParam :: special (fx . pointer_type , ArgumentPurpose :: Normal) ,] ; let args = [lhs . load_scalar (fx) , rhs . load_scalar (fx) , oflow_out_place . to_ptr () . get_addr (fx)] ; let ret = fx . lib_call (if is_signed { "__rust_i128_mulo" } else { "__rust_u128_mulo" } , param_types , vec ! [AbiParam :: new (types :: I128)] , & args ,) ; let mul = ret [0] ; let oflow = oflow_out_place . to_cvalue (fx) . load_scalar (fx) ; let oflow = clif_intcast (fx , oflow , types :: I8 , false) ; let layout = fx . layout_of (Ty :: new_tup (fx . tcx , & [lhs . layout () . ty , fx . tcx . types . bool])) ; Some (CValue :: by_val_pair (mul , oflow , layout)) }
};
}
