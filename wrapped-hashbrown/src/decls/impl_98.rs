macro_rules! deps {
    () => {
        RawDrain!();
        RawIter!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < T , A : Allocator > RawDrain < '_ , T , A > { # [cfg_attr (feature = "inline-more" , inline)] pub fn iter (& self) -> RawIter < T > { self . iter . clone () } }
    };
}

impl_98!();