macro_rules! deps {
    () => {
        SslOpt!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl fmt :: Debug for SslOpt { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("SslOpt") . field ("no_revoke" , & (self . bits & curl_sys :: CURLSSLOPT_NO_REVOKE != 0) ,) . field ("allow_beast" , & (self . bits & curl_sys :: CURLSSLOPT_ALLOW_BEAST != 0) ,) . finish () } }
    };
}

impl_95!()