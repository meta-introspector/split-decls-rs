macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl From < Utf8PathBuf > for String { fn from (path : Utf8PathBuf) -> String { path . into_string () } }
    };
}

impl_89!()