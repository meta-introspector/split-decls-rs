// Generated macro for Impl (struct)
macro_rules! Depcrate_astImpl {
() => {
// Module: crate::ast
// Provides: {"Impl"}
// Dependencies: {}
# [doc = " Top level AST and everything required to implement `Arbitrary` for any"] # [doc = " given type. Linearizing this AST gives you the impl wrt. Rust code."] pub struct Impl { # [doc = " Name of the type."] typ : syn :: Ident , # [doc = " Tracker for uses of Arbitrary trait for a generic type."] tracker : UseTracker , # [doc = " The three main parts, see description of `ImplParts` for details."] parts : ImplParts , }
};
}
