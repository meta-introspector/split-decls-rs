// Generated macro for Header (trait)
macro_rules! Depcrate_traits_findHeader {
() => {
// Module: crate::traits::find
// Provides: {"Header"}
// Dependencies: {}
# [doc = " Find the header of an object in the object store."] pub trait Header { # [doc = " Find the header of the object matching `id` in the database."] # [doc = ""] # [doc = " Returns `Some` header if it was present, or the error that occurred during lookup."] fn try_header (& self , id : & gix_hash :: oid) -> Result < Option < crate :: Header > , find :: Error > ; }
};
}
