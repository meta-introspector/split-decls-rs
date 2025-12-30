// Generated macro for impl_30 (impl)
macro_rules! Depcrate_exprimpl_30 {
() => {
// Module: crate::expr
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a > MulAssign < & 'a EvalResult > for EvalResult { fn mul_assign (& mut self , rhs : & 'a EvalResult) { use self :: EvalResult :: * ; * self = match (& * self , rhs) { (& Int (a) , & Int (b)) => Int (a * b) , (& Float (a) , & Int (b)) => Float (a * (b . 0 as f64)) , (& Int (a) , & Float (b)) => Float (a . 0 as f64 * b) , (& Float (a) , & Float (b)) => Float (a * b) , _ => Invalid , } ; } }
};
}
