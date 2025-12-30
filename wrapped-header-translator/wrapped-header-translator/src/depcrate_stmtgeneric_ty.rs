// Generated macro for generic_ty (function)
macro_rules! Depcrate_stmtgeneric_ty {
() => {
// Module: crate::stmt
// Provides: {"generic_ty"}
// Dependencies: {}
fn generic_ty < 'a , I : IntoIterator < Item = & 'a GenericWithBound > > (iter : I ,) -> GenericTyHelper < impl IntoIterator < Item = & 'a String > + Clone > where I :: IntoIter : Clone , { GenericTyHelper (iter . into_iter () . map (| (generic , _bound) | generic)) }
};
}
