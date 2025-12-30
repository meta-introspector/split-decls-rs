// Generated macro for RefLog (enum)
macro_rules! Depcrate_transactionRefLog {
() => {
// Module: crate::transaction
// Provides: {"RefLog"}
// Dependencies: {}
# [doc = " The way to deal with the Reflog in deletions."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub enum RefLog { # [doc = " Delete or update the reference and the log"] AndReference , # [doc = " Delete or update only the reflog"] Only , }
};
}
