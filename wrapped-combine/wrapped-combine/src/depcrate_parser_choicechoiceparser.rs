// Generated macro for ChoiceParser (trait)
macro_rules! Depcrate_parser_choiceChoiceParser {
() => {
// Module: crate::parser::choice
// Provides: {"ChoiceParser"}
// Dependencies: {}
# [doc = " `ChoiceParser` represents a parser which may parse one of several different choices depending"] # [doc = " on the input."] # [doc = ""] # [doc = " This is an internal trait used to overload the `choice` function."] pub trait ChoiceParser < Input : Stream > { type Output ; type PartialState : Default ; fn parse_first (& mut self , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > ; fn parse_partial (& mut self , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > ; fn parse_mode_choice < M > (& mut self , mode : M , input : & mut Input , state : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , Self : Sized ; fn add_error_choice (& mut self , error : & mut Tracked < < Input as StreamOnce > :: Error >) ; }
};
}
