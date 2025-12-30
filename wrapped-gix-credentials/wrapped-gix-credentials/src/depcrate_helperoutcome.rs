// Generated macro for Outcome (struct)
macro_rules! Depcrate_helperOutcome {
() => {
// Module: crate::helper
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The outcome of the credentials helper [invocation][crate::helper::invoke()]."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct Outcome { # [doc = " The username to use in the identity, if set."] pub username : Option < String > , # [doc = " The password to use in the identity, if set."] pub password : Option < String > , # [doc = " An OAuth refresh token that may accompany a password. It is to be treated confidentially, just like the password."] pub oauth_refresh_token : Option < String > , # [doc = " If set, the helper asked to stop the entire process, whether the identity is complete or not."] pub quit : bool , # [doc = " A handle to the action to perform next in another call to [`helper::invoke()`][crate::helper::invoke()]."] pub next : NextAction , }
};
}
