macro_rules! deps {
    () => {
        Utf8Error!();
    };
}

macro_rules! try_os_str_into_bstr {
    () => {
        deps!();
        # [doc = " Like [`into_bstr()`], but takes `Cow<OsStr>` as input for a lossless, but fallible, conversion."] pub fn try_os_str_into_bstr (path : Cow < '_ , OsStr >) -> Result < Cow < '_ , BStr > , Utf8Error > { match path { Cow :: Borrowed (path) => os_str_into_bstr (path) . map (Cow :: Borrowed) , Cow :: Owned (path) => os_string_into_bstring (path) . map (Cow :: Owned) , } }
    };
}

try_os_str_into_bstr!();