macro_rules! deps {
    () => {
        MergeOperands!();
        MergeFn!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < F > MergeFn for F where F : Fn (& [u8] , Option < & [u8] > , & MergeOperands) -> Option < Vec < u8 > > + Send + Sync + 'static { }
    };
}

impl_272!();