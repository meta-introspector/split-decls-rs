macro_rules! deps {
    () => {
        ShareError!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl ShareError { # [doc = " Creates a new error from the underlying code returned by libcurl."] pub fn new (code : curl_sys :: CURLSHcode) -> ShareError { ShareError { code } } # [doc = " Returns whether this error corresponds to CURLSHE_BAD_OPTION."] pub fn is_bad_option (& self) -> bool { self . code == curl_sys :: CURLSHE_BAD_OPTION } # [doc = " Returns whether this error corresponds to CURLSHE_IN_USE."] pub fn is_in_use (& self) -> bool { self . code == curl_sys :: CURLSHE_IN_USE } # [doc = " Returns whether this error corresponds to CURLSHE_INVALID."] pub fn is_invalid (& self) -> bool { self . code == curl_sys :: CURLSHE_INVALID } # [doc = " Returns whether this error corresponds to CURLSHE_NOMEM."] pub fn is_nomem (& self) -> bool { self . code == curl_sys :: CURLSHE_NOMEM } # [doc = " Returns the value of the underlying error corresponding to libcurl."] pub fn code (& self) -> curl_sys :: CURLSHcode { self . code } # [doc = " Returns curl's human-readable version of this error."] pub fn description (& self) -> & str { unsafe { let s = curl_sys :: curl_share_strerror (self . code) ; assert ! (! s . is_null ()) ; str :: from_utf8 (CStr :: from_ptr (s) . to_bytes ()) . unwrap () } } }
    };
}

impl_6!();