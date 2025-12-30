// Generated macro for impl_181 (impl)
macro_rules! Depcrate_ord_mapimpl_181 {
() => {
// Module: crate::ord::map
// Provides: {"impl_181"}
// Dependencies: {}
impl < K , V > Clone for OrdMap < K , V > { # [doc = " Clone a map."] # [doc = ""] # [doc = " Time: O(1)"] # [inline] fn clone (& self) -> Self { OrdMap { size : self . size , pool : self . pool . clone () , root : self . root . clone () , } } }
};
}
