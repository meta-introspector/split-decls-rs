// Generated macro for PrimTy (enum)
macro_rules! Depcrate_hirPrimTy {
() => {
// Module: crate::hir
// Provides: {"PrimTy"}
// Dependencies: {}
# [doc = " Not represented directly in the AST; referred to by name through a `ty_path`."] # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Hash , Debug , HashStable_Generic)] pub enum PrimTy { Int (IntTy) , Uint (UintTy) , Float (FloatTy) , Str , Bool , Char , }
};
}
