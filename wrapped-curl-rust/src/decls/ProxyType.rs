macro_rules! ProxyType {
    () => {
        # [doc = " Possible proxy types that libcurl currently understands."] # [non_exhaustive] # [allow (missing_docs)] # [derive (Debug , Clone , Copy)] pub enum ProxyType { Http = curl_sys :: CURLPROXY_HTTP as isize , Http1 = curl_sys :: CURLPROXY_HTTP_1_0 as isize , Socks4 = curl_sys :: CURLPROXY_SOCKS4 as isize , Socks5 = curl_sys :: CURLPROXY_SOCKS5 as isize , Socks4a = curl_sys :: CURLPROXY_SOCKS4A as isize , Socks5Hostname = curl_sys :: CURLPROXY_SOCKS5_HOSTNAME as isize , }
    };
}

ProxyType!()