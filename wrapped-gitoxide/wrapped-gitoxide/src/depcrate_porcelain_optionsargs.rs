// Generated macro for Args (struct)
macro_rules! Depcrate_porcelain_optionsArgs {
() => {
// Module: crate::porcelain::options
// Provides: {"Args"}
// Dependencies: {}
# [derive (Debug , clap :: Parser)] # [clap (name = "ein" , about = "The rusty git" , version = option_env ! ("GIX_VERSION"))] # [clap (subcommand_required = true)] pub struct Args { # [doc = " Do not display verbose messages and progress information."] # [clap (long , short = 'q')] pub quiet : bool , # [doc = " Bring up a terminal user interface displaying progress visually."] # [clap (long , conflicts_with ("quiet"))] pub progress : bool , # [doc = " The number of threads to use. If unset, use all cores, if 0 use all physical cores."] # [clap (short = 't' , long)] pub threads : Option < usize > , # [doc = " The progress TUI will stay up even though the work is already completed."] # [doc = ""] # [doc = " Use this to be able to read progress messages or additional information visible in the TUI log pane."] # [clap (long , conflicts_with ("quiet") , requires ("progress"))] pub progress_keep_open : bool , # [clap (subcommand)] pub cmd : Subcommands , }
};
}
