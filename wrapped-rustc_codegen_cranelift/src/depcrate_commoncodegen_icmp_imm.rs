// Generated macro for codegen_icmp_imm (function)
macro_rules! Depcrate_commoncodegen_icmp_imm {
() => {
// Module: crate::common
// Provides: {"codegen_icmp_imm"}
// Dependencies: {}
pub (crate) fn codegen_icmp_imm (fx : & mut FunctionCx < '_ , '_ , '_ > , intcc : IntCC , lhs : Value , rhs : i128 ,) -> Value { let lhs_ty = fx . bcx . func . dfg . value_type (lhs) ; if lhs_ty == types :: I128 { let (lhs_lsb , lhs_msb) = fx . bcx . ins () . isplit (lhs) ; let (rhs_lsb , rhs_msb) = (rhs as u128 as u64 as i64 , (rhs as u128 >> 64) as u64 as i64) ; match intcc { IntCC :: Equal => { let lsb_eq = fx . bcx . ins () . icmp_imm (IntCC :: Equal , lhs_lsb , rhs_lsb) ; let msb_eq = fx . bcx . ins () . icmp_imm (IntCC :: Equal , lhs_msb , rhs_msb) ; fx . bcx . ins () . band (lsb_eq , msb_eq) } IntCC :: NotEqual => { let lsb_ne = fx . bcx . ins () . icmp_imm (IntCC :: NotEqual , lhs_lsb , rhs_lsb) ; let msb_ne = fx . bcx . ins () . icmp_imm (IntCC :: NotEqual , lhs_msb , rhs_msb) ; fx . bcx . ins () . bor (lsb_ne , msb_ne) } _ => { let msb_eq = fx . bcx . ins () . icmp_imm (IntCC :: Equal , lhs_msb , rhs_msb) ; let lsb_cc = fx . bcx . ins () . icmp_imm (intcc , lhs_lsb , rhs_lsb) ; let msb_cc = fx . bcx . ins () . icmp_imm (intcc , lhs_msb , rhs_msb) ; fx . bcx . ins () . select (msb_eq , lsb_cc , msb_cc) } } } else { let rhs = rhs as i64 ; fx . bcx . ins () . icmp_imm (intcc , lhs , rhs) } }
};
}
