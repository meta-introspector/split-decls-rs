// Generated macro for OwnedStorage (type)
macro_rules! Depcrate_stringOwnedStorage {
() => {
// Module: crate::string
// Provides: {"OwnedStorage"}
// Dependencies: {}
# [doc = " Implementation of [`StringStorage`] that stores the data in an array whose size is known at"] # [doc = " compile time."] pub type OwnedStorage < const N : usize > = OwnedVecStorage < u8 , N > ;
};
}
