macro_rules! deps {
    () => {
        ByteClassElements!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < 'a > Iterator for ByteClassElements < 'a > { type Item = u8 ; fn next (& mut self) -> Option < u8 > { while let Some (byte) = self . bytes . next () { if self . class == self . classes . get (byte) { return Some (byte) ; } } None } }
    };
}

impl_317!();