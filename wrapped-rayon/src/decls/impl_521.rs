macro_rules! deps {
    () => {
        FlatMapIter!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        impl < I , F > FlatMapIter < I , F > { # [doc = " Creates a new `FlatMapIter` iterator."] pub (super) fn new (base : I , map_op : F) -> Self { FlatMapIter { base , map_op } } }
    };
}

impl_521!()