// Generated macro for impl_26 (impl)
macro_rules! Depcrate_exprimpl_26 {
() => {
// Module: crate::expr
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a > BitAndAssign < & 'a EvalResult > for EvalResult { fn bitand_assign (& mut self , rhs : & 'a EvalResult) { use self :: EvalResult :: * ; * self = match (& * self , rhs) { (& Int (a) , & Int (b)) => Int (a & b) , _ => Invalid , } ; } }
};
}
