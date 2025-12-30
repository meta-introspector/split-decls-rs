// Generated macro for impl_415 (impl)
macro_rules! Depcrate_hirimpl_415 {
() => {
// Module: crate::hir
// Provides: {"impl_415"}
// Dependencies: {}
impl FnHeader { pub fn is_async (& self) -> bool { matches ! (self . asyncness , IsAsync :: Async (_)) } pub fn is_const (& self) -> bool { matches ! (self . constness , Constness :: Const) } pub fn is_unsafe (& self) -> bool { self . safety () . is_unsafe () } pub fn is_safe (& self) -> bool { self . safety () . is_safe () } pub fn safety (& self) -> Safety { match self . safety { HeaderSafety :: SafeTargetFeatures => Safety :: Unsafe , HeaderSafety :: Normal (safety) => safety , } } }
};
}
