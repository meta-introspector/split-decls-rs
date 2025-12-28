macro_rules! deps {
    () => {
        RawIterRange!();
        RawParIter!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < T > RawParIter < T > { # [cfg_attr (feature = "inline-more" , inline)] pub (super) unsafe fn iter (& self) -> RawIterRange < T > { self . iter . clone () } }
    };
}

impl_161!();