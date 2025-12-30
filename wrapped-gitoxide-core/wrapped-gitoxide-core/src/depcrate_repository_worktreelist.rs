// Generated macro for list (function)
macro_rules! Depcrate_repository_worktreelist {
() => {
// Module: crate::repository::worktree
// Provides: {"list"}
// Dependencies: {}
pub fn list (repo : gix :: Repository , out : & mut dyn std :: io :: Write , format : OutputFormat) -> anyhow :: Result < () > { if format != OutputFormat :: Human { bail ! ("JSON output isn't implemented yet") ; } if let Some (worktree) = repo . worktree () { writeln ! (out , "{base} [{branch}]" , base = gix :: path :: realpath (worktree . base ()) ?. display () , branch = repo . head_name () ? . map_or ("<detached>" . into () , | name | name . shorten () . to_owned ()) ,) ? ; } for proxy in repo . worktrees () ? { writeln ! (out , "{base} [{name}]" , base = proxy . base () ?. display () , name = proxy . id ()) ? ; } Ok (()) }
};
}
