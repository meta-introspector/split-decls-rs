// Generated macro for PodG1 (struct)
macro_rules! DepcratePodG1 {
() => {
// Module: crate
// Provides: {"PodG1"}
// Dependencies: {}
# [doc = " The BN254 (BN128) group element in G1 as a POD type."] # [doc = ""] # [doc = " A group element in G1 consists of two field elements `(x, y)`. A `PodG1`"] # [doc = " type expects a group element to be encoded as `[le(x), le(y)]` where"] # [doc = " `le(..)` is the little-endian encoding of the input field element as used"] # [doc = " in the `ark-bn254` crate. Note that this differs from the EIP-197 standard,"] # [doc = " which specifies that the field elements are encoded as big-endian."] # [doc = ""] # [doc = " `PodG1` can be constructed from both big-endian (EIP-197) and little-endian"] # [doc = " (ark-bn254) encodings using `from_be_bytes` and `from_le_bytes` methods,"] # [doc = " respectively."] # [cfg (not (target_os = "solana"))] # [derive (Clone , Copy , Debug , PartialEq , Eq , Pod , Zeroable)] # [repr (transparent)] pub struct PodG1 (pub [u8 ; G1_POINT_SIZE]) ;
};
}
