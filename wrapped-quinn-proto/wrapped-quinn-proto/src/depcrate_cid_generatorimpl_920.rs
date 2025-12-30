// Generated macro for impl_920 (impl)
macro_rules! Depcrate_cid_generatorimpl_920 {
() => {
// Module: crate::cid_generator
// Provides: {"impl_920"}
// Dependencies: {}
impl ConnectionIdGenerator for HashedConnectionIdGenerator { fn generate_cid (& mut self) -> ConnectionId { let mut bytes_arr = [0 ; NONCE_LEN + SIGNATURE_LEN] ; rand :: rng () . fill_bytes (& mut bytes_arr [.. NONCE_LEN]) ; let mut hasher = rustc_hash :: FxHasher :: default () ; hasher . write_u64 (self . key) ; hasher . write (& bytes_arr [.. NONCE_LEN]) ; bytes_arr [NONCE_LEN ..] . copy_from_slice (& hasher . finish () . to_le_bytes () [.. SIGNATURE_LEN]) ; ConnectionId :: new (& bytes_arr) } fn validate (& self , cid : ConnectionId) -> Result < () , InvalidCid > { let (nonce , signature) = cid . split_at (NONCE_LEN) ; let mut hasher = rustc_hash :: FxHasher :: default () ; hasher . write_u64 (self . key) ; hasher . write (nonce) ; let expected = hasher . finish () . to_le_bytes () ; match expected [.. SIGNATURE_LEN] == signature [..] { true => Ok (()) , false => Err (InvalidCid) , } } fn cid_len (& self) -> usize { NONCE_LEN + SIGNATURE_LEN } fn cid_lifetime (& self) -> Option < Duration > { self . lifetime } }
};
}
