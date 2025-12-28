macro_rules! deps {
    () => {
        MmapRaw!();
        MmapMut!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl From < MmapMut > for MmapRaw { fn from (value : MmapMut) -> Self { Self { inner : value . inner } } }
    };
}

impl_23!()