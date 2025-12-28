macro_rules! deps {
    () => {
        CollectedBound!();
    };
}

macro_rules! CollectedSizednessBounds {
    () => {
        deps!();
        # [derive (Debug)] struct CollectedSizednessBounds { sized : CollectedBound , meta_sized : CollectedBound , pointee_sized : CollectedBound , }
    };
}

CollectedSizednessBounds!();