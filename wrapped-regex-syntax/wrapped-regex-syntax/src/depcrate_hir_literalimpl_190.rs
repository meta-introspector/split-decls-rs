// Generated macro for impl_190 (impl)
macro_rules! Depcrate_hir_literalimpl_190 {
() => {
// Module: crate::hir::literal
// Provides: {"impl_190"}
// Dependencies: {}
impl core :: fmt :: Debug for Seq { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Seq") ? ; if let Some (lits) = self . literals () { f . debug_list () . entries (lits . iter ()) . finish () } else { write ! (f , "[∞]") } } }
};
}
