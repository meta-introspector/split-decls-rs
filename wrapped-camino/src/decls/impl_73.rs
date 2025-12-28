macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl From < Utf8PathBuf > for OsString { fn from (path : Utf8PathBuf) -> OsString { path . into_os_string () } }
    };
}

impl_73!()