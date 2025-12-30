// Generated macro for QuotingStrategy (enum)
macro_rules! Depcrate_bytesQuotingStrategy {
() => {
// Module: crate::bytes
// Provides: {"QuotingStrategy"}
// Dependencies: {}
# [derive (PartialEq)] enum QuotingStrategy { # [doc = " No quotes and no backslash escapes.  (If backslash escapes would be necessary, we use a"] # [doc = " different strategy instead.)"] Unquoted , # [doc = " Single quoted."] SingleQuoted , # [doc = " Double quotes, potentially with backslash escapes."] DoubleQuoted , }
};
}
