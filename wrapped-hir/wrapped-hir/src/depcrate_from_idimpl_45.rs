// Generated macro for impl_45 (impl)
macro_rules! Depcrate_from_idimpl_45 {
() => {
// Module: crate::from_id
// Provides: {"impl_45"}
// Dependencies: {}
impl From < FieldId > for Field { fn from (def : FieldId) -> Self { Field { parent : def . parent . into () , id : def . local_id } } }
};
}
