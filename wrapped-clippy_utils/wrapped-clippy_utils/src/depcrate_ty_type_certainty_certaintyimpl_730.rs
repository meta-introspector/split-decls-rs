// Generated macro for impl_730 (impl)
macro_rules! Depcrate_ty_type_certainty_certaintyimpl_730 {
() => {
// Module: crate::ty::type_certainty::certainty
// Provides: {"impl_730"}
// Dependencies: {}
impl Meet for Certainty { fn meet (self , other : Self) -> Self { match (self , other) { (Certainty :: Uncertain , _) | (_ , Certainty :: Uncertain) => Certainty :: Uncertain , (Certainty :: Certain (lhs) , Certainty :: Certain (rhs)) => Certainty :: Certain (lhs . meet (rhs)) , (Certainty :: Certain (inner) , _) | (_ , Certainty :: Certain (inner)) => Certainty :: Certain (inner) , (Certainty :: Contradiction , Certainty :: Contradiction) => Certainty :: Contradiction , } } }
};
}
