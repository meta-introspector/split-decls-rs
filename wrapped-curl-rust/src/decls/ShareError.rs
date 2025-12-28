macro_rules! ShareError {
    () => {
        # [doc = " An error returned from \"share\" operations."] # [doc = ""] # [doc = " This structure wraps a `CURLSHcode`."] # [derive (Clone , PartialEq)] pub struct ShareError { code : curl_sys :: CURLSHcode , }
    };
}

ShareError!()