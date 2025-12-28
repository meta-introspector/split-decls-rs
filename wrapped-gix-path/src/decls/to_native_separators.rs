macro_rules! to_native_separators {
    () => {
        # [doc = " Assures the given bytes use the native path separator."] pub fn to_native_separators < 'a > (path : impl Into < Cow < 'a , BStr > >) -> Cow < 'a , BStr > { # [cfg (not (windows))] let p = to_unix_separators (path) ; # [cfg (windows)] let p = to_windows_separators (path) ; p }
    };
}

to_native_separators!()