macro_rules! macro_294 {
    () => {
        ffi_fn ! { # [doc = " Free an HTTP response."] # [doc = ""] # [doc = " This should be used for any response once it is no longer needed."] fn hyper_response_free (resp : * mut hyper_response) { drop (non_null ! (Box :: from_raw (resp) ?= ())) ; } }
    };
}

macro_294!()