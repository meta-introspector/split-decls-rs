macro_rules! deps {
    () => {
        Idx!();
        IdxRange!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for IdxRange < T > { fn next_back (& mut self) -> Option < Self :: Item > { self . range . next_back () . map (| raw | Idx :: from_raw (raw . into ())) } }
    };
}

impl_20!()