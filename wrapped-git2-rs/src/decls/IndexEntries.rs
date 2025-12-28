macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! IndexEntries {
    () => {
        deps!();
        # [doc = " An iterator over the entries in an index"] pub struct IndexEntries < 'index > { range : Range < usize > , index : & 'index Index , }
    };
}

IndexEntries!()