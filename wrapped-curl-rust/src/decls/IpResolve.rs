macro_rules! IpResolve {
    () => {
        # [doc = " Possible values to pass to the `ip_resolve` method."] # [non_exhaustive] # [allow (missing_docs)] # [derive (Debug , Clone , Copy)] pub enum IpResolve { V4 = curl_sys :: CURL_IPRESOLVE_V4 as isize , V6 = curl_sys :: CURL_IPRESOLVE_V6 as isize , Any = curl_sys :: CURL_IPRESOLVE_WHATEVER as isize , }
    };
}

IpResolve!()