// Generated macro for impl_130 (impl)
macro_rules! Depcrate_boxedimpl_130 {
() => {
// Module: crate::boxed
// Provides: {"impl_130"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + PartialOrd , A : Allocator > PartialOrd for Box < T , A > { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { PartialOrd :: partial_cmp (& * * self , & * * other) } # [inline] fn lt (& self , other : & Self) -> bool { PartialOrd :: lt (& * * self , & * * other) } # [inline] fn le (& self , other : & Self) -> bool { PartialOrd :: le (& * * self , & * * other) } # [inline] fn ge (& self , other : & Self) -> bool { PartialOrd :: ge (& * * self , & * * other) } # [inline] fn gt (& self , other : & Self) -> bool { PartialOrd :: gt (& * * self , & * * other) } }
};
}
