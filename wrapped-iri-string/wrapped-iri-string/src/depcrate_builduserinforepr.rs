// Generated macro for UserinfoRepr (enum)
macro_rules! Depcrate_buildUserinfoRepr {
() => {
// Module: crate::build
// Provides: {"UserinfoRepr"}
// Dependencies: {}
# [doc = " Internal representation of a userinfo builder."] # [derive (Clone , Copy)] enum UserinfoRepr < 'a > { # [doc = " Not specified (absent)."] None , # [doc = " Direct `userinfo` content."] Direct (& 'a str) , # [doc = " User name and password."] UserPass (& 'a str , Option < & 'a str >) , }
};
}
