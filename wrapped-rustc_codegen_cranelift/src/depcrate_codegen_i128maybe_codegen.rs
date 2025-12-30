// Generated macro for maybe_codegen (function)
macro_rules! Depcrate_codegen_i128maybe_codegen {
() => {
// Module: crate::codegen_i128
// Provides: {"maybe_codegen"}
// Dependencies: {}
pub (crate) fn maybe_codegen < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , bin_op : BinOp , lhs : CValue < 'tcx > , rhs : CValue < 'tcx > ,) -> Option < CValue < 'tcx > > { if lhs . layout () . ty != fx . tcx . types . u128 && lhs . layout () . ty != fx . tcx . types . i128 && rhs . layout () . ty != fx . tcx . types . u128 && rhs . layout () . ty != fx . tcx . types . i128 { return None ; } let is_signed = type_sign (lhs . layout () . ty) ; match bin_op { BinOp :: BitAnd | BinOp :: BitOr | BinOp :: BitXor => None , BinOp :: Add | BinOp :: AddUnchecked | BinOp :: Sub | BinOp :: SubUnchecked => None , BinOp :: Mul | BinOp :: MulUnchecked => None , BinOp :: Offset => unreachable ! ("offset should only be used on pointers, not 128bit ints") , BinOp :: Div | BinOp :: Rem => { let name = match (bin_op , is_signed) { (BinOp :: Div , false) => "__udivti3" , (BinOp :: Div , true) => "__divti3" , (BinOp :: Rem , false) => "__umodti3" , (BinOp :: Rem , true) => "__modti3" , _ => unreachable ! () , } ; let args = [lhs . load_scalar (fx) , rhs . load_scalar (fx)] ; let ret_val = fx . lib_call (name , vec ! [AbiParam :: new (types :: I128) , AbiParam :: new (types :: I128)] , vec ! [AbiParam :: new (types :: I128)] , & args ,) [0] ; Some (CValue :: by_val (ret_val , lhs . layout ())) } BinOp :: Lt | BinOp :: Le | BinOp :: Eq | BinOp :: Ge | BinOp :: Gt | BinOp :: Ne | BinOp :: Cmp => None , BinOp :: Shl | BinOp :: ShlUnchecked | BinOp :: Shr | BinOp :: ShrUnchecked => None , BinOp :: AddWithOverflow | BinOp :: SubWithOverflow | BinOp :: MulWithOverflow => unreachable ! () , } }
};
}
