macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T : ? Sized + AsRef < str > > From < & T > for Box < Utf8Path > { fn from (s : & T) -> Box < Utf8Path > { Utf8PathBuf :: from (s) . into_boxed_path () } }
    };
}

impl_61!()