// Generated macro for _impls (module)
macro_rules! Depcrate_object_tree_impls {
() => {
// Module: crate::object::tree
// Provides: {"_impls"}
// Dependencies: {}
mod _impls { use crate :: Tree ; impl TryFrom < Tree < '_ > > for gix_object :: Tree { type Error = gix_object :: decode :: Error ; fn try_from (t : Tree < '_ >) -> Result < Self , Self :: Error > { t . decode () . map (Into :: into) } } }
};
}
