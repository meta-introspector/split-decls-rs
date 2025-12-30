// Generated macro for Parser (struct)
macro_rules! Depcrate_parserParser {
() => {
// Module: crate::parser
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " A convenience parser for regular expressions."] # [doc = ""] # [doc = " This parser takes as input a regular expression pattern string (the"] # [doc = " \"concrete syntax\") and returns a high-level intermediate representation"] # [doc = " (the HIR) suitable for most types of analysis. In particular, this parser"] # [doc = " hides the intermediate state of producing an AST (the \"abstract syntax\")."] # [doc = " The AST is itself far more complex than the HIR, so this parser serves as a"] # [doc = " convenience for never having to deal with it at all."] # [doc = ""] # [doc = " If callers have more fine grained use cases that need an AST, then please"] # [doc = " see the [`ast::parse`] module."] # [doc = ""] # [doc = " A `Parser` can be configured in more detail via a [`ParserBuilder`]."] # [derive (Clone , Debug)] pub struct Parser { ast : ast :: parse :: Parser , hir : hir :: translate :: Translator , }
};
}
