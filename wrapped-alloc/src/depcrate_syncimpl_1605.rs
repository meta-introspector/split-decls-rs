// Generated macro for impl_1605 (impl)
macro_rules! Depcrate_syncimpl_1605 {
() => {
// Module: crate::sync
// Provides: {"impl_1605"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + PartialEq , A : Allocator > ArcEqIdent < T , A > for Arc < T , A > { # [inline] default fn eq (& self , other : & Arc < T , A >) -> bool { * * self == * * other } # [inline] default fn ne (& self , other : & Arc < T , A >) -> bool { * * self != * * other } }
};
}
