macro_rules! deps {
    () => {
        Handler!();
        Inner!();
    };
}

macro_rules! opensocket_cb {
    () => {
        deps!();
        extern "C" fn opensocket_cb < H : Handler > (data : * mut c_void , _purpose : curl_sys :: curlsocktype , address : * mut curl_sys :: curl_sockaddr ,) -> curl_sys :: curl_socket_t { let res = panic :: catch (| | unsafe { (* (data as * mut Inner < H >)) . handler . open_socket ((* address) . family , (* address) . socktype , (* address) . protocol) . unwrap_or (curl_sys :: CURL_SOCKET_BAD) }) ; res . unwrap_or (curl_sys :: CURL_SOCKET_BAD) }
    };
}

opensocket_cb!()