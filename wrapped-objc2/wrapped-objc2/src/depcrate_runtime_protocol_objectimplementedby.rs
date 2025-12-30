// Generated macro for ImplementedBy (trait)
macro_rules! Depcrate_runtime_protocol_objectImplementedBy {
() => {
// Module: crate::runtime::protocol_object
// Provides: {"ImplementedBy"}
// Dependencies: {}
# [doc = " An internal helper trait for [`ProtocolObject`]."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This is meant to be a sealed trait, and should not be implemented outside"] # [doc = " of the [`extern_protocol!`] macro."] # [doc = ""] # [doc = " [`extern_protocol!`]: crate::extern_protocol"] pub unsafe trait ImplementedBy < T : ? Sized + Message > { # [doc (hidden)] const __INNER : () ; }
};
}
