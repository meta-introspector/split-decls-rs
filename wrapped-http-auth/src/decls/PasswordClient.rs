macro_rules! deps {
    () => {
        BasicClient!();
        PasswordClientBuilder!();
        DigestClient!();
    };
}

macro_rules! PasswordClient {
    () => {
        deps!();
        # [doc = " Client for responding to a password challenge."] # [doc = ""] # [doc = " Typically created via [`TryFrom`] implementations for a parsed challenge"] # [doc = " ([`crate::ChallengeRef`]) or unparsed challenges (`str`,"] # [doc = " [`http::header::HeaderValue`], or [`http::header::GetAll`]). See full"] # [doc = " example in the [crate-level documentation](crate)."] # [doc = ""] # [doc = " For more complex scenarios, see [`PasswordClientBuilder`]."] # [derive (Debug , Eq , PartialEq)] # [non_exhaustive] pub enum PasswordClient { # [cfg (feature = "basic-scheme")] # [cfg_attr (docsrs , doc (cfg (feature = "basic-scheme")))] Basic (BasicClient) , # [cfg (feature = "digest-scheme")] # [cfg_attr (docsrs , doc (cfg (feature = "digest-scheme")))] Digest (DigestClient) , }
    };
}

PasswordClient!();