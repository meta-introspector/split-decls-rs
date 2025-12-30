// Generated macro for impl_6 (impl)
macro_rules! Depcrate_linspaceimpl_6 {
() => {
// Module: crate::linspace
// Provides: {"impl_6"}
// Dependencies: {}
impl < F > DoubleEndedIterator for Linspace < F > where F : Float , { # [inline] fn next_back (& mut self) -> Option < F > { if self . index >= self . len { None } else { self . len -= 1 ; let i = self . len ; Some (self . start + self . step * F :: from (i) . unwrap ()) } } }
};
}
