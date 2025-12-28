macro_rules! deps {
    () => {
        IsNormalizedSinkStr!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < 'a > IsNormalizedSinkStr < 'a > { pub fn new (slice : & 'a str) -> Self { IsNormalizedSinkStr { expect : slice } } pub fn remaining_len (& self) -> usize { self . expect . len () } }
    };
}

impl_135!()