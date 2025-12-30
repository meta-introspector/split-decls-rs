// Generated macro for Parser (struct)
macro_rules! Depcrate_parser_parserParser {
() => {
// Module: crate::parser::parser
// Provides: {"Parser"}
// Dependencies: {}
pub (crate) struct Parser < 'cmd > { cmd : & 'cmd mut Command , cur_idx : Cell < usize > , # [doc = " Index of the previous flag subcommand in a group of flags."] flag_subcmd_at : Option < usize > , # [doc = " Counter indicating the number of items to skip"] # [doc = " when revisiting the group of flags which includes the flag subcommand."] flag_subcmd_skip : usize , }
};
}
