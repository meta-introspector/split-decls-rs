// Generated macro for impl_696 (impl)
macro_rules! Depcrate_mirimpl_696 {
() => {
// Module: crate::mir
// Provides: {"impl_696"}
// Dependencies: {}
impl BinOp { fn run_compare < T : PartialEq + PartialOrd > (& self , l : T , r : T) -> bool { match self { BinOp :: Ge => l >= r , BinOp :: Gt => l > r , BinOp :: Le => l <= r , BinOp :: Lt => l < r , BinOp :: Eq => l == r , BinOp :: Ne => l != r , x => panic ! ("`run_compare` called on operator {x:?}") , } } }
};
}
