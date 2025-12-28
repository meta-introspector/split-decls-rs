macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! ProxyAuthMethod {
    () => {
        deps!();
        # [doc = " The `http.proxyAuthMethod` key, as well as others of the same type."] pub type ProxyAuthMethod = keys :: Any < validate :: ProxyAuthMethod > ;
    };
}

ProxyAuthMethod!();