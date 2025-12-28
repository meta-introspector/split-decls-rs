macro_rules! deps {
    () => {
        IterateBounds!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl < K : Into < Vec < u8 > > > IterateBounds for std :: ops :: RangeFrom < K > { fn into_bounds (self) -> (Option < Vec < u8 > > , Option < Vec < u8 > >) { (Some (self . start . into ()) , None) } }
    };
}

impl_264!();