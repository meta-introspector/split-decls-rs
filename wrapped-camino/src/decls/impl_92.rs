macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl From < Utf8PathBuf > for Arc < Utf8Path > { fn from (path : Utf8PathBuf) -> Arc < Utf8Path > { let arc : Arc < Path > = Arc :: from (path . 0) ; let ptr = Arc :: into_raw (arc) as * const Utf8Path ; unsafe { Arc :: from_raw (ptr) } } }
    };
}

impl_92!();