macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! SliceDrain {
    () => {
        deps!();
        pub (crate) struct SliceDrain < 'data , T > { iter : slice :: IterMut < 'data , T > , }
    };
}

SliceDrain!();