macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < T : ? Sized + AsRef < str > > From < & T > for Utf8PathBuf { fn from (s : & T) -> Utf8PathBuf { Utf8PathBuf :: from (s . as_ref () . to_owned ()) } }
    };
}

impl_77!()