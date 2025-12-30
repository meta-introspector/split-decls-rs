// Generated macro for impl_95 (impl)
macro_rules! Depcrateimpl_95 {
() => {
// Module: crate
// Provides: {"impl_95"}
// Dependencies: {}
impl PasswordClient { # [doc = " Builds a new `PasswordClient`."] # [doc = ""] # [doc = " See example at [`PasswordClientBuilder`]."] pub fn builder () -> PasswordClientBuilder { PasswordClientBuilder :: default () } # [doc = " Responds to the challenge with the supplied parameters."] # [doc = ""] # [doc = " The caller should use the returned string as an `Authorization` or"] # [doc = " `Proxy-Authorization` header value."] # [allow (unused_variables)] pub fn respond (& mut self , p : & PasswordParams) -> Result < String , String > { match self { # [cfg (feature = "basic-scheme")] Self :: Basic (c) => Ok (c . respond (p . username , p . password)) , # [cfg (feature = "digest-scheme")] Self :: Digest (c) => c . respond (p) , # [cfg (not (any (feature = "basic-scheme" , feature = "digest-scheme")))] _ => unreachable ! () , } } }
};
}
