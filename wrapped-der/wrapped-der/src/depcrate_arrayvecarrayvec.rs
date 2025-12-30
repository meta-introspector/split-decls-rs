// Generated macro for ArrayVec (struct)
macro_rules! Depcrate_arrayvecArrayVec {
() => {
// Module: crate::arrayvec
// Provides: {"ArrayVec"}
// Dependencies: {}
# [doc = " Array-backed append-only vector type."] # [derive (Clone , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub (crate) struct ArrayVec < T , const N : usize > { # [doc = " Elements of the set."] elements : [Option < T > ; N] , # [doc = " Last populated element."] length : usize , }
};
}
