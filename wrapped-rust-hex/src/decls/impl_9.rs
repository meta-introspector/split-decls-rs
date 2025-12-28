macro_rules! deps {
    () => {
        BytesToHexChars!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl ExactSizeIterator for BytesToHexChars < '_ > { fn len (& self) -> usize { let mut length = self . inner . len () * 2 ; if self . next . is_some () { length += 1 ; } length } }
    };
}

impl_9!()