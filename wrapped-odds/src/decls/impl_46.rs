macro_rules! deps {
    () => {
        SliceCopyIter!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'a , T > Default for SliceCopyIter < 'a , T > where T : Copy , { # [doc = " Create an empty `SliceCopyIter`."] fn default () -> Self { unsafe { SliceCopyIter :: new (0x1 as * const T , 0x1 as * const T) } } }
    };
}

impl_46!();