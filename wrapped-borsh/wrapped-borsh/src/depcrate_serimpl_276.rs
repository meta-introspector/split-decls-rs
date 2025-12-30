// Generated macro for impl_276 (impl)
macro_rules! Depcrate_serimpl_276 {
() => {
// Module: crate::ser
// Provides: {"impl_276"}
// Dependencies: {}
# [cfg (feature = "bson")] impl BorshSerialize for bson :: oid :: ObjectId { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . bytes () . serialize (writer) } }
};
}
