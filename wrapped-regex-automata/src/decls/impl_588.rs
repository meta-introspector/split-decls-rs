macro_rules! deps {
    () => {
        Unit!();
        ByteClassIter!();
    };
}

macro_rules! impl_588 {
    () => {
        deps!();
        impl < 'a > Iterator for ByteClassIter < 'a > { type Item = Unit ; fn next (& mut self) -> Option < Unit > { if self . i + 1 == self . classes . alphabet_len () { self . i += 1 ; Some (self . classes . eoi ()) } else if self . i < self . classes . alphabet_len () { let class = u8 :: try_from (self . i) . unwrap () ; self . i += 1 ; Some (Unit :: u8 (class)) } else { None } } }
    };
}

impl_588!();