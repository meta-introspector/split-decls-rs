macro_rules! MultiError {
    () => {
        # [doc = " An error from \"multi\" operations."] # [doc = ""] # [doc = " THis structure wraps a `CURLMcode`."] # [derive (Clone , PartialEq)] pub struct MultiError { code : curl_sys :: CURLMcode , }
    };
}

MultiError!();