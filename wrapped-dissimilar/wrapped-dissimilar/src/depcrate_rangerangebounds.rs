// Generated macro for RangeBounds (trait)
macro_rules! Depcrate_rangeRangeBounds {
() => {
// Module: crate::range
// Provides: {"RangeBounds"}
// Dependencies: {}
pub trait RangeBounds : Sized + Clone + Debug { fn try_index (self , len : usize) -> Option < (usize , usize) > ; fn index (self , len : usize) -> (usize , usize) { match self . clone () . try_index (len) { Some (range) => range , None => panic ! ("index out of range, index={:?}, len={}" , self , len) , } } }
};
}
