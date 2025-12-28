macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl From < & '_ Utf8Path > for Rc < Utf8Path > { fn from (path : & Utf8Path) -> Rc < Utf8Path > { let rc : Rc < Path > = Rc :: from (AsRef :: < Path > :: as_ref (path)) ; let ptr = Rc :: into_raw (rc) as * const Utf8Path ; unsafe { Rc :: from_raw (ptr) } } }
    };
}

impl_63!()