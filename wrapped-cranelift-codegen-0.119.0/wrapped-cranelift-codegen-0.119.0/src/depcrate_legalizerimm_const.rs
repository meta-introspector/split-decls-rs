// Generated macro for imm_const (function)
macro_rules! Depcrate_legalizerimm_const {
() => {
// Module: crate::legalizer
// Provides: {"imm_const"}
// Dependencies: {}
fn imm_const (pos : & mut FuncCursor , arg : Value , imm : Imm64 , is_signed : bool) -> Value { let ty = pos . func . dfg . value_type (arg) ; match (ty , is_signed) { (I128 , true) => { let imm = pos . ins () . iconst (I64 , imm) ; pos . ins () . sextend (I128 , imm) } (I128 , false) => { let imm = pos . ins () . iconst (I64 , imm) ; pos . ins () . uextend (I128 , imm) } _ => { let bits = imm . bits () ; let unsigned = match ty . lane_type () { types :: I8 => bits as u8 as i64 , types :: I16 => bits as u16 as i64 , types :: I32 => bits as u32 as i64 , types :: I64 => bits , _ => unreachable ! () , } ; pos . ins () . iconst (ty . lane_type () , unsigned) } } }
};
}
