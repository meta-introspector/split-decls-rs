macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl From < String > for Utf8PathBuf { fn from (string : String) -> Utf8PathBuf { Utf8PathBuf (string . into ()) } }
    };
}

impl_57!()