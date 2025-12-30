// Generated macro for parse_name_and_section (function)
macro_rules! Depcrate_utilparse_name_and_section {
() => {
// Module: crate::util
// Provides: {"parse_name_and_section"}
// Dependencies: {}
# [doc = " Splits the text `foo(1)` into \"foo\" and `1`."] pub fn parse_name_and_section (text : & str) -> Result < (& str , u8) , Error > { let mut i = text . split_terminator (& ['(' , ')'] [..]) ; let name = i . next () . ok_or_else (| | format_err ! ("man reference must have a name")) ? ; let section = i . next () . ok_or_else (| | format_err ! ("man reference must have a section such as mycommand(1)")) ? ; if let Some (s) = i . next () { bail ! ("man reference must have the form mycommand(1), got extra part `{}`" , s) ; } let section : u8 = section . parse () . with_context (| | format ! ("section must be a number, got {}" , section)) ? ; Ok ((name , section)) }
};
}
