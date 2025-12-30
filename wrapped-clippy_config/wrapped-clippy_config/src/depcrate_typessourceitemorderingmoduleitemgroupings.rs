// Generated macro for SourceItemOrderingModuleItemGroupings (struct)
macro_rules! Depcrate_typesSourceItemOrderingModuleItemGroupings {
() => {
// Module: crate::types
// Provides: {"SourceItemOrderingModuleItemGroupings"}
// Dependencies: {}
# [doc = " Represents the configured ordering of items within a module."] # [doc = ""] # [doc = " The [`Deserialize`] implementation checks that no item kinds have been"] # [doc = " omitted and that there are no duplicates in the user configuration."] # [derive (Clone)] pub struct SourceItemOrderingModuleItemGroupings { groups : Vec < (String , Vec < SourceItemOrderingModuleItemKind >) > , lut : HashMap < SourceItemOrderingModuleItemKind , usize > , back_lut : HashMap < SourceItemOrderingModuleItemKind , String > , }
};
}
