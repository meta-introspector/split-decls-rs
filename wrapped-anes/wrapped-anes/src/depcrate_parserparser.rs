// Generated macro for Parser (struct)
macro_rules! Depcrate_parserParser {
() => {
// Module: crate::parser
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " An ANSI escape sequence parser."] # [doc = ""] # [doc = " `Parser` implements the `Iterator<Item = Sequence>` trait, thus you can use the"] # [doc = " `next()` method to consume all valid sequences with known meaning."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Parse cursor position:"] # [doc = ""] # [doc = " ```"] # [doc = " use anes::parser::{Parser, Sequence};"] # [doc = ""] # [doc = " let mut parser = Parser::default();"] # [doc = " parser.advance(b\"\\x1B[20;10R\", false);"] # [doc = ""] # [doc = " assert_eq!(Some(Sequence::CursorPosition(10, 20)), parser.next());"] # [doc = " assert!(parser.next().is_none());"] # [doc = " ```"] # [doc = ""] # [doc = " Parse keyboard event:"] # [doc = ""] # [doc = " ```"] # [doc = " use anes::parser::{KeyCode, KeyModifiers, Parser, Sequence};"] # [doc = ""] # [doc = " let mut parser = Parser::default();"] # [doc = " parser.advance(\"𐌼a\".as_bytes(), false);"] # [doc = ""] # [doc = " assert_eq!(Some(Sequence::Key(KeyCode::Char('𐌼'), KeyModifiers::empty())), parser.next());"] # [doc = " assert_eq!(Some(Sequence::Key(KeyCode::Char('a'), KeyModifiers::empty())), parser.next());"] # [doc = " assert!(parser.next().is_none());"] # [doc = " ```"] # [derive (Default)] pub struct Parser { engine : Engine , provider : SequenceProvider , }
};
}
