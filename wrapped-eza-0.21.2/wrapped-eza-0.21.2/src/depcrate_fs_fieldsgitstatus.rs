// Generated macro for GitStatus (enum)
macro_rules! Depcrate_fs_fieldsGitStatus {
() => {
// Module: crate::fs::fields
// Provides: {"GitStatus"}
// Dependencies: {}
# [doc = " A file’s status in a Git repository. Whether a file is in a repository or"] # [doc = " not is handled by the Git module, rather than having a “null” variant in"] # [doc = " this enum."] # [derive (PartialEq , Eq , Copy , Clone)] pub enum GitStatus { # [doc = " This file hasn’t changed since the last commit."] NotModified , # [doc = " This file didn’t exist for the last commit, and is not specified in"] # [doc = " the ignored files list."] New , # [doc = " A file that’s been modified since the last commit."] Modified , # [doc = " A deleted file. This can’t ever be shown, but it’s here anyway!"] Deleted , # [doc = " A file that Git has tracked a rename for."] Renamed , # [doc = " A file that’s had its type (such as the file permissions) changed."] TypeChange , # [doc = " A file that’s ignored (that matches a line in .gitignore)"] Ignored , # [doc = " A file that’s updated but unmerged."] Conflicted , }
};
}
