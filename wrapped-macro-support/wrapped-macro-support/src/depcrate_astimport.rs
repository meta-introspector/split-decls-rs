// Generated macro for Import (struct)
macro_rules! Depcrate_astImport {
() => {
// Module: crate::ast
// Provides: {"Import"}
// Dependencies: {}
# [doc = " Things imported from a JS module (in an `extern` block)"] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub struct Import { # [doc = " The type of module being imported from, if any"] pub module : Option < ImportModule > , # [doc = " The namespace to access the item through, if any"] pub js_namespace : Option < Vec < String > > , # [doc = " If Some, this import should be re-exported with the optional given name"] pub reexport : Option < Option < String > > , # [doc = " The type of item being imported"] pub kind : ImportKind , }
};
}
