// Generated macro for ConcatMetaVarExprElem (enum)
macro_rules! Depcrate_parserConcatMetaVarExprElem {
() => {
// Module: crate::parser
// Provides: {"ConcatMetaVarExprElem"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum ConcatMetaVarExprElem { # [doc = " There is NO preceding dollar sign, which means that this identifier should be interpreted"] # [doc = " as a literal."] Ident (tt :: Ident < Span >) , # [doc = " There is a preceding dollar sign, which means that this identifier should be expanded"] # [doc = " and interpreted as a variable."] Var (tt :: Ident < Span >) , # [doc = " For example, a number or a string."] Literal (tt :: Literal < Span >) , }
};
}
