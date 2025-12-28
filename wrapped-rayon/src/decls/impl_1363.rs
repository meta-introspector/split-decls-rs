macro_rules! deps {
    () => {
        SliceDrain!();
    };
}

macro_rules! impl_1363 {
    () => {
        deps!();
        impl < 'data , T : 'data > ExactSizeIterator for SliceDrain < 'data , T > { fn len (& self) -> usize { self . iter . len () } }
    };
}

impl_1363!();