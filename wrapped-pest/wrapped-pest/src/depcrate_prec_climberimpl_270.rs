// Generated macro for impl_270 (impl)
macro_rules! Depcrate_prec_climberimpl_270 {
() => {
// Module: crate::prec_climber
// Provides: {"impl_270"}
// Dependencies: {}
impl < R : RuleType > BitOr for Operator < R > { type Output = Self ; fn bitor (mut self , rhs : Self) -> Self { fn assign_next < R : RuleType > (op : & mut Operator < R > , next : Operator < R >) { if let Some (ref mut child) = op . next { assign_next (child , next) ; } else { op . next = Some (Box :: new (next)) ; } } assign_next (& mut self , rhs) ; self } }
};
}
