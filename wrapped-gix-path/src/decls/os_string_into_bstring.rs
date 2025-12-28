macro_rules! deps {
    () => {
        Utf8Error!();
    };
}

macro_rules! os_string_into_bstring {
    () => {
        deps!();
        # [doc = " Like [`into_bstr()`], but takes `OsString` as input for a lossless, but fallible, conversion."] pub fn os_string_into_bstring (path : OsString) -> Result < BString , Utf8Error > { let path = try_into_bstr (Cow :: Owned (path . into ())) ? ; match path { Cow :: Borrowed (_path) => unreachable ! ("borrowed cows stay borrowed") , Cow :: Owned (path) => Ok (path) , } }
    };
}

os_string_into_bstring!();