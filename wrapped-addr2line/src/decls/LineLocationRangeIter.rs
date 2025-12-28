macro_rules! deps {
    () => {
        Lines!();
    };
}

macro_rules! LineLocationRangeIter {
    () => {
        deps!();
        pub (crate) struct LineLocationRangeIter < 'ctx > { lines : & 'ctx Lines , seq_idx : usize , row_idx : usize , probe_high : u64 , }
    };
}

LineLocationRangeIter!();