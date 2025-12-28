macro_rules! deps {
    () => {
        RangeIter!();
        Reader!();
        RangeIterInner!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl < R : Reader > Default for RangeIter < R > { fn default () -> Self { RangeIter (RangeIterInner :: Single (None)) } }
    };
}

impl_279!()