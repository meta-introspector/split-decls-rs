// Generated macro for impl_43 (impl)
macro_rules! Depcrate_bindingsimpl_43 {
() => {
// Module: crate::bindings
// Provides: {"impl_43"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType > Iterator for IIterator < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { let result = if self . HasCurrent () . unwrap_or (false) { self . Current () . ok () } else { None } ; if result . is_some () { self . MoveNext () . ok () ? ; } result } }
};
}
