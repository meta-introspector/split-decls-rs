// Generated macro for Separator (enum)
macro_rules! Depcrate_parserSeparator {
() => {
// Module: crate::parser
// Provides: {"Separator"}
// Dependencies: {}
# [derive (Clone , Debug , Eq)] pub (crate) enum Separator { Literal (tt :: Literal < Span >) , Ident (tt :: Ident < Span >) , Puncts (ArrayVec < tt :: Punct < Span > , MAX_GLUED_PUNCT_LEN >) , Lifetime (tt :: Punct < Span > , tt :: Ident < Span >) , }
};
}
