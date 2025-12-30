// Generated macro for impl_25 (impl)
macro_rules! Depcrate_operatorsimpl_25 {
() => {
// Module: crate::operators
// Provides: {"impl_25"}
// Dependencies: {}
impl ops :: BitOr < Term > for PartialConstraint { type Output = Constraint ; fn bitor (self , rhs : Term) -> Constraint { let (op , s) = self . 1 . into () ; Constraint :: new (self . 0 - rhs , op , s) } }
};
}
