macro_rules! Auth {
    () => {
        # [doc = " Structure which stores possible authentication methods to get passed to"] # [doc = " `http_auth` and `proxy_auth`."] # [derive (Clone)] pub struct Auth { bits : c_long , }
    };
}

Auth!()