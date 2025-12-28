macro_rules! deps {
    () => {
        Inner!();
        Handler!();
    };
}

macro_rules! ssl_ctx_cb {
    () => {
        deps!();
        extern "C" fn ssl_ctx_cb < H : Handler > (_handle : * mut curl_sys :: CURL , ssl_ctx : * mut c_void , data : * mut c_void ,) -> curl_sys :: CURLcode { let res = panic :: catch (| | unsafe { match (* (data as * mut Inner < H >)) . handler . ssl_ctx (ssl_ctx) { Ok (()) => curl_sys :: CURLE_OK , Err (e) => e . code () , } }) ; res . unwrap_or (curl_sys :: CURLE_SSL_CONNECT_ERROR) }
    };
}

ssl_ctx_cb!()