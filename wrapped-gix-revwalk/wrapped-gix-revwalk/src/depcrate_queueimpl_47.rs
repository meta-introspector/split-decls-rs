// Generated macro for impl_47 (impl)
macro_rules! Depcrate_queueimpl_47 {
() => {
// Module: crate::queue
// Provides: {"impl_47"}
// Dependencies: {}
impl < K , T > Clone for Item < K , T > where K : Clone , T : Clone , { fn clone (& self) -> Self { Item { key : self . key . clone () , value : self . value . clone () , } } }
};
}
