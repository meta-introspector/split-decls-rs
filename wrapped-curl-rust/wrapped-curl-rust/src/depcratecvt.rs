// Generated macro for cvt (function)
macro_rules! Depcratecvt {
() => {
// Module: crate
// Provides: {"cvt"}
// Dependencies: {}
fn cvt (r : curl_sys :: CURLcode) -> Result < () , Error > { if r == curl_sys :: CURLE_OK { Ok (()) } else { Err (Error :: new (r)) } }
};
}
