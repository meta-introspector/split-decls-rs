// Generated macro for Hash (struct)
macro_rules! DepcrateHash {
() => {
// Module: crate
// Provides: {"Hash"}
// Dependencies: {}
# [doc = " A hash; the 32-byte output of a hashing algorithm."] # [doc = ""] # [doc = " This struct is used most often in `solana-sdk` and related crates to contain"] # [doc = " a [SHA-256] hash, but may instead contain a [blake3] hash."] # [doc = ""] # [doc = " [SHA-256]: https://en.wikipedia.org/wiki/SHA-2"] # [doc = " [blake3]: https://github.com/BLAKE3-team/BLAKE3"] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "borsh" , derive (BorshSerialize , BorshDeserialize) , borsh (crate = "borsh"))] # [cfg_attr (feature = "borsh" , derive (BorshSchema))] # [cfg_attr (feature = "bytemuck" , derive (Pod , Zeroable))] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize ,))] # [cfg_attr (feature = "wincode" , derive (SchemaWrite , SchemaRead))] # [cfg_attr (feature = "copy" , derive (Copy))] # [cfg_attr (not (feature = "decode") , derive (Debug))] # [derive (Clone , Default , Eq , PartialEq , Ord , PartialOrd , Hash)] # [repr (transparent)] pub struct Hash (pub (crate) [u8 ; HASH_BYTES]) ;
};
}
