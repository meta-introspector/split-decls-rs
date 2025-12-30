// Generated macro for JsImportName (enum)
macro_rules! Depcrate_wit_nonstandardJsImportName {
() => {
// Module: crate::wit::nonstandard
// Provides: {"JsImportName"}
// Dependencies: {}
# [doc = " Return value of `determine_import` which is where we look at an imported"] # [doc = " function AST and figure out where it's actually being imported from"] # [doc = " (performing some validation checks and whatnot)."] # [derive (Debug , Hash , Eq , PartialEq , Clone)] pub enum JsImportName { # [doc = " An item is imported from the global scope. The `name` is what's"] # [doc = " imported."] Global { name : String } , # [doc = " Same as `Global`, except the `name` is imported via an ESM import from"] # [doc = " the specified `module` path."] Module { module : String , name : String } , # [doc = " Same as `Module`, except we're importing from a local module defined in"] # [doc = " a local JS snippet."] LocalModule { module : String , name : String } , # [doc = " Same as `Module`, except we're importing from an `inline_js` attribute"] InlineJs { unique_crate_identifier : String , snippet_idx_in_crate : usize , name : String , } , # [doc = " A global import which may have a number of vendor prefixes associated"] # [doc = " with it, like `webkitAudioPrefix`. The `name` is the name to test"] # [doc = " whether it's prefixed."] VendorPrefixed { name : String , prefixes : Vec < String > } , }
};
}
