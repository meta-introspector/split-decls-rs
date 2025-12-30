// Generated macro for impl_22 (impl)
macro_rules! Depcrate_operatorsimpl_22 {
() => {
// Module: crate::operators
// Provides: {"impl_22"}
// Dependencies: {}
impl ops :: BitOr < f64 > for PartialConstraint { type Output = Constraint ; fn bitor (self , rhs : f64) -> Constraint { let (op , s) = self . 1 . into () ; Constraint :: new (self . 0 - rhs , op , s) } }
};
}
