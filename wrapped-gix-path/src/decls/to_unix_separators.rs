macro_rules! to_unix_separators {
    () => {
        # [doc = " Replace Windows path separators with slashes, which typically resembles a Unix path, unconditionally."] # [doc = ""] # [doc = " **Note** Do not use these and prefer the conditional versions of this method."] pub fn to_unix_separators < 'a > (path : impl Into < Cow < 'a , BStr > >) -> Cow < 'a , BStr > { replace (path , b'\\' , b'/') }
    };
}

to_unix_separators!()