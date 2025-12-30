// Generated macro for invoke (function)
macro_rules! Depcrate_helper_invokeinvoke {
() => {
// Module: crate::helper::invoke
// Provides: {"invoke"}
// Dependencies: {}
# [doc = " Invoke the given `helper` with `action` in `context`."] # [doc = ""] # [doc = " Usually the first call is performed with [`Action::Get`] to obtain `Some` identity, which subsequently can be used if it is complete."] # [doc = " Note that it may also only contain the username _or_ password, and should start out with everything the helper needs."] # [doc = " On successful usage, use [`NextAction::store()`], otherwise [`NextAction::erase()`], which is when this function"] # [doc = " returns `Ok(None)` as no outcome is expected."] pub fn invoke (helper : & mut crate :: Program , action : & Action) -> Result { match raw (helper , action) ? { None => Ok (None) , Some (stdout) => { let ctx = Context :: from_bytes (stdout . as_slice ()) ? ; Ok (Some (Outcome { username : ctx . username , password : ctx . password , oauth_refresh_token : ctx . oauth_refresh_token , quit : ctx . quit . unwrap_or (false) , next : NextAction { previous_output : stdout . into () , } , })) } } }
};
}
