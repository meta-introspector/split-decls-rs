// Generated macro for HashValue (trait)
macro_rules! Depcrate_nodes_hamtHashValue {
() => {
// Module: crate::nodes::hamt
// Provides: {"HashValue"}
// Dependencies: {}
pub trait HashValue { type Key : Eq ; fn extract_key (& self) -> & Self :: Key ; fn ptr_eq (& self , other : & Self) -> bool ; }
};
}
