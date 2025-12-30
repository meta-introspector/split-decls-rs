// Generated macro for Submodules (enum)
macro_rules! Depcrate_repository_statusSubmodules {
() => {
// Module: crate::repository::status
// Provides: {"Submodules"}
// Dependencies: {}
pub enum Submodules { # [doc = " display all information about submodules, including ref changes, modifications and untracked files."] All , # [doc = " Compare only the configuration of the superprojects commit with the actually checked out `HEAD` commit."] RefChange , # [doc = " See if there are worktree modifications compared to the index, but do not check for untracked files."] Modifications , # [doc = " Ignore all submodule changes."] None , }
};
}
