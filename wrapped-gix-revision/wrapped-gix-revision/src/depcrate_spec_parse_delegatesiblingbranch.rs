// Generated macro for SiblingBranch (enum)
macro_rules! Depcrate_spec_parse_delegateSiblingBranch {
() => {
// Module: crate::spec::parse::delegate
// Provides: {"SiblingBranch"}
// Dependencies: {}
# [doc = " The kind of sibling branch to obtain."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub enum SiblingBranch { # [doc = " The upstream branch as configured in `branch.<name>.remote` or `branch.<name>.merge`."] Upstream , # [doc = " The upstream branch to which we would push."] Push , }
};
}
