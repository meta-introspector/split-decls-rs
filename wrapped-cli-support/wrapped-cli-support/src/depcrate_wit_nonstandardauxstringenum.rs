// Generated macro for AuxStringEnum (struct)
macro_rules! Depcrate_wit_nonstandardAuxStringEnum {
() => {
// Module: crate::wit::nonstandard
// Provides: {"AuxStringEnum"}
// Dependencies: {}
# [derive (Debug)] pub struct AuxStringEnum { # [doc = " The name of this enum"] pub name : String , # [doc = " The copied Rust comments to forward to JS"] pub comments : String , # [doc = " A list of variants values"] pub variant_values : Vec < String > , # [doc = " Whether typescript bindings should be generated for this enum."] pub generate_typescript : bool , # [doc = " The namespace to export the enum through, if any"] # [doc = " Note: Currently unused as string enums don't generate exports,"] # [doc = " but kept for consistency and potential future use."] # [allow (dead_code)] pub js_namespace : Option < Vec < String > > , }
};
}
