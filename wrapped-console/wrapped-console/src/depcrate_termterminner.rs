// Generated macro for TermInner (struct)
macro_rules! Depcrate_termTermInner {
() => {
// Module: crate::term
// Provides: {"TermInner"}
// Dependencies: {}
# [derive (Debug)] struct TermInner { target : TermTarget , buffer : Option < Mutex < Vec < u8 > > > , prompt : RwLock < String > , prompt_guard : Mutex < () > , }
};
}
