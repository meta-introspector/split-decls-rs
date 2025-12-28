macro_rules! deps {
    () => {
        Block!();
        BlockedIter!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a , B , T > ExactSizeIterator for BlockedIter < 'a , B , T > where B : Block < Item = T > { }
    };
}

impl_32!();