// Generated macro for GLOBAL_MUTEX (static)
macro_rules! Depcrate_stdGLOBAL_MUTEX {
() => {
// Module: crate::std
// Provides: {"GLOBAL_MUTEX"}
// Dependencies: {}
# [cfg (not (loom))] static GLOBAL_MUTEX : Mutex < () > = Mutex :: new (()) ;
};
}
