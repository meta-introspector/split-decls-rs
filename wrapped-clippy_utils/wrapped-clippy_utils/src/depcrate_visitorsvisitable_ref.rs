// Generated macro for visitable_ref (macro)
macro_rules! Depcrate_visitorsvisitable_ref {
() => {
// Module: crate::visitors
// Provides: {"visitable_ref"}
// Dependencies: {}
macro_rules ! visitable_ref { ($ t : ident , $ f : ident) => { impl <'tcx > Visitable <'tcx > for &'tcx $ t <'tcx > { fn visit < V : Visitor <'tcx >> (self , visitor : & mut V) -> V :: Result { visitor .$ f (self) } } } ; }
};
}
