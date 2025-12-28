macro_rules! deps {
    () => {
        SliceDrain!();
    };
}

macro_rules! impl_1364 {
    () => {
        deps!();
        impl < 'data , T : 'data > iter :: FusedIterator for SliceDrain < 'data , T > { }
    };
}

impl_1364!()