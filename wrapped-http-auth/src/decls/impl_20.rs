macro_rules! deps {
    () => {
        PasswordClient!();
        PasswordClientBuilder!();
        HeaderValue!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [doc = " Tries to create a `PasswordClient` from the supplied `HeaderValue` challenge list."] # [doc = ""] # [doc = " This is a convenience wrapper around [`PasswordClientBuilder`]."] # [cfg (feature = "http10")] # [cfg_attr (docsrs , doc (cfg (feature = "http10")))] impl TryFrom < & http10 :: HeaderValue > for PasswordClient { type Error = String ; # [inline] fn try_from (value : & http10 :: HeaderValue) -> Result < Self , Self :: Error > { PasswordClient :: builder () . header_value (value) . build () } }
    };
}

impl_20!()