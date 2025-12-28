macro_rules! RawMulti {
    () => {
        # [derive (Debug)] struct RawMulti { handle : * mut curl_sys :: CURLM , }
    };
}

RawMulti!();