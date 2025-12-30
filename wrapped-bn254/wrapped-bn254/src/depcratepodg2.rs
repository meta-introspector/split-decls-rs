// Generated macro for PodG2 (struct)
macro_rules! DepcratePodG2 {
() => {
// Module: crate
// Provides: {"PodG2"}
// Dependencies: {}
# [doc = " The BN254 (BN128) group element in G2 as a POD type."] # [doc = ""] # [doc = " Elements in G2 is represented by 2 field-extension elements `(x, y)`. Each"] # [doc = " field-extension element itself is a degree 1 polynomial `x = x0 + x1*X`,"] # [doc = " `y = y0 + y1*X`. The EIP-197 standard encodes a G2 element as"] # [doc = " `[be(x1), be(x0), be(y1), be(y0)]` where `be(..)` is the big-endian"] # [doc = " encoding of the input field element. The `ark-bn254` crate encodes a G2"] # [doc = " element as `[le(x0), le(x1), le(y0), le(y1)]` where `le(..)` is the"] # [doc = " little-endian encoding of the input field element. Notably, in addition to"] # [doc = " the differences in the big-endian vs. little-endian encodings of field"] # [doc = " elements, the order of the polynomial field coefficients `x0`, `x1`, `y0`,"] # [doc = " and `y1` are different."] # [doc = ""] # [doc = " `PodG2` can be constructed from both big-endian (EIP-197) and little-endian"] # [doc = " (ark-bn254) encodings using `from_be_bytes` and `from_le_bytes` methods,"] # [doc = " respectively."] # [cfg (not (target_os = "solana"))] # [derive (Clone , Copy , Debug , PartialEq , Eq , Pod , Zeroable)] # [repr (transparent)] pub struct PodG2 (pub [u8 ; G2_POINT_SIZE]) ;
};
}
