// Generated macro for Join (trait)
macro_rules! Depcrate_joinJoin {
() => {
// Module: crate::join
// Provides: {"Join"}
// Dependencies: {}
# [doc = " The trait that abstracts over single-threaded and multi-threaded recursion."] # [doc = ""] # [doc = " See the [`join` module docs](index.html) for more details."] pub trait Join { fn join < A , B , RA , RB > (oper_a : A , oper_b : B) -> (RA , RB) where A : FnOnce () -> RA + Send , B : FnOnce () -> RB + Send , RA : Send , RB : Send ; }
};
}
