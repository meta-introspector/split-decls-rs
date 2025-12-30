// Generated macro for AHasher (struct)
macro_rules! Depcrate_fallback_hashAHasher {
() => {
// Module: crate::fallback_hash
// Provides: {"AHasher"}
// Dependencies: {}
# [doc = " A `Hasher` for hashing an arbitrary stream of bytes."] # [doc = ""] # [doc = " Instances of [`AHasher`] represent state that is updated while hashing data."] # [doc = ""] # [doc = " Each method updates the internal state based on the new data provided. Once"] # [doc = " all of the data has been provided, the resulting hash can be obtained by calling"] # [doc = " `finish()`"] # [doc = ""] # [doc = " [Clone] is also provided in case you wish to calculate hashes for two different items that"] # [doc = " start with the same data."] # [doc = ""] # [derive (Debug , Clone)] pub struct AHasher { buffer : u64 , pad : u64 , extra_keys : [u64 ; 2] , }
};
}
