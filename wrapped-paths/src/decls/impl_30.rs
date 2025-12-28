macro_rules! deps {
    () => {
        RelPathBuf!();
        RelPath!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl RelPathBuf { # [doc = " Coerces to a `RelPath` slice."] # [doc = ""] # [doc = " Equivalent of [`Utf8PathBuf::as_path`] for `RelPathBuf`."] pub fn as_path (& self) -> & RelPath { RelPath :: new_unchecked (self . 0 . as_path ()) } }
    };
}

impl_30!()