// Generated macro for SourceItemOrderingTraitAssocItemKinds (struct)
macro_rules! Depcrate_typesSourceItemOrderingTraitAssocItemKinds {
() => {
// Module: crate::types
// Provides: {"SourceItemOrderingTraitAssocItemKinds"}
// Dependencies: {}
# [doc = " Represents the order in which associated trait items should be ordered."] # [doc = ""] # [doc = " The reason to wrap a `Vec` in a newtype is to be able to implement"] # [doc = " [`Deserialize`]. Implementing `Deserialize` allows for implementing checks"] # [doc = " on configuration completeness at the time of loading the clippy config,"] # [doc = " letting the user know if there's any issues with the config (e.g. not"] # [doc = " listing all item kinds that should be sorted)."] # [derive (Clone)] pub struct SourceItemOrderingTraitAssocItemKinds (Vec < SourceItemOrderingTraitAssocItemKind >) ;
};
}
