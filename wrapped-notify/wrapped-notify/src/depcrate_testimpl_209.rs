// Generated macro for impl_209 (impl)
macro_rules! Depcrate_testimpl_209 {
() => {
// Module: crate::test
// Provides: {"impl_209"}
// Dependencies: {}
impl Trackers { # [doc = " Pushes new tracker if the provided event has some and it is not equal to the last one"] pub fn try_push (& mut self , event : & Event) -> bool { let Some (tracker) = event . attrs . tracker () else { return false ; } ; if self . 0 . last () != Some (& tracker) { self . 0 . push (tracker) ; true } else { false } } }
};
}
