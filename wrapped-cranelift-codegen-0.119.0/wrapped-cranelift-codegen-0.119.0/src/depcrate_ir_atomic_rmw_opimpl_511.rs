// Generated macro for impl_511 (impl)
macro_rules! Depcrate_ir_atomic_rmw_opimpl_511 {
() => {
// Module: crate::ir::atomic_rmw_op
// Provides: {"impl_511"}
// Dependencies: {}
impl FromStr for AtomicRmwOp { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "add" => Ok (AtomicRmwOp :: Add) , "sub" => Ok (AtomicRmwOp :: Sub) , "and" => Ok (AtomicRmwOp :: And) , "nand" => Ok (AtomicRmwOp :: Nand) , "or" => Ok (AtomicRmwOp :: Or) , "xor" => Ok (AtomicRmwOp :: Xor) , "xchg" => Ok (AtomicRmwOp :: Xchg) , "umin" => Ok (AtomicRmwOp :: Umin) , "umax" => Ok (AtomicRmwOp :: Umax) , "smin" => Ok (AtomicRmwOp :: Smin) , "smax" => Ok (AtomicRmwOp :: Smax) , _ => Err (()) , } } }
};
}
