macro_rules! to_unix_separators_on_windows {
    () => {
        # [doc = " Replace Windows path separators with slashes, but only do so on Windows."] pub fn to_unix_separators_on_windows < 'a > (path : impl Into < Cow < 'a , BStr > >) -> Cow < 'a , BStr > { # [cfg (windows)] { to_unix_separators (path) } # [cfg (not (windows))] { path . into () } }
    };
}

to_unix_separators_on_windows!();