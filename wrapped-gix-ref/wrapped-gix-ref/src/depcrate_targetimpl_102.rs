// Generated macro for impl_102 (impl)
macro_rules! Depcrate_targetimpl_102 {
() => {
// Module: crate::target
// Provides: {"impl_102"}
// Dependencies: {}
impl TryFrom < Target > for ObjectId { type Error = Target ; fn try_from (value : Target) -> Result < Self , Self :: Error > { match value { Target :: Object (id) => Ok (id) , Target :: Symbolic (_) => Err (value) , } } }
};
}
