// Generated macro for RemoteRedirect (enum)
macro_rules! Depcrate_remoteRemoteRedirect {
() => {
// Module: crate::remote
// Provides: {"RemoteRedirect"}
// Dependencies: {}
# [doc = " Remote redirection settings; whether redirects to another host are"] # [doc = " permitted."] # [doc = ""] # [doc = " By default, git will follow a redirect on the initial request"] # [doc = " (`/info/refs`), but not subsequent requests."] pub enum RemoteRedirect { # [doc = " Do not follow any off-site redirects at any stage of the fetch or push."] None , # [doc = " Allow off-site redirects only upon the initial request. This is the"] # [doc = " default."] Initial , # [doc = " Allow redirects at any stage in the fetch or push."] All , }
};
}
