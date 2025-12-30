// Generated macro for Diff (enum)
macro_rules! Depcrate_diffDiff {
() => {
// Module: crate::diff
// Provides: {"Diff"}
// Dependencies: {}
# [doc = " A type returned by the [`diff_with`] function."] # [doc = ""] # [doc = " `Diff` represents the way in which the elements yielded by the iterator `I` differ to some"] # [doc = " iterator `J`."] pub enum Diff < I , J > where I : Iterator , J : Iterator , { # [doc = " The index of the first non-matching element along with both iterator's remaining elements"] # [doc = " starting with the first mis-match."] FirstMismatch (usize , PutBack < I > , PutBack < J >) , # [doc = " The total number of elements that were in `J` along with the remaining elements of `I`."] Shorter (usize , PutBack < I >) , # [doc = " The total number of elements that were in `I` along with the remaining elements of `J`."] Longer (usize , PutBack < J >) , }
};
}
