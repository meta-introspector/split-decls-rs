// Generated macro for Account (struct)
macro_rules! Depcrate_identityAccount {
() => {
// Module: crate::identity
// Provides: {"Account"}
// Dependencies: {}
# [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [doc = " An account based identity"] pub struct Account { # [doc = " The user's name"] pub username : String , # [doc = " The user's password"] pub password : String , # [doc = " An OAuth refresh token that may accompany the password. It is to be treated confidentially, just like the password."] pub oauth_refresh_token : Option < String > , }
};
}
