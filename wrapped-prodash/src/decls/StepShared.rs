macro_rules! deps {
    () => {
        AtomicStep!();
    };
}

macro_rules! StepShared {
    () => {
        deps!();
        # [doc = " As step, but shareable."] pub type StepShared = Arc < AtomicStep > ;
    };
}

StepShared!();