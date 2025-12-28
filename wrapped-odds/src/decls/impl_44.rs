macro_rules! deps {
    () => {
        SliceCopyIter!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'a , T > ExactSizeIterator for SliceCopyIter < 'a , T > where T : Copy { }
    };
}

impl_44!()