// Generated macro for impl_51 (impl)
macro_rules! Depcrate_automatonimpl_51 {
() => {
// Module: crate::automaton
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a , 'h , A : Automaton > Iterator for FindOverlappingIter < 'a , 'h , A > { type Item = Match ; # [inline (always)] fn next (& mut self) -> Option < Match > { self . aut . try_find_overlapping (& self . input , & mut self . state) . expect ("already checked that no match error can occur here") ; self . state . get_match () } }
};
}
