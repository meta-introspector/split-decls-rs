macro_rules! deps {
    () => {
        RawIntoIter!();
        RawIter!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < T , A : Allocator > RawIntoIter < T , A > { # [cfg_attr (feature = "inline-more" , inline)] pub fn iter (& self) -> RawIter < T > { self . iter . clone () } }
    };
}

impl_88!()