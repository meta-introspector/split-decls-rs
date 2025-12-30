// Generated macro for to_genmc_rmw_op (function)
macro_rules! Depcrate_concurrency_genmc_helperto_genmc_rmw_op {
() => {
// Module: crate::concurrency::genmc::helper
// Provides: {"to_genmc_rmw_op"}
// Dependencies: {}
# [doc = " Convert an atomic binary operation to its GenMC counterpart."] pub (super) fn to_genmc_rmw_op (atomic_op : AtomicRmwOp , is_signed : bool) -> RMWBinOp { match (atomic_op , is_signed) { (AtomicRmwOp :: Min , true) => RMWBinOp :: Min , (AtomicRmwOp :: Max , true) => RMWBinOp :: Max , (AtomicRmwOp :: Min , false) => RMWBinOp :: UMin , (AtomicRmwOp :: Max , false) => RMWBinOp :: UMax , (AtomicRmwOp :: MirOp { op , neg } , _is_signed) => match (op , neg) { (mir :: BinOp :: Add , false) => RMWBinOp :: Add , (mir :: BinOp :: Sub , false) => RMWBinOp :: Sub , (mir :: BinOp :: BitXor , false) => RMWBinOp :: Xor , (mir :: BinOp :: BitAnd , false) => RMWBinOp :: And , (mir :: BinOp :: BitAnd , true) => RMWBinOp :: Nand , (mir :: BinOp :: BitOr , false) => RMWBinOp :: Or , _ => { panic ! ("unsupported atomic operation: bin_op: {op:?}, negate: {neg}") ; } } , } }
};
}
