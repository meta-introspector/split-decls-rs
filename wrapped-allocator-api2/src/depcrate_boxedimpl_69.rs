// Generated macro for impl_69 (impl)
macro_rules! Depcrate_boxedimpl_69 {
() => {
// Module: crate::boxed
// Provides: {"impl_69"}
// Dependencies: {}
impl < T : ? Sized + PartialEq , A : Allocator > PartialEq for Box < T , A > { # [inline (always)] fn eq (& self , other : & Self) -> bool { PartialEq :: eq (& * * self , & * * other) } # [inline (always)] fn ne (& self , other : & Self) -> bool { PartialEq :: ne (& * * self , & * * other) } }
};
}
