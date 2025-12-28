macro_rules! deps {
    () => {
        OsStr!();
        RawValues!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for RawValues < 'a > { fn next_back (& mut self) -> Option < & 'a OsStr > { if let Some (next) = self . iter . next_back () { self . len -= 1 ; Some (next) } else { None } } }
    };
}

impl_460!();