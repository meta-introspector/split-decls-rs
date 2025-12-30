// Generated macro for Literal (struct)
macro_rules! Depcrate_hir_literalLiteral {
() => {
// Module: crate::hir::literal
// Provides: {"Literal"}
// Dependencies: {}
# [doc = " A single literal extracted from an [`Hir`] expression."] # [doc = ""] # [doc = " A literal is composed of two things:"] # [doc = ""] # [doc = " * A sequence of bytes. No guarantees with respect to UTF-8 are provided."] # [doc = " In particular, even if the regex a literal is extracted from is UTF-8, the"] # [doc = " literal extracted may not be valid UTF-8. (For example, if an [`Extractor`]"] # [doc = " limit resulted in trimming a literal in a way that splits a codepoint.)"] # [doc = " * Whether the literal is \"exact\" or not. An \"exact\" literal means that it"] # [doc = " has not been trimmed, and may continue to be extended. If a literal is"] # [doc = " \"exact\" after visiting the entire `Hir` expression, then this implies that"] # [doc = " the literal leads to a match state. (Although it doesn't necessarily imply"] # [doc = " all occurrences of the literal correspond to a match of the regex, since"] # [doc = " literal extraction ignores look-around assertions.)"] # [derive (Clone , Eq , PartialEq , PartialOrd , Ord)] pub struct Literal { bytes : Vec < u8 > , exact : bool , }
};
}
