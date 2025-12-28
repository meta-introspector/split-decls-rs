macro_rules! deps {
    () => {
        HeaderValue!();
        PasswordClient!();
        PasswordClientBuilder!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [doc = " Tries to create a `PasswordClient` from the supplied `HeaderValue` challenge list."] # [doc = ""] # [doc = " This is a convenience wrapper around [`PasswordClientBuilder`]."] # [cfg (feature = "http")] # [cfg_attr (docsrs , doc (cfg (feature = "http")))] impl TryFrom < & http :: HeaderValue > for PasswordClient { type Error = String ; # [inline] fn try_from (value : & http :: HeaderValue) -> Result < Self , Self :: Error > { PasswordClient :: builder () . header_value (value) . build () } }
    };
}

impl_19!()