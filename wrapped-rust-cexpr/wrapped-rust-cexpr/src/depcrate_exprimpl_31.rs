// Generated macro for impl_31 (impl)
macro_rules! Depcrate_exprimpl_31 {
() => {
// Module: crate::expr
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a > RemAssign < & 'a EvalResult > for EvalResult { fn rem_assign (& mut self , rhs : & 'a EvalResult) { use self :: EvalResult :: * ; * self = match (& * self , rhs) { (& Int (a) , & Int (b)) => Int (a % b) , (& Float (a) , & Int (b)) => Float (a % (b . 0 as f64)) , (& Int (a) , & Float (b)) => Float (a . 0 as f64 % b) , (& Float (a) , & Float (b)) => Float (a % b) , _ => Invalid , } ; } }
};
}
