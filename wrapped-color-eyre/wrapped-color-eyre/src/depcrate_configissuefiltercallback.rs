// Generated macro for IssueFilterCallback (type)
macro_rules! Depcrate_configIssueFilterCallback {
() => {
// Module: crate::config
// Provides: {"IssueFilterCallback"}
// Dependencies: {}
# [doc = " Callback for filtering issue url generation in error reports"] # [cfg (feature = "issue-url")] # [cfg_attr (docsrs , doc (cfg (feature = "issue-url")))] pub type IssueFilterCallback = dyn Fn (crate :: ErrorKind < '_ >) -> bool + Send + Sync + 'static ;
};
}
