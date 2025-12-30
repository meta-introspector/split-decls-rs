// Generated macro for MatchInner (struct)
macro_rules! Depcrate_nonblockMatchInner {
() => {
// Module: crate::nonblock
// Provides: {"MatchInner"}
// Dependencies: {}
struct MatchInner { token : AtomicUsize , cb : Mutex < Option < Box < dyn FnMut (Message) -> bool + Send > > > , }
};
}
