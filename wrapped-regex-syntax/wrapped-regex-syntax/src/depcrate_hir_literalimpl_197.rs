// Generated macro for impl_197 (impl)
macro_rules! Depcrate_hir_literalimpl_197 {
() => {
// Module: crate::hir::literal
// Provides: {"impl_197"}
// Dependencies: {}
impl core :: fmt :: Debug for Literal { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let tag = if self . exact { "E" } else { "I" } ; f . debug_tuple (tag) . field (& crate :: debug :: Bytes (self . as_bytes ())) . finish () } }
};
}
