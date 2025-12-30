// Generated macro for Attribute (struct)
macro_rules! Depcrate_tokenizer_interfaceAttribute {
() => {
// Module: crate::tokenizer::interface
// Provides: {"Attribute"}
// Dependencies: {}
# [doc = " A tag attribute."] # [doc = ""] # [doc = " The namespace on the attribute name is almost always ns!(\"\")."] # [doc = " The tokenizer creates all attributes this way, but the tree"] # [doc = " builder will adjust certain attribute names inside foreign"] # [doc = " content (MathML, SVG)."] # [derive (PartialEq , Eq , PartialOrd , Ord , Clone , Debug)] pub struct Attribute { pub name : QualName , pub value : StrTendril , }
};
}
