macro_rules! deps {
    () => {
        Blame!();
    };
}

macro_rules! BlameIter {
    () => {
        deps!();
        # [doc = " An iterator over the hunks in a blame."] pub struct BlameIter < 'blame > { range : Range < usize > , blame : & 'blame Blame < 'blame > , }
    };
}

BlameIter!()