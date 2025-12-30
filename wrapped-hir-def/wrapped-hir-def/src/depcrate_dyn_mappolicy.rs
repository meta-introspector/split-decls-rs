// Generated macro for Policy (trait)
macro_rules! Depcrate_dyn_mapPolicy {
() => {
// Module: crate::dyn_map
// Provides: {"Policy"}
// Dependencies: {}
pub trait Policy { type K ; type V ; fn insert (map : & mut DynMap , key : Self :: K , value : Self :: V) ; fn get < 'a > (map : & 'a DynMap , key : & Self :: K) -> Option < & 'a Self :: V > ; fn is_empty (map : & DynMap) -> bool ; }
};
}
