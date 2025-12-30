// Generated macro for impl_21 (impl)
macro_rules! Depcrate_blob_builtin_driver_textimpl_21 {
() => {
// Module: crate::blob::builtin_driver::text
// Provides: {"impl_21"}
// Dependencies: {}
impl Conflict { # [doc = " The amount of conflict marker characters to print by default."] pub const DEFAULT_MARKER_SIZE : u8 = 7 ; # [doc = " The amount of conflict markers to print if this instance contains them, or `None` otherwise"] pub fn marker_size (& self) -> Option < u8 > { match self { Conflict :: Keep { marker_size , .. } => Some (marker_size . get ()) , Conflict :: ResolveWithOurs | Conflict :: ResolveWithTheirs | Conflict :: ResolveWithUnion => None , } } }
};
}
