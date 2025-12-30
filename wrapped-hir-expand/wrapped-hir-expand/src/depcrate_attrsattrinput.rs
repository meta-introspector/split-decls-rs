// Generated macro for AttrInput (enum)
macro_rules! Depcrate_attrsAttrInput {
() => {
// Module: crate::attrs
// Provides: {"AttrInput"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum AttrInput { # [doc = " `#[attr = \"string\"]`"] Literal (tt :: Literal) , # [doc = " `#[attr(subtree)]`"] TokenTree (tt :: TopSubtree) , }
};
}
