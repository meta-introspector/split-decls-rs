macro_rules! deps {
    () => {
        Url!();
        Error!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl TryFrom < & std :: ffi :: OsStr > for Url { type Error = parse :: Error ; fn try_from (value : & std :: ffi :: OsStr) -> Result < Self , Self :: Error > { gix_path :: os_str_into_bstr (value) . expect ("no illformed UTF-8 on Windows") . try_into () } }
    };
}

impl_18!();