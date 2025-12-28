macro_rules! deps {
    () => {
        HeaderValue!();
        PasswordClientBuilder!();
        Error!();
        PasswordClient!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        # [doc = " Tries to create a `PasswordClient` from the supplied `http::header::GetAll` challenge lists."] # [doc = ""] # [doc = " This is a convenience wrapper around [`PasswordClientBuilder`]."] # [cfg (feature = "http")] # [cfg_attr (docsrs , doc (cfg (feature = "http")))] impl TryFrom < http :: header :: GetAll < '_ , http :: HeaderValue > > for PasswordClient { type Error = String ; fn try_from (value : http :: header :: GetAll < '_ , http :: HeaderValue >) -> Result < Self , Self :: Error > { let mut builder = PasswordClient :: builder () ; for v in value { builder = builder . header_value (v) ; } builder . build () } }
    };
}

impl_78!()