// Generated macro for Redaction (enum)
macro_rules! Depcrate_redactionRedaction {
() => {
// Module: crate::redaction
// Provides: {"Redaction"}
// Dependencies: {}
# [doc = " Replaces a value with another one."] # [doc = ""] # [doc = " Represents a redaction."] # [cfg_attr (docsrs , doc (cfg (feature = "redactions")))] pub enum Redaction { # [doc = " Static redaction with new content."] Static (Content) , # [doc = " Redaction with new content."] Dynamic (Box < dyn Fn (Content , ContentPath < '_ >) -> Content + Sync + Send >) , }
};
}
