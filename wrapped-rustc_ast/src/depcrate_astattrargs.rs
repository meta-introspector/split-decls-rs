// Generated macro for AttrArgs (enum)
macro_rules! Depcrate_astAttrArgs {
() => {
// Module: crate::ast
// Provides: {"AttrArgs"}
// Dependencies: {}
# [doc = " Arguments passed to an attribute macro."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum AttrArgs { # [doc = " No arguments: `#[attr]`."] Empty , # [doc = " Delimited arguments: `#[attr()/[]/{}]`."] Delimited (DelimArgs) , # [doc = " Arguments of a key-value attribute: `#[attr = \"value\"]`."] Eq { # [doc = " Span of the `=` token."] eq_span : Span , expr : Box < Expr > , } , }
};
}
