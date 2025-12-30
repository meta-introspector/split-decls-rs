// Generated macro for Error (enum)
macro_rules! Depcrate_codepointtrie_errorError {
() => {
// Module: crate::codepointtrie::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " A custom error type for [`CodePointTrie`](super::CodePointTrie)."] # [derive (Copy , Clone , Display , Debug , PartialEq)] # [non_exhaustive] pub enum Error { # [doc = " Could not construct [`CodePointTrie`](super::CodePointTrie) from deserialized values"] # [displaydoc ("Could not construct CodePointTrie from deserialized values: {reason}")] FromDeserialized { # [doc = " Reason for inability to deserialize values."] reason : & 'static str , } , # [doc = " [`CodePointTrie`](super::CodePointTrie) must be constructed from data vector with at least one element"] # [displaydoc ("CodePointTrie must be constructed from data vector with at least one element")] EmptyDataVector , # [doc = " [`CodePointTrie`](super::CodePointTrie) must be constructed from index vector long enough to accommodate fast-path access"] # [displaydoc ("CodePointTrie must be constructed from index vector long enough to accommodate fast-path access")] IndexTooShortForFastAccess , # [doc = " [`CodePointTrie`](super::CodePointTrie) must be constructed from data vector long enough to accommodate fast-path access"] # [displaydoc ("CodePointTrie must be constructed from data vector long enough to accommodate fast-path access")] DataTooShortForFastAccess , }
};
}
