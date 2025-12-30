// Generated macro for CommitTimeOrder (enum)
macro_rules! Depcrate_commit_simpleCommitTimeOrder {
() => {
// Module: crate::commit::simple
// Provides: {"CommitTimeOrder"}
// Dependencies: {}
# [derive (Default , Debug , Copy , Clone)] # [doc = " The order with which to prioritize the search."] pub enum CommitTimeOrder { # [default] # [doc = " Sort commits by newest first."] NewestFirst , # [doc = " Sort commits by oldest first."] # [doc (alias = "Sort::REVERSE" , alias = "git2")] OldestFirst , }
};
}
