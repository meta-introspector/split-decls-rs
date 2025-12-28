macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl FromIterator < u8 > for Bytes { fn from_iter < T : IntoIterator < Item = u8 > > (into_iter : T) -> Self { Vec :: from_iter (into_iter) . into () } }
    };
}

impl_79!()