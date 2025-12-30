// Generated macro for AncestorsIter (type)
macro_rules! Depcrate_ext_object_idAncestorsIter {
() => {
// Module: crate::ext::object_id
// Provides: {"AncestorsIter"}
// Dependencies: {}
pub type AncestorsIter < Find > = gix_traverse :: commit :: Simple < Find , fn (& gix_hash :: oid) -> bool > ;
};
}
