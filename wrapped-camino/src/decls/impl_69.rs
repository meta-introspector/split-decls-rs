macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl From < Box < Utf8Path > > for Utf8PathBuf { fn from (path : Box < Utf8Path >) -> Utf8PathBuf { path . into_path_buf () } }
    };
}

impl_69!()