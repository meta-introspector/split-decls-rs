macro_rules! deps {
    () => {
        Epoch!();
    };
}

macro_rules! AtomicEpoch {
    () => {
        deps!();
        # [doc = " An atomic value that holds an `Epoch`."] # [derive (Default , Debug)] pub (crate) struct AtomicEpoch { # [doc = " Since `Epoch` is just a wrapper around `usize`, an `AtomicEpoch` is similarly represented"] # [doc = " using an `AtomicUsize`."] data : AtomicUsize , }
    };
}

AtomicEpoch!();