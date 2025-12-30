// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error returned from various \"easy\" operations."] # [doc = ""] # [doc = " This structure wraps a `CURLcode`."] # [derive (Clone , PartialEq)] pub struct Error { code : curl_sys :: CURLcode , extra : Option < Box < str > > , }
};
}
