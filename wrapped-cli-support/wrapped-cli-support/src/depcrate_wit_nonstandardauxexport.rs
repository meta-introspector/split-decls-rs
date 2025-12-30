// Generated macro for AuxExport (struct)
macro_rules! Depcrate_wit_nonstandardAuxExport {
() => {
// Module: crate::wit::nonstandard
// Provides: {"AuxExport"}
// Dependencies: {}
# [derive (Debug)] pub struct AuxExport { # [doc = " When generating errors about this export, a helpful name to remember it"] # [doc = " by."] pub debug_name : String , # [doc = " Comments parsed in Rust and forwarded here to show up in JS bindings."] pub comments : String , # [doc = " Function's argument info in Rust forwarded here to configure the signature"] # [doc = " that shows up in bindings."] pub args : Option < Vec < AuxFunctionArgumentData > > , # [doc = " Whether this is an async function, to configure the TypeScript return value."] pub asyncness : bool , # [doc = " What kind of function this is and where it shows up"] pub kind : AuxExportKind , # [doc = " The namespace to export the item through, if any"] pub js_namespace : Option < Vec < String > > , # [doc = " Whether typescript bindings should be generated for this export."] pub generate_typescript : bool , # [doc = " Whether jsdoc comments should be generated for this export."] pub generate_jsdoc : bool , # [doc = " Whether typescript bindings should be generated for this export."] pub variadic : bool , # [doc = " Function's return overriding type"] pub fn_ret_ty_override : Option < String > , # [doc = " Function's return description"] pub fn_ret_desc : Option < String > , }
};
}
