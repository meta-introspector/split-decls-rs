macro_rules! deps {
    () => {
        IterateBounds!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl IterateBounds for std :: ops :: RangeFull { fn into_bounds (self) -> (Option < Vec < u8 > > , Option < Vec < u8 > >) { (None , None) } }
    };
}

impl_262!();