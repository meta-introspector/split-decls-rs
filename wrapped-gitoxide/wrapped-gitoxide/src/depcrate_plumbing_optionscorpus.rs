// Generated macro for corpus (module)
macro_rules! Depcrate_plumbing_optionscorpus {
() => {
// Module: crate::plumbing::options
// Provides: {"corpus"}
// Dependencies: {}
# [cfg (feature = "gitoxide-core-tools-corpus")] pub mod corpus { use std :: path :: PathBuf ; # [derive (Debug , clap :: Parser)] # [command (about = "Run algorithms on a corpus of git repositories and store their results for later analysis")] pub struct Platform { # [doc = " The path to the database to read and write depending on the sub-command."] # [arg (long , default_value = "corpus.db")] pub db : PathBuf , # [doc = " The path to the root of the corpus to search repositories in."] # [arg (long , short = 'p' , default_value = ".")] pub path : PathBuf , # [clap (subcommand)] pub cmd : SubCommands , } # [derive (Debug , clap :: Subcommand)] pub enum SubCommands { # [doc = " Perform a corpus run on all registered repositories."] Run { # [doc = " Don't run any task, but print all repos that would be traversed once."] # [doc = ""] # [doc = " Note that this will refresh repositories if necessary and store them in the database, it just won't run tasks."] # [clap (long , short = 'n')] dry_run : bool , # [doc = " The SQL that will be appended to the actual select statement for repositories to apply additional filtering, like `LIMIT 10`."] # [doc = ""] # [doc = " The string must be trusted even though the engine will only execute a single statement."] # [clap (long , short = 'r')] repo_sql_suffix : Option < String > , # [doc = " The short_names of the tasks to include when running."] # [clap (long , short = 't')] include_task : Vec < String > , } , # [doc = " Re-read all repositories under the corpus directory, and add or update them."] Refresh , } }
};
}
