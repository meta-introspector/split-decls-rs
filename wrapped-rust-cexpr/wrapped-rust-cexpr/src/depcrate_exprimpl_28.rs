// Generated macro for impl_28 (impl)
macro_rules! Depcrate_exprimpl_28 {
() => {
// Module: crate::expr
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a > BitXorAssign < & 'a EvalResult > for EvalResult { fn bitxor_assign (& mut self , rhs : & 'a EvalResult) { use self :: EvalResult :: * ; * self = match (& * self , rhs) { (& Int (a) , & Int (b)) => Int (a ^ b) , _ => Invalid , } ; } }
};
}
