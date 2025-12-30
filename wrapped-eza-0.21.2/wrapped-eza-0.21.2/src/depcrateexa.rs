// Generated macro for Exa (struct)
macro_rules! DepcrateExa {
() => {
// Module: crate
// Provides: {"Exa"}
// Dependencies: {}
# [doc = " The main program wrapper."] pub struct Exa < 'args > { # [doc = " List of command-line options, having been successfully parsed."] pub options : Options , # [doc = " The output handle that we write to."] pub writer : io :: Stdout , # [doc = " List of the free command-line arguments that should correspond to file"] # [doc = " names (anything that isn’t an option)."] pub input_paths : Vec < & 'args OsStr > , # [doc = " The theme that has been configured from the command-line options and"] # [doc = " environment variables. If colours are disabled, this is a theme with"] # [doc = " every style set to the default."] pub theme : Theme , # [doc = " The detected width of the console. This is used to determine which"] # [doc = " view to use."] pub console_width : Option < usize > , # [doc = " A global Git cache, if the option was passed in."] # [doc = " This has to last the lifetime of the program, because the user might"] # [doc = " want to list several directories in the same repository."] pub git : Option < GitCache > , pub git_repos : bool , }
};
}
