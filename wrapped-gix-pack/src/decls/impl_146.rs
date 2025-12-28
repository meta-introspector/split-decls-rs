macro_rules! deps {
    () => {
        BytesToEntriesIter!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < R > std :: iter :: ExactSizeIterator for BytesToEntriesIter < R > where R : io :: BufRead { }
    };
}

impl_146!();