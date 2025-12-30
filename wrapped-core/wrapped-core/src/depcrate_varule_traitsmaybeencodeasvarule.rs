// Generated macro for MaybeEncodeAsVarULE (trait)
macro_rules! Depcrate_varule_traitsMaybeEncodeAsVarULE {
() => {
// Module: crate::varule_traits
// Provides: {"MaybeEncodeAsVarULE"}
// Dependencies: {}
# [doc = " Export-only trait associated with [`MaybeAsVarULE`]. See that trait"] # [doc = " for additional details."] # [doc = ""] # [doc = " ✨ *Enabled with the `export` Cargo feature.*"] # [cfg (feature = "export")] pub trait MaybeEncodeAsVarULE : MaybeAsVarULE { # [doc = " Returns the [`MaybeAsVarULE::EncodedStruct`] that represents this data struct,"] # [doc = " or `None` if the data struct does not support this representation."] fn maybe_encode_as_varule (& self) -> Option < & Self :: EncodedStruct > ; }
};
}
