// Generated macro for Element (enum)
macro_rules! Depcrate_transliterate_compile_parseElement {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"Element"}
// Dependencies: {}
# [derive (Debug , Clone)] pub (crate) enum Element { Literal (String) , VariableRef (String) , BackRef (u32) , Quantifier (QuantifierKind , Box < Element >) , Segment (Section) , UnicodeSet (UnicodeSet) , FunctionCall (SingleId , Section) , Cursor (u32 , u32) , AnchorStart , AnchorEnd , }
};
}
