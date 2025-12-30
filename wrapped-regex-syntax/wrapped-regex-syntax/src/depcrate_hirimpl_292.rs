// Generated macro for impl_292 (impl)
macro_rules! Depcrate_hirimpl_292 {
() => {
// Module: crate::hir
// Provides: {"impl_292"}
// Dependencies: {}
impl core :: fmt :: Debug for LookSet { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { if self . is_empty () { return write ! (f , "∅") ; } for look in self . iter () { write ! (f , "{}" , look . as_char ()) ? ; } Ok (()) } }
};
}
