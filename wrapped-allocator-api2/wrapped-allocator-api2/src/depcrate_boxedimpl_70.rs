// Generated macro for impl_70 (impl)
macro_rules! Depcrate_boxedimpl_70 {
() => {
// Module: crate::boxed
// Provides: {"impl_70"}
// Dependencies: {}
impl < T : ? Sized + PartialOrd , A : Allocator > PartialOrd for Box < T , A > { # [inline (always)] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { PartialOrd :: partial_cmp (& * * self , & * * other) } # [inline (always)] fn lt (& self , other : & Self) -> bool { PartialOrd :: lt (& * * self , & * * other) } # [inline (always)] fn le (& self , other : & Self) -> bool { PartialOrd :: le (& * * self , & * * other) } # [inline (always)] fn ge (& self , other : & Self) -> bool { PartialOrd :: ge (& * * self , & * * other) } # [inline (always)] fn gt (& self , other : & Self) -> bool { PartialOrd :: gt (& * * self , & * * other) } }
};
}
