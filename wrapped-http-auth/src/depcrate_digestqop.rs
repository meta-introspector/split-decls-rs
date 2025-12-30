// Generated macro for Qop (enum)
macro_rules! Depcrate_digestQop {
() => {
// Module: crate::digest
// Provides: {"Qop"}
// Dependencies: {}
# [doc = " \"Quality of protection\" value."] # [doc = ""] # [doc = " The values here can be used in a bitmask as in [`DigestClient::qop`]."] # [derive (Copy , Clone , Debug)] # [repr (u8)] # [non_exhaustive] pub enum Qop { # [doc = " Authentication."] Auth = 1 , # [doc = " Authentication with integrity protection."] # [doc = ""] # [doc = " \"Integrity protection\" means protection of the request entity body."] AuthInt = 2 , }
};
}
