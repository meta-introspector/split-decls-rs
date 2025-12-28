macro_rules! deps {
    () => {
        RangeDecoder!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < R > RangeDecoder < R > { pub (crate) fn into_inner (self) -> R { self . inner } pub (crate) fn inner (& self) -> & R { & self . inner } pub (crate) fn inner_mut (& mut self) -> & mut R { & mut self . inner } }
    };
}

impl_92!();