// Generated macro for GLOB_STACK (static)
macro_rules! Depcrate_globGLOB_STACK {
() => {
// Module: crate::glob
// Provides: {"GLOB_STACK"}
// Dependencies: {}
# [doc = " the glob stack holds failure count and an indication if `cargo insta review`"] # [doc = " should be run."] pub (crate) static GLOB_STACK : Lazy < Mutex < Vec < GlobCollector > > > = Lazy :: new (Mutex :: default) ;
};
}
