macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl From < Utf8PathBuf > for OsString { fn from (path : Utf8PathBuf) -> OsString { path . into_os_string () } }
    };
}

impl_90!()