macro_rules! deps {
    () => {
        SliceCopyIter!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'a , T > Copy for SliceCopyIter < 'a , T > { }
    };
}

impl_39!();