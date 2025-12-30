// Generated macro for worktrees_env_init (function)
macro_rules! Depcrate_testworktrees_env_init {
() => {
// Module: crate::test
// Provides: {"worktrees_env_init"}
// Dependencies: {}
pub fn worktrees_env_init (repo : & Repository) -> (TempDir , Branch < '_ >) { let oid = repo . head () . unwrap () . target () . unwrap () ; let commit = repo . find_commit (oid) . unwrap () ; let branch = repo . branch ("wt-branch" , & commit , true) . unwrap () ; let wtdir = TempDir :: new () . unwrap () ; (wtdir , branch) }
};
}
