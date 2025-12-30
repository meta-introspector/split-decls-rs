// Generated macro for impl_1175 (impl)
macro_rules! Depcrate_statusimpl_1175 {
() => {
// Module: crate::status
// Provides: {"impl_1175"}
// Dependencies: {}
impl < 'a > Iterator for StatusIter < 'a > { type Item = StatusEntry < 'a > ; fn next (& mut self) -> Option < StatusEntry < 'a > > { self . range . next () . and_then (| i | self . statuses . get (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
