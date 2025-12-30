// Generated macro for AdapterJsImportKind (enum)
macro_rules! Depcrate_wit_standardAdapterJsImportKind {
() => {
// Module: crate::wit::standard
// Provides: {"AdapterJsImportKind"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq)] pub enum AdapterJsImportKind { # [doc = " The first argument is an `externref` which is the `this` of the function"] # [doc = " call"] Method , # [doc = " The value imported should be invoked as `new`"] Constructor , # [doc = " A bland function import"] Normal , }
};
}
