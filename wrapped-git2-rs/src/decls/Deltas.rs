macro_rules! deps {
    () => {
        Diff!();
    };
}

macro_rules! Deltas {
    () => {
        deps!();
        # [doc = " An iterator over the diffs in a delta"] pub struct Deltas < 'diff > { range : Range < usize > , diff : & 'diff Diff < 'diff > , }
    };
}

Deltas!();