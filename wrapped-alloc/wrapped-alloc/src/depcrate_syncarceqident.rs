// Generated macro for ArcEqIdent (trait)
macro_rules! Depcrate_syncArcEqIdent {
() => {
// Module: crate::sync
// Provides: {"ArcEqIdent"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] trait ArcEqIdent < T : ? Sized + PartialEq , A : Allocator > { fn eq (& self , other : & Arc < T , A >) -> bool ; fn ne (& self , other : & Arc < T , A >) -> bool ; }
};
}
