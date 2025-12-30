// Generated macro for RebaseOperationType (enum)
macro_rules! Depcrate_rebaseRebaseOperationType {
() => {
// Module: crate::rebase
// Provides: {"RebaseOperationType"}
// Dependencies: {}
# [doc = " A rebase operation"] # [doc = ""] # [doc = " Describes a single instruction/operation to be performed during the"] # [doc = " rebase."] # [derive (Debug , PartialEq)] pub enum RebaseOperationType { # [doc = " The given commit is to be cherry-picked. The client should commit the"] # [doc = " changes and continue if there are no conflicts."] Pick , # [doc = " The given commit is to be cherry-picked, but the client should prompt"] # [doc = " the user to provide an updated commit message."] Reword , # [doc = " The given commit is to be cherry-picked, but the client should stop to"] # [doc = " allow the user to edit the changes before committing them."] Edit , # [doc = " The given commit is to be squashed into the previous commit. The commit"] # [doc = " message will be merged with the previous message."] Squash , # [doc = " The given commit is to be squashed into the previous commit. The commit"] # [doc = " message from this commit will be discarded."] Fixup , # [doc = " No commit will be cherry-picked. The client should run the given command"] # [doc = " and (if successful) continue."] Exec , }
};
}
