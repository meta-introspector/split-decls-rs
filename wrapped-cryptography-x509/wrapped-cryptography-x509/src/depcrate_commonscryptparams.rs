// Generated macro for ScryptParams (struct)
macro_rules! Depcrate_commonScryptParams {
() => {
// Module: crate::common
// Provides: {"ScryptParams"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , PartialEq , Eq , Hash , Clone , Debug)] pub struct ScryptParams < 'a > { pub salt : & 'a [u8] , pub cost_parameter : u64 , pub block_size : u64 , pub parallelization_parameter : u64 , pub key_length : Option < u32 > , }
};
}
