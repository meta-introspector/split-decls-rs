// Generated macro for check_null_bytes (function)
macro_rules! Depcrate_utilcheck_null_bytes {
() => {
// Module: crate::util
// Provides: {"check_null_bytes"}
// Dependencies: {}
# [doc = " Verify that the input has no interior nulls and check whether the last byte is a null."] # [doc = ""] # [doc = " If any `b'\\0'` at positions other than the last byte are found, an error is returned. Otherwise"] # [doc = " `true` will be returned only if the last byte is `b'\\0'`."] pub (crate) fn check_null_bytes (data : & [u8]) -> Result < bool , Error > { if let [rest @ .. , last] = data { if rest . contains (& 0) { Err (Error :: InteriorZeroElements) } else { Ok (* last == 0) } } else { Ok (false) } }
};
}
