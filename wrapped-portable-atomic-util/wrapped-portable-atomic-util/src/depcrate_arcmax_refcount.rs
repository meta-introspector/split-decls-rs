// Generated macro for MAX_REFCOUNT (const)
macro_rules! Depcrate_arcMAX_REFCOUNT {
() => {
// Module: crate::arc
// Provides: {"MAX_REFCOUNT"}
// Dependencies: {}
# [doc = " A soft limit on the amount of references that may be made to an `Arc`."] # [doc = ""] # [doc = " Going above this limit will abort your program (although not"] # [doc = " necessarily) at _exactly_ `MAX_REFCOUNT + 1` references."] # [doc = " Trying to go above it might call a `panic` (if not actually going above it)."] # [doc = ""] # [doc = " This is a global invariant, and also applies when using a compare-exchange loop."] # [doc = ""] # [doc = " See comment in `Arc::clone`."] const MAX_REFCOUNT : usize = isize :: MAX as usize ;
};
}
