// Generated macro for remote (module)
macro_rules! Depcrate_plumbing_optionsremote {
() => {
// Module: crate::plumbing::options
// Provides: {"remote"}
// Dependencies: {}
# [cfg (any (feature = "gitoxide-core-async-client" , feature = "gitoxide-core-blocking-client"))] pub mod remote { # [derive (Debug , clap :: Parser)] pub struct Platform { # [doc = " The name of the remote to connect to, or the URL of the remote to connect to directly."] # [doc = ""] # [doc = " If unset, the current branch will determine the remote."] # [clap (long , short = 'n')] pub name : Option < String > , # [doc = " Output additional typically information provided by the server as part of the connection handshake."] # [clap (long , short = 'H')] pub handshake_info : bool , # [doc = " Subcommands"] # [clap (subcommand)] pub cmd : Subcommands , } # [derive (Debug , clap :: Subcommand)] # [clap (visible_alias = "remotes")] pub enum Subcommands { # [doc = " Print all references available on the remote."] Refs , # [doc = " Print all references available on the remote as filtered through ref-specs."] RefMap { # [doc = " Also display remote references that were sent by the server, but filtered by the refspec locally."] # [clap (long , short = 'u')] show_unmapped_remote_refs : bool , # [doc = " Override the built-in and configured ref-specs with one or more of the given ones."] # [clap (value_parser = crate :: shared :: AsBString)] ref_spec : Vec < gix :: bstr :: BString > , } , } }
};
}
