macro_rules! deps {
    () => {
        SeqHandle!();
    };
}

macro_rules! SeqInner {
    () => {
        deps!();
        # [derive (Default)] struct SeqInner { # [doc = " Should match the `seq` field of the next [`SeqHandle`] that has not been"] # [doc = " fully satisfied."] satisfaction_level : AtomicUsize , }
    };
}

SeqInner!();