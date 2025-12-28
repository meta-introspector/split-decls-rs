macro_rules! deps {
    () => {
        PasswordClientBuilder!();
        ChallengeRef!();
        DigestClient!();
        Error!();
        PasswordClient!();
        BasicClient!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        # [doc = " Tries to create a `PasswordClient` from the single supplied challenge."] # [doc = ""] # [doc = " This is a convenience wrapper around [`PasswordClientBuilder`]."] impl TryFrom < & ChallengeRef < '_ > > for PasswordClient { type Error = String ; fn try_from (value : & ChallengeRef < '_ >) -> Result < Self , Self :: Error > { # [cfg (feature = "basic-scheme")] if value . scheme . eq_ignore_ascii_case ("Basic") { return Ok (PasswordClient :: Basic (BasicClient :: try_from (value) ?)) ; } # [cfg (feature = "digest-scheme")] if value . scheme . eq_ignore_ascii_case ("Digest") { return Ok (PasswordClient :: Digest (DigestClient :: try_from (value) ?)) ; } Err (format ! ("unsupported challenge scheme {:?}" , value . scheme)) } }
    };
}

impl_74!()