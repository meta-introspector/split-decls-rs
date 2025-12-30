// Generated macro for impl_27 (impl)
macro_rules! Depcrate_exprimpl_27 {
() => {
// Module: crate::expr
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a > BitOrAssign < & 'a EvalResult > for EvalResult { fn bitor_assign (& mut self , rhs : & 'a EvalResult) { use self :: EvalResult :: * ; * self = match (& * self , rhs) { (& Int (a) , & Int (b)) => Int (a | b) , _ => Invalid , } ; } }
};
}
