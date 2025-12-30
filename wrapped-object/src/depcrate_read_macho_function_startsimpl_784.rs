// Generated macro for impl_784 (impl)
macro_rules! Depcrate_read_macho_function_startsimpl_784 {
() => {
// Module: crate::read::macho::function_starts
// Provides: {"impl_784"}
// Dependencies: {}
impl < 'data > Iterator for FunctionStartsIterator < 'data > { type Item = Result < u64 > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
