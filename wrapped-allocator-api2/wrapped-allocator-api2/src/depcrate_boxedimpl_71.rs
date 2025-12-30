// Generated macro for impl_71 (impl)
macro_rules! Depcrate_boxedimpl_71 {
() => {
// Module: crate::boxed
// Provides: {"impl_71"}
// Dependencies: {}
impl < T : ? Sized + Ord , A : Allocator > Ord for Box < T , A > { # [inline (always)] fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
};
}
