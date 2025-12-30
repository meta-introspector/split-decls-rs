// Generated macro for ParserNode (struct)
macro_rules! Depcrate_parserParserNode {
() => {
// Module: crate::parser
// Provides: {"ParserNode"}
// Dependencies: {}
# [doc = " The pest grammar node"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct ParserNode < 'i > { # [doc = " The node's expression"] pub expr : ParserExpr < 'i > , # [doc = " The node's span"] pub span : Span < 'i > , }
};
}
