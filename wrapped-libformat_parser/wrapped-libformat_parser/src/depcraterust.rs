// Generated macro for rust (module)
macro_rules! Depcraterust {
() => {
// Module: crate
// Provides: {"rust"}
// Dependencies: {}
pub mod rust { use crate :: ffi :: ParseMode ; use generic_format_parser :: { Parser , Piece } ; pub fn collect_pieces (input : & str , style : Option < usize > , snippet : Option < String > , append_newline : bool , parse_mode : ParseMode ,) -> Vec < Piece < '_ > > { let converted_parse_mode = match parse_mode { ParseMode :: Format => generic_format_parser :: ParseMode :: Format , ParseMode :: InlineAsm => generic_format_parser :: ParseMode :: InlineAsm , } ; let parser = Parser :: new (input , style , snippet , append_newline , converted_parse_mode) ; parser . into_iter () . collect () } }
};
}
