// Generated macro for Action (enum)
macro_rules! Depcrate_helperAction {
() => {
// Module: crate::helper
// Provides: {"Action"}
// Dependencies: {}
# [doc = " The action to perform by the credentials [helper][`crate::helper::invoke()`]."] # [derive (Clone , Debug)] pub enum Action { # [doc = " Provide credentials using the given repository context, which must include the repository url."] Get (Context) , # [doc = " Approve the credentials as identified by the previous input provided as `BString`, containing information from [`Context`]."] Store (BString) , # [doc = " Reject the credentials as identified by the previous input provided as `BString`. containing information from [`Context`]."] Erase (BString) , }
};
}
