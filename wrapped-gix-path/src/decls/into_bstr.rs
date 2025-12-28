macro_rules! into_bstr {
    () => {
        # [doc = " Similar to [`try_into_bstr()`] but **panics** if malformed surrogates are encountered on Windows."] pub fn into_bstr < 'a > (path : impl Into < Cow < 'a , Path > >) -> Cow < 'a , BStr > { try_into_bstr (path) . expect ("prefix path doesn't contain ill-formed UTF-8") }
    };
}

into_bstr!();