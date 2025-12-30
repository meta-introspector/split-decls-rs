// Generated macro for rvalue_locals (function)
macro_rules! Depcrate_mir_possible_borrowerrvalue_locals {
() => {
// Module: crate::mir::possible_borrower
// Provides: {"rvalue_locals"}
// Dependencies: {}
fn rvalue_locals (rvalue : & mir :: Rvalue < '_ > , mut visit : impl FnMut (mir :: Local)) { use rustc_middle :: mir :: Rvalue :: { Aggregate , BinaryOp , Cast , Repeat , UnaryOp , Use } ; let mut visit_op = | op : & mir :: Operand < '_ > | match op { mir :: Operand :: Copy (p) | mir :: Operand :: Move (p) => visit (p . local) , mir :: Operand :: Constant (..) => () , } ; match rvalue { Use (op) | Repeat (op , _) | Cast (_ , op , _) | UnaryOp (_ , op) => visit_op (op) , Aggregate (_ , ops) => ops . iter () . for_each (visit_op) , BinaryOp (_ , box (lhs , rhs)) => { visit_op (lhs) ; visit_op (rhs) ; } , _ => () , } }
};
}
