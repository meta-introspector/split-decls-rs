macro_rules! deps {
    () => {
        DigestClient!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl std :: fmt :: Debug for DigestClient { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("DigestClient") . field ("realm" , & self . realm ()) . field ("domain" , & self . domain ()) . field ("opaque" , & self . opaque ()) . field ("nonce" , & self . nonce ()) . field ("algorithm" , & self . algorithm . as_str (self . session)) . field ("stale" , & self . stale) . field ("qop" , & self . qop) . field ("rfc2069_compat" , & self . rfc2069_compat) . field ("userhash" , & self . userhash) . field ("nc" , & self . nc) . finish () } }
    };
}

impl_35!();