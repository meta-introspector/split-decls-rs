// Generated macro for impl_330 (impl)
macro_rules! Depcrate_attrimpl_330 {
() => {
// Module: crate::attr
// Provides: {"impl_330"}
// Dependencies: {}
impl MetaItemLit { pub fn value_str (& self) -> Option < Symbol > { LitKind :: from_token_lit (self . as_token_lit ()) . ok () . and_then (| lit | lit . str ()) } }
};
}
