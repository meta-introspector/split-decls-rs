// Generated macro for cvt (function)
macro_rules! Depcrate_multicvt {
() => {
// Module: crate::multi
// Provides: {"cvt"}
// Dependencies: {}
fn cvt (code : curl_sys :: CURLMcode) -> Result < () , MultiError > { if code == curl_sys :: CURLM_OK { Ok (()) } else { Err (MultiError :: new (code)) } }
};
}
