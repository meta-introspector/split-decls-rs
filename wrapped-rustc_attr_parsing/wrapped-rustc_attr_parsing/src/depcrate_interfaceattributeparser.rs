// Generated macro for AttributeParser (struct)
macro_rules! Depcrate_interfaceAttributeParser {
() => {
// Module: crate::interface
// Provides: {"AttributeParser"}
// Dependencies: {}
# [doc = " Context created once, for example as part of the ast lowering"] # [doc = " context, through which all attributes can be lowered."] pub struct AttributeParser < 'sess , S : Stage = Late > { pub (crate) tools : Vec < Symbol > , pub (crate) features : Option < & 'sess Features > , pub (crate) sess : & 'sess Session , pub (crate) stage : S , # [doc = " *Only* parse attributes with this symbol."] # [doc = ""] # [doc = " Used in cases where we want the lowering infrastructure for parse just a single attribute."] parse_only : Option < Symbol > , }
};
}
