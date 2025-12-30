// Generated macro for impl_916 (impl)
macro_rules! Depcrate_cid_generatorimpl_916 {
() => {
// Module: crate::cid_generator
// Provides: {"impl_916"}
// Dependencies: {}
impl ConnectionIdGenerator for RandomConnectionIdGenerator { fn generate_cid (& mut self) -> ConnectionId { let mut bytes_arr = [0 ; MAX_CID_SIZE] ; rand :: rng () . fill_bytes (& mut bytes_arr [.. self . cid_len]) ; ConnectionId :: new (& bytes_arr [.. self . cid_len]) } # [doc = " Provide the length of dst_cid in short header packet"] fn cid_len (& self) -> usize { self . cid_len } fn cid_lifetime (& self) -> Option < Duration > { self . lifetime } }
};
}
