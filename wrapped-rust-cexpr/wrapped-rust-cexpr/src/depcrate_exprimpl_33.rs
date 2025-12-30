// Generated macro for impl_33 (impl)
macro_rules! Depcrate_exprimpl_33 {
() => {
// Module: crate::expr
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a > ShrAssign < & 'a EvalResult > for EvalResult { fn shr_assign (& mut self , rhs : & 'a EvalResult) { use self :: EvalResult :: * ; * self = match (& * self , rhs) { (& Int (a) , & Int (b)) => Int (a >> (b . 0 as usize)) , _ => Invalid , } ; } }
};
}
