macro_rules! deps {
    () => {
        Utf8Error!();
    };
}

macro_rules! os_str_into_bstr {
    () => {
        deps!();
        # [doc = " Like [`into_bstr()`], but takes `OsStr` as input for a lossless, but fallible, conversion."] pub fn os_str_into_bstr (path : & OsStr) -> Result < & BStr , Utf8Error > { let path = try_into_bstr (Cow :: Borrowed (path . as_ref ())) ? ; match path { Cow :: Borrowed (path) => Ok (path) , Cow :: Owned (_) => unreachable ! ("borrowed cows stay borrowed") , } }
    };
}

os_str_into_bstr!()