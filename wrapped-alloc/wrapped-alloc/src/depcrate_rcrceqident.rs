// Generated macro for RcEqIdent (trait)
macro_rules! Depcrate_rcRcEqIdent {
() => {
// Module: crate::rc
// Provides: {"RcEqIdent"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] trait RcEqIdent < T : ? Sized + PartialEq , A : Allocator > { fn eq (& self , other : & Rc < T , A >) -> bool ; fn ne (& self , other : & Rc < T , A >) -> bool ; }
};
}
