// Generated macro for ArcInner (struct)
macro_rules! Depcrate_syncArcInner {
() => {
// Module: crate::sync
// Provides: {"ArcInner"}
// Dependencies: {}
# [repr (C)] struct ArcInner < T : ? Sized > { strong : Atomic < usize > , weak : Atomic < usize > , data : T , }
};
}
