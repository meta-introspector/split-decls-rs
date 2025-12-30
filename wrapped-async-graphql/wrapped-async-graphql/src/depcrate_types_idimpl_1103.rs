// Generated macro for impl_1103 (impl)
macro_rules! Depcrate_types_idimpl_1103 {
() => {
// Module: crate::types::id
// Provides: {"impl_1103"}
// Dependencies: {}
# [cfg (feature = "bson")] impl TryFrom < ID > for ObjectId { type Error = oid :: Error ; fn try_from (id : ID) -> std :: result :: Result < Self , oid :: Error > { ObjectId :: parse_str (id . 0) } }
};
}
