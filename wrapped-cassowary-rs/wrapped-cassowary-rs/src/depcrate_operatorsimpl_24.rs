// Generated macro for impl_24 (impl)
macro_rules! Depcrate_operatorsimpl_24 {
() => {
// Module: crate::operators
// Provides: {"impl_24"}
// Dependencies: {}
impl ops :: BitOr < Variable > for PartialConstraint { type Output = Constraint ; fn bitor (self , rhs : Variable) -> Constraint { let (op , s) = self . 1 . into () ; Constraint :: new (self . 0 - rhs , op , s) } }
};
}
