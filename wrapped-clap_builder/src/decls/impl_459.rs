macro_rules! deps {
    () => {
        RawValues!();
        OsStr!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl < 'a > Iterator for RawValues < 'a > { type Item = & 'a OsStr ; fn next (& mut self) -> Option < & 'a OsStr > { if let Some (next) = self . iter . next () { self . len -= 1 ; Some (next) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
    };
}

impl_459!()