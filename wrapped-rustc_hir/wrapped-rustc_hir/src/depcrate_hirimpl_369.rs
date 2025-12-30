// Generated macro for impl_369 (impl)
macro_rules! Depcrate_hirimpl_369 {
() => {
// Module: crate::hir
// Provides: {"impl_369"}
// Dependencies: {}
impl PreciseCapturingArg < '_ > { pub fn hir_id (self) -> HirId { match self { PreciseCapturingArg :: Lifetime (lt) => lt . hir_id , PreciseCapturingArg :: Param (param) => param . hir_id , } } pub fn name (self) -> Symbol { match self { PreciseCapturingArg :: Lifetime (lt) => lt . ident . name , PreciseCapturingArg :: Param (param) => param . ident . name , } } }
};
}
