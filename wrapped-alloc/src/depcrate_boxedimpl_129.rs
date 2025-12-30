// Generated macro for impl_129 (impl)
macro_rules! Depcrate_boxedimpl_129 {
() => {
// Module: crate::boxed
// Provides: {"impl_129"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + PartialEq , A : Allocator > PartialEq for Box < T , A > { # [inline] fn eq (& self , other : & Self) -> bool { PartialEq :: eq (& * * self , & * * other) } # [inline] fn ne (& self , other : & Self) -> bool { PartialEq :: ne (& * * self , & * * other) } }
};
}
