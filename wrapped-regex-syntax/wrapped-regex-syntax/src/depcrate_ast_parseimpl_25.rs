// Generated macro for impl_25 (impl)
macro_rules! Depcrate_ast_parseimpl_25 {
() => {
// Module: crate::ast::parse
// Provides: {"impl_25"}
// Dependencies: {}
impl Parser { # [doc = " Create a new parser with a default configuration."] # [doc = ""] # [doc = " The parser can be run with either the `parse` or `parse_with_comments`"] # [doc = " methods. The parse methods return an abstract syntax tree."] # [doc = ""] # [doc = " To set configuration options on the parser, use [`ParserBuilder`]."] pub fn new () -> Parser { ParserBuilder :: new () . build () } # [doc = " Parse the regular expression into an abstract syntax tree."] pub fn parse (& mut self , pattern : & str) -> Result < Ast > { ParserI :: new (self , pattern) . parse () } # [doc = " Parse the regular expression and return an abstract syntax tree with"] # [doc = " all of the comments found in the pattern."] pub fn parse_with_comments (& mut self , pattern : & str ,) -> Result < ast :: WithComments > { ParserI :: new (self , pattern) . parse_with_comments () } # [doc = " Reset the internal state of a parser."] # [doc = ""] # [doc = " This is called at the beginning of every parse. This prevents the"] # [doc = " parser from running with inconsistent state (say, if a previous"] # [doc = " invocation returned an error and the parser is reused)."] fn reset (& self) { self . pos . set (Position { offset : 0 , line : 1 , column : 1 }) ; self . ignore_whitespace . set (self . initial_ignore_whitespace) ; self . comments . borrow_mut () . clear () ; self . stack_group . borrow_mut () . clear () ; self . stack_class . borrow_mut () . clear () ; } }
};
}
