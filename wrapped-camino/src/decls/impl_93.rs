macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl From < Utf8PathBuf > for Rc < Utf8Path > { fn from (path : Utf8PathBuf) -> Rc < Utf8Path > { let rc : Rc < Path > = Rc :: from (path . 0) ; let ptr = Rc :: into_raw (rc) as * const Utf8Path ; unsafe { Rc :: from_raw (ptr) } } }
    };
}

impl_93!()