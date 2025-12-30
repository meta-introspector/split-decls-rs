// Generated macro for impl_1102 (impl)
macro_rules! Depcrate_types_idimpl_1102 {
() => {
// Module: crate::types::id
// Provides: {"impl_1102"}
// Dependencies: {}
# [cfg (feature = "uuid")] impl TryFrom < ID > for uuid :: Uuid { type Error = uuid :: Error ; fn try_from (id : ID) -> Result < Self , Self :: Error > { uuid :: Uuid :: parse_str (& id . 0) } }
};
}
