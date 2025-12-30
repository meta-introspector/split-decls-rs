// Generated macro for impl_32 (impl)
macro_rules! Depcrate_exprimpl_32 {
() => {
// Module: crate::expr
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a > ShlAssign < & 'a EvalResult > for EvalResult { fn shl_assign (& mut self , rhs : & 'a EvalResult) { use self :: EvalResult :: * ; * self = match (& * self , rhs) { (& Int (a) , & Int (b)) => Int (a << (b . 0 as usize)) , _ => Invalid , } ; } }
};
}
