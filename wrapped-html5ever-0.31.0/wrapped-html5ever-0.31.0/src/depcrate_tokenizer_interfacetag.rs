// Generated macro for Tag (struct)
macro_rules! Depcrate_tokenizer_interfaceTag {
() => {
// Module: crate::tokenizer::interface
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " A tag token."] # [derive (PartialEq , Eq , Clone , Debug)] pub struct Tag { pub kind : TagKind , pub name : LocalName , pub self_closing : bool , pub attrs : Vec < Attribute > , }
};
}
