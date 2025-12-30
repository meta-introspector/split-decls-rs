// Generated macro for AuxStruct (struct)
macro_rules! Depcrate_wit_nonstandardAuxStruct {
() => {
// Module: crate::wit::nonstandard
// Provides: {"AuxStruct"}
// Dependencies: {}
# [derive (Debug)] pub struct AuxStruct { # [doc = " The name of this struct"] pub name : String , # [doc = " The copied Rust comments to forward to JS"] pub comments : String , # [doc = " Whether to generate helper methods for inspecting the class"] pub is_inspectable : bool , # [doc = " Whether typescript bindings should be generated for this struct."] pub generate_typescript : bool , # [doc = " The namespace to export the struct through, if any"] pub js_namespace : Option < Vec < String > > , }
};
}
