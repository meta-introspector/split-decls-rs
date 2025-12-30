// Generated macro for impl_314 (impl)
macro_rules! Depcrate_hirimpl_314 {
() => {
// Module: crate::hir
// Provides: {"impl_314"}
// Dependencies: {}
impl Literal { pub fn negate (self) -> Option < Self > { if let Literal :: Int (i , k) = self { Some (Literal :: Int (- i , k)) } else { None } } }
};
}
