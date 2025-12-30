// Generated macro for OwnedStorage (type)
macro_rules! Depcrate_linear_mapOwnedStorage {
() => {
// Module: crate::linear_map
// Provides: {"OwnedStorage"}
// Dependencies: {}
# [doc = " Implementation of [`LinearMapStorage`] that stores the data in an array whose size is known at"] # [doc = " compile time."] pub type OwnedStorage < K , V , const N : usize > = OwnedVecStorage < (K , V) , N > ;
};
}
