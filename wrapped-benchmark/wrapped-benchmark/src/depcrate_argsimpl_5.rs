// Generated macro for impl_5 (impl)
macro_rules! Depcrate_argsimpl_5 {
() => {
// Module: crate::args
// Provides: {"impl_5"}
// Dependencies: {}
impl Iterator for ArgRange { type Item = usize ; fn next (& mut self) -> Option < usize > { if self . current <= self . limit { let result = self . current ; self . current = self . current . saturating_add (self . step) ; Some (result) } else { None } } }
};
}
