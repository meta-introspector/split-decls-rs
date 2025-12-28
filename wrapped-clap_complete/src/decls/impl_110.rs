macro_rules! deps {
    () => {
        ArgValueCandidates!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl ArgExt for ArgValueCandidates { }
    };
}

impl_110!();