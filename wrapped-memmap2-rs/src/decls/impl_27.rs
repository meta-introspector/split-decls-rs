macro_rules! deps {
    () => {
        MmapRaw!();
        Mmap!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl From < Mmap > for MmapRaw { fn from (value : Mmap) -> Self { Self { inner : value . inner } } }
    };
}

impl_27!();