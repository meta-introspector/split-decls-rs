macro_rules! deps {
    () => {
        MergeOperands!();
    };
}

macro_rules! MergeFn {
    () => {
        deps!();
        pub trait MergeFn : Fn (& [u8] , Option < & [u8] > , & MergeOperands) -> Option < Vec < u8 > > + Send + Sync + 'static { }
    };
}

MergeFn!();