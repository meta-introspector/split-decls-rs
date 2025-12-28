macro_rules! deps {
    () => {
        List!();
    };
}

macro_rules! raw {
    () => {
        deps!();
        pub fn raw (list : & List) -> * mut curl_sys :: curl_slist { list . raw }
    };
}

raw!();