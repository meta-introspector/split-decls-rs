// Generated macro for reference_hash (function)
macro_rules! Depcrate_testreference_hash {
() => {
// Module: crate::test
// Provides: {"reference_hash"}
// Dependencies: {}
fn reference_hash (input : & [u8]) -> crate :: Hash { let mut hasher = reference_impl :: Hasher :: new () ; hasher . update (input) ; let mut bytes = [0 ; 32] ; hasher . finalize (& mut bytes) ; bytes . into () }
};
}
