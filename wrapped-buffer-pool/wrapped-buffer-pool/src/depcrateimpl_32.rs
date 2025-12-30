// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < T : Default + Reuse > Pooled < T > { pub fn into_inner (mut self) -> T { std :: mem :: take (& mut self . inner) } }
};
}
