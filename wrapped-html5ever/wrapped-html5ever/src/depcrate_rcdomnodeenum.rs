// Generated macro for NodeEnum (enum)
macro_rules! Depcrate_rcdomNodeEnum {
() => {
// Module: crate::rcdom
// Provides: {"NodeEnum"}
// Dependencies: {}
# [doc = " The different kinds of nodes in the DOM."] # [derive (Debug)] pub enum NodeEnum { # [doc = " The `Document` itself."] Document , # [doc = " A `DOCTYPE` with name, public id, and system id."] Doctype (StrTendril , StrTendril , StrTendril) , # [doc = " A text node."] Text (StrTendril) , # [doc = " A comment."] Comment (StrTendril) , # [doc = " An element with attributes."] Element (QualName , ElementEnum , Vec < Attribute >) , }
};
}
