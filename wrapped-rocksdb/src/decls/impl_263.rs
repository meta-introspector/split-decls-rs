macro_rules! deps {
    () => {
        IterateBounds!();
        Range!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl < K : Into < Vec < u8 > > > IterateBounds for std :: ops :: Range < K > { fn into_bounds (self) -> (Option < Vec < u8 > > , Option < Vec < u8 > >) { (Some (self . start . into ()) , Some (self . end . into ())) } }
    };
}

impl_263!()