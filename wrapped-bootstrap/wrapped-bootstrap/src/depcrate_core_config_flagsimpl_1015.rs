// Generated macro for impl_1015 (impl)
macro_rules! Depcrate_core_config_flagsimpl_1015 {
() => {
// Module: crate::core::config::flags
// Provides: {"impl_1015"}
// Dependencies: {}
impl Flags { # [doc = " Check if `<cmd> -h -v` was passed."] # [doc = " If yes, print the available paths and return `true`."] pub fn try_parse_verbose_help (args : & [String]) -> bool { # [derive (Parser)] # [command (disable_help_flag (true))] struct HelpVerboseOnly { # [arg (short , long)] help : bool , # [arg (global = true , short , long , action = clap :: ArgAction :: Count)] pub verbose : u8 , # [arg (value_enum)] cmd : Kind , } if let Ok (HelpVerboseOnly { help : true , verbose : 1 .. , cmd : subcommand }) = HelpVerboseOnly :: try_parse_from (normalize_args (args)) { println ! ("NOTE: updating submodules before printing available paths") ; let flags = Self :: parse (& [String :: from ("build")]) ; let config = Config :: parse (flags) ; let build = Build :: new (config) ; let paths = Builder :: get_help (& build , subcommand) ; if let Some (s) = paths { println ! ("{s}") ; } else { panic ! ("No paths available for subcommand `{}`" , subcommand . as_str ()) ; } true } else { false } } # [cfg_attr (feature = "tracing" , instrument (level = "trace" , name = "Flags::parse" , skip_all , fields (args = ? args)))] pub fn parse (args : & [String]) -> Self { Flags :: parse_from (normalize_args (args)) } }
};
}
