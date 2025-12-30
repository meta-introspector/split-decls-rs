// Generated macro for impl_51 (impl)
macro_rules! Depcrate_dfa_denseimpl_51 {
() => {
// Module: crate::dfa::dense
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a > Iterator for StartStateIter < 'a > { type Item = (StateID , Anchored , Start) ; fn next (& mut self) -> Option < (StateID , Anchored , Start) > { let i = self . i ; let table = self . st . table () ; if i >= table . len () { return None ; } self . i += 1 ; let start_type = Start :: from_usize (i % self . st . stride) . unwrap () ; let anchored = if i < self . st . stride { Anchored :: No } else if i < (2 * self . st . stride) { Anchored :: Yes } else { let pid = (i - (2 * self . st . stride)) / self . st . stride ; Anchored :: Pattern (PatternID :: new (pid) . unwrap ()) } ; Some ((table [i] , anchored , start_type)) } }
};
}
