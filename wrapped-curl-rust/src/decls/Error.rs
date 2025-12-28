macro_rules! Error {
    () => {
        # [doc = " An error returned from various \"easy\" operations."] # [doc = ""] # [doc = " This structure wraps a `CURLcode`."] # [derive (Clone , PartialEq)] pub struct Error { code : curl_sys :: CURLcode , extra : Option < Box < str > > , }
    };
}

Error!()