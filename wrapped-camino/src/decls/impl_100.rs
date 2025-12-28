macro_rules! deps {
    () => {
        FromOsStrError!();
        Utf8PathBuf!();
        FromOsStringError!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl TryFrom < OsString > for Utf8PathBuf { type Error = FromOsStringError ; fn try_from (os_string : OsString) -> Result < Utf8PathBuf , Self :: Error > { Utf8PathBuf :: from_os_string (os_string) . map_err (| os_string | FromOsStringError { os_string , error : FromOsStrError (()) , }) } }
    };
}

impl_100!()