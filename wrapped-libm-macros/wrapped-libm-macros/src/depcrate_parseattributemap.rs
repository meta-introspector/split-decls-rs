// Generated macro for AttributeMap (struct)
macro_rules! Depcrate_parseAttributeMap {
() => {
// Module: crate::parse
// Provides: {"AttributeMap"}
// Dependencies: {}
# [doc = " A mapping of attributes to identifiers (just a simplified `Expr`)."] # [doc = ""] # [doc = " Expressed as:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[meta1]"] # [doc = " #[meta2]"] # [doc = " [foo, bar, baz]"] # [doc = " ```"] # [derive (Debug)] pub struct AttributeMap { pub meta : Vec < Meta > , pub names : Vec < Ident > , }
};
}
