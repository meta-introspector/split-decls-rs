// Generated macro for ParseMode (trait)
macro_rules! Depcrate_parserParseMode {
() => {
// Module: crate::parser
// Provides: {"ParseMode"}
// Dependencies: {}
# [doc = " Internal API. May break without a semver bump"] # [doc (hidden)] # [doc = " Specifies whether the parser must check for partial state that must be resumed"] pub trait ParseMode : Copy { # [doc = " If `true` then the parser has no previous state to resume otherwise the parser *might* have"] # [doc = " state to resume which it must check."] fn is_first (self) -> bool ; # [doc = " Puts the mode into `first` parsing."] fn set_first (& mut self) ; fn parse < P , Input > (self , parser : & mut P , input : & mut Input , state : & mut P :: PartialState ,) -> ParseResult < P :: Output , Input :: Error > where P : Parser < Input > , Input : Stream ; # [inline] fn parse_committed < P , Input > (self , parser : & mut P , input : & mut Input , state : & mut P :: PartialState ,) -> ParseResult < P :: Output , < Input as StreamOnce > :: Error > where P : Parser < Input > , Input : Stream , { let before = input . checkpoint () ; let mut result = parser . parse_mode_impl (self , input , state) ; if let ParseResult :: PeekErr (ref mut error) = result { ctry ! (input . reset (before . clone ()) . committed ()) ; if let Ok (t) = input . uncons () { ctry ! (input . reset (before) . committed ()) ; error . error . add_unexpected (Token (t)) ; } else { error . error . add (StreamErrorFor :: < Input > :: end_of_input ()) ; } parser . add_error (error) ; } result } }
};
}
