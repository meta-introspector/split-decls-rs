// Generated macro for JsImport (struct)
macro_rules! Depcrate_wit_nonstandardJsImport {
() => {
// Module: crate::wit::nonstandard
// Provides: {"JsImport"}
// Dependencies: {}
# [doc = " What can actually be imported and typically a value in each of the variants"] # [doc = " above of `AuxImport`"] # [doc = ""] # [doc = " A `JsImport` is intended to indicate what exactly is being imported for a"] # [doc = " particular operation."] # [derive (Debug , Hash , Eq , PartialEq , Clone)] pub struct JsImport { # [doc = " The base of whatever is being imported, either from a module, the global"] # [doc = " namespace, or similar."] pub name : JsImportName , # [doc = " Various field accesses (like `.foo.bar.baz`) to hang off the `name`"] # [doc = " above."] pub fields : Vec < String > , }
};
}
