macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl From < & '_ Utf8Path > for Arc < Utf8Path > { fn from (path : & Utf8Path) -> Arc < Utf8Path > { let arc : Arc < Path > = Arc :: from (AsRef :: < Path > :: as_ref (path)) ; let ptr = Arc :: into_raw (arc) as * const Utf8Path ; unsafe { Arc :: from_raw (ptr) } } }
    };
}

impl_62!()