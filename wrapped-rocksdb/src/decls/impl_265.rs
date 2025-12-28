macro_rules! deps {
    () => {
        IterateBounds!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl < K : Into < Vec < u8 > > > IterateBounds for std :: ops :: RangeTo < K > { fn into_bounds (self) -> (Option < Vec < u8 > > , Option < Vec < u8 > >) { (None , Some (self . end . into ())) } }
    };
}

impl_265!()