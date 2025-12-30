// Generated macro for impl_395 (impl)
macro_rules! Depcrate_stmtimpl_395 {
() => {
// Module: crate::stmt
// Provides: {"impl_395"}
// Dependencies: {}
impl < I : IntoIterator + Clone > fmt :: Display for GenericTyHelper < I > where I :: Item : Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut iter = self . 0 . clone () . into_iter () ; if let Some (first) = iter . next () { write ! (f , "<{first}") ? ; for generic in iter { write ! (f , ", {generic}") ? ; } write ! (f , ">") ? ; } Ok (()) } }
};
}
