// Generated macro for AlgorithmIdentifier (trait)
macro_rules! Depcrate_kemAlgorithmIdentifier {
() => {
// Module: crate::kem
// Provides: {"AlgorithmIdentifier"}
// Dependencies: {}
# [doc = " An identifier for a KEM algorithm."] pub trait AlgorithmIdentifier : Copy + Clone + Debug + PartialEq + crate :: sealed :: Sealed + 'static { # [doc = " Returns the algorithm's associated AWS-LC nid."] fn nid (self) -> i32 ; }
};
}
