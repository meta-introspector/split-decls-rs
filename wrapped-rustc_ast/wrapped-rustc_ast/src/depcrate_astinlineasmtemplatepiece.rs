// Generated macro for InlineAsmTemplatePiece (enum)
macro_rules! Depcrate_astInlineAsmTemplatePiece {
() => {
// Module: crate::ast
// Provides: {"InlineAsmTemplatePiece"}
// Dependencies: {}
# [derive (Clone , PartialEq , Encodable , Decodable , Debug , Hash , HashStable_Generic , Walkable)] pub enum InlineAsmTemplatePiece { String (Cow < 'static , str >) , Placeholder { operand_idx : usize , modifier : Option < char > , span : Span } , }
};
}
