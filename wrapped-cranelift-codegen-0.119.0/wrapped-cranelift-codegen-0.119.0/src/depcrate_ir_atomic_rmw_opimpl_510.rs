// Generated macro for impl_510 (impl)
macro_rules! Depcrate_ir_atomic_rmw_opimpl_510 {
() => {
// Module: crate::ir::atomic_rmw_op
// Provides: {"impl_510"}
// Dependencies: {}
impl Display for AtomicRmwOp { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { let s = match self { AtomicRmwOp :: Add => "add" , AtomicRmwOp :: Sub => "sub" , AtomicRmwOp :: And => "and" , AtomicRmwOp :: Nand => "nand" , AtomicRmwOp :: Or => "or" , AtomicRmwOp :: Xor => "xor" , AtomicRmwOp :: Xchg => "xchg" , AtomicRmwOp :: Umin => "umin" , AtomicRmwOp :: Umax => "umax" , AtomicRmwOp :: Smin => "smin" , AtomicRmwOp :: Smax => "smax" , } ; f . write_str (s) } }
};
}
