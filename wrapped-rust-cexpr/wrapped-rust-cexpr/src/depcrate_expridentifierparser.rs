// Generated macro for IdentifierParser (struct)
macro_rules! Depcrate_exprIdentifierParser {
() => {
// Module: crate::expr
// Provides: {"IdentifierParser"}
// Dependencies: {}
# [doc = " Expression parser/evaluator that supports identifiers."] # [derive (Debug)] pub struct IdentifierParser < 'ident > { identifiers : & 'ident HashMap < Vec < u8 > , EvalResult > , }
};
}
