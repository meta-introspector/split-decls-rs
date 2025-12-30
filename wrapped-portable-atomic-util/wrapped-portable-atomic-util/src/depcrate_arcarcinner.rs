// Generated macro for ArcInner (struct)
macro_rules! Depcrate_arcArcInner {
() => {
// Module: crate::arc
// Provides: {"ArcInner"}
// Dependencies: {}
# [repr (C)] struct ArcInner < T : ? Sized > { strong : atomic :: AtomicUsize , weak : atomic :: AtomicUsize , data : T , }
};
}
