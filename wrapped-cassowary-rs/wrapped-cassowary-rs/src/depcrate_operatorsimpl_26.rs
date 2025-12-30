// Generated macro for impl_26 (impl)
macro_rules! Depcrate_operatorsimpl_26 {
() => {
// Module: crate::operators
// Provides: {"impl_26"}
// Dependencies: {}
impl ops :: BitOr < Expression > for PartialConstraint { type Output = Constraint ; fn bitor (self , rhs : Expression) -> Constraint { let (op , s) = self . 1 . into () ; Constraint :: new (self . 0 - rhs , op , s) } }
};
}
