// Generated macro for impl_61 (impl)
macro_rules! Depcrate_deimpl_61 {
() => {
// Module: crate::de
// Provides: {"impl_61"}
// Dependencies: {}
# [cfg (feature = "bson")] impl BorshDeserialize for bson :: oid :: ObjectId { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let mut buf = [0u8 ; 12] ; reader . read_exact (& mut buf) ? ; Ok (bson :: oid :: ObjectId :: from_bytes (buf)) } }
};
}
