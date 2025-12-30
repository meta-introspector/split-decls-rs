// Generated macro for MAX_REFCOUNT (const)
macro_rules! Depcrate_arcMAX_REFCOUNT {
() => {
// Module: crate::arc
// Provides: {"MAX_REFCOUNT"}
// Dependencies: {}
# [doc = " A soft limit on the amount of references that may be made to an `Arc`."] # [doc = ""] # [doc = " Going above this limit will abort your program (although not"] # [doc = " necessarily) at _exactly_ `MAX_REFCOUNT + 1` references."] const MAX_REFCOUNT : usize = (isize :: MAX) as usize ;
};
}
