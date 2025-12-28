macro_rules! deps {
    () => {
        Auth!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl fmt :: Debug for Auth { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let bits = self . bits as c_ulong ; f . debug_struct ("Auth") . field ("basic" , & (bits & curl_sys :: CURLAUTH_BASIC != 0)) . field ("digest" , & (bits & curl_sys :: CURLAUTH_DIGEST != 0)) . field ("digest_ie" , & (bits & curl_sys :: CURLAUTH_DIGEST_IE != 0)) . field ("gssnegotiate" , & (bits & curl_sys :: CURLAUTH_GSSNEGOTIATE != 0) ,) . field ("ntlm" , & (bits & curl_sys :: CURLAUTH_NTLM != 0)) . field ("ntlm_wb" , & (bits & curl_sys :: CURLAUTH_NTLM_WB != 0)) . field ("aws_sigv4" , & (bits & curl_sys :: CURLAUTH_AWS_SIGV4 != 0)) . finish () } }
    };
}

impl_93!()