macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! SslVersion {
    () => {
        deps!();
        # [doc = " The `http.sslVersion` key, as well as others of the same type."] pub type SslVersion = keys :: Any < validate :: SslVersion > ;
    };
}

SslVersion!();