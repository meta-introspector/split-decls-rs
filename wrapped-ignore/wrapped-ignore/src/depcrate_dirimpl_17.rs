// Generated macro for impl_17 (impl)
macro_rules! Depcrate_dirimpl_17 {
() => {
// Module: crate::dir
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a > Iterator for Parents < 'a > { type Item = & 'a Ignore ; fn next (& mut self) -> Option < & 'a Ignore > { match self . 0 . take () { None => None , Some (ig) => { self . 0 = ig . 0 . parent . as_ref () ; Some (ig) } } } }
};
}
