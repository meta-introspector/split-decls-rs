// Generated macro for FnKind (enum)
macro_rules! Depcrate_intravisitFnKind {
() => {
// Module: crate::intravisit
// Provides: {"FnKind"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] pub enum FnKind < 'a > { # [doc = " `#[xxx] pub async/const/extern \"Abi\" fn foo()`"] ItemFn (Ident , & 'a Generics < 'a > , FnHeader) , # [doc = " `fn foo(&self)`"] Method (Ident , & 'a FnSig < 'a >) , # [doc = " `|x, y| {}`"] Closure , }
};
}
