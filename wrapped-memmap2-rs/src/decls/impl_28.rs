macro_rules! deps {
    () => {
        MmapRaw!();
        MmapMut!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl From < MmapMut > for MmapRaw { fn from (value : MmapMut) -> Self { Self { inner : value . inner } } }
    };
}

impl_28!();