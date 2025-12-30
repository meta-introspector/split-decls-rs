// Generated macro for impl_210 (impl)
macro_rules! Depcrate_hirimpl_210 {
() => {
// Module: crate::hir
// Provides: {"impl_210"}
// Dependencies: {}
impl Path < '_ > { pub fn is_global (& self) -> bool { self . segments . first () . is_some_and (| segment | segment . ident . name == kw :: PathRoot) } }
};
}
