// Generated macro for Header (trait)
macro_rules! Depcrate_traitsHeader {
() => {
// Module: crate::traits
// Provides: {"Header"}
// Dependencies: {}
# [doc = " A way to obtain object properties without fully decoding it."] pub trait Header { # [doc = " Try to read the header of the object associated with `id` or return `None` if it could not be found."] fn try_header (& self , id : & gix_hash :: oid) -> Result < Option < find :: Header > , gix_object :: find :: Error > ; }
};
}
