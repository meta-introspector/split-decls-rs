// Generated macro for impl_47 (impl)
macro_rules! Depcrate_attrimpl_47 {
() => {
// Module: crate::attr
// Provides: {"impl_47"}
// Dependencies: {}
impl DocExpr { fn parse < S : Copy > (tt : & tt :: TopSubtree < S >) -> DocExpr { next_doc_expr (tt . iter ()) . unwrap_or (DocExpr :: Invalid) } pub fn aliases (& self) -> & [Symbol] { match self { DocExpr :: Atom (DocAtom :: KeyValue { key , value }) if * key == sym :: alias => { std :: slice :: from_ref (value) } DocExpr :: Alias (aliases) => aliases , _ => & [] , } } }
};
}
