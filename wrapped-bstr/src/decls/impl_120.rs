macro_rules! deps {
    () => {
        DrainBytes!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < 'a > iter :: FusedIterator for DrainBytes < 'a > { }
    };
}

impl_120!()