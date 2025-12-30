// Generated macro for TrailerRef (struct)
macro_rules! Depcrate_commit_message_bodyTrailerRef {
() => {
// Module: crate::commit::message::body
// Provides: {"TrailerRef"}
// Dependencies: {}
# [doc = " A trailer as parsed from the commit message body."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct TrailerRef < 'a > { # [doc = " The name of the trailer, like \"Signed-off-by\", up to the separator `: `."] # [cfg_attr (feature = "serde" , serde (borrow))] pub token : & 'a BStr , # [doc = " The value right after the separator `: `, with leading and trailing whitespace trimmed."] # [doc = " Note that multi-line values aren't currently supported."] pub value : & 'a BStr , }
};
}
