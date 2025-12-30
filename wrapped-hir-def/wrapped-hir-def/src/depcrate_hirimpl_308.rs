// Generated macro for impl_308 (impl)
macro_rules! Depcrate_hirimpl_308 {
() => {
// Module: crate::hir
// Provides: {"impl_308"}
// Dependencies: {}
impl Literal { pub fn negate (self) -> Option < Self > { if let Literal :: Int (i , k) = self { Some (Literal :: Int (- i , k)) } else { None } } }
};
}
