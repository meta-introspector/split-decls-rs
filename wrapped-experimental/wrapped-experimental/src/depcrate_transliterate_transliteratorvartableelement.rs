// Generated macro for VarTableElement (enum)
macro_rules! Depcrate_transliterate_transliteratorVarTableElement {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"VarTableElement"}
// Dependencies: {}
enum VarTableElement < 'a > { Compound (& 'a str) , Quantifier (QuantifierKind , & 'a str) , Segment (Segment < 'a >) , UnicodeSet (CodePointInversionListAndStringList < 'a >) , FunctionCall (FunctionCall < 'a >) , BackReference (u16) , AnchorStart , AnchorEnd , LeftPlaceholderCursor (u16) , RightPlaceholderCursor (u16) , PureCursor , }
};
}
