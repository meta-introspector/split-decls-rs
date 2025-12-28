macro_rules! deps {
    () => {
        Slots!();
        NFA!();
    };
}

macro_rules! SlotsIter {
    () => {
        deps!();
        # [doc = " An iterator over all of the bits set in a slot set."] # [doc = ""] # [doc = " This returns the bit index that is set, so callers may need to offset it"] # [doc = " to get the actual NFA slot index."] # [derive (Debug)] struct SlotsIter { slots : Slots , }
    };
}

SlotsIter!()