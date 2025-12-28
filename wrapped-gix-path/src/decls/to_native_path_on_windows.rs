macro_rules! to_native_path_on_windows {
    () => {
        # [doc = " Convert paths with slashes to backslashes on Windows and do nothing on Unix,"] # [doc = " but **panic** if unpaired surrogates are encountered on Windows."] pub fn to_native_path_on_windows < 'a > (path : impl Into < Cow < 'a , BStr > >) -> Cow < 'a , std :: path :: Path > { # [cfg (not (windows))] { crate :: from_bstr (path) } # [cfg (windows)] { crate :: from_bstr (to_windows_separators (path)) } }
    };
}

to_native_path_on_windows!();