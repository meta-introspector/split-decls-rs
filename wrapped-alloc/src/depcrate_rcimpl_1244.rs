// Generated macro for impl_1244 (impl)
macro_rules! Depcrate_rcimpl_1244 {
() => {
// Module: crate::rc
// Provides: {"impl_1244"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + PartialEq , A : Allocator > RcEqIdent < T , A > for Rc < T , A > { # [inline] default fn eq (& self , other : & Rc < T , A >) -> bool { * * self == * * other } # [inline] default fn ne (& self , other : & Rc < T , A >) -> bool { * * self != * * other } }
};
}
