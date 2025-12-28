macro_rules! deps {
    () => {
        IsNormalizedSinkUtf8!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        # [cfg (feature = "utf8_iter")] impl < 'a > IsNormalizedSinkUtf8 < 'a > { pub fn new (slice : & 'a [u8]) -> Self { IsNormalizedSinkUtf8 { expect : slice } } pub fn remaining_len (& self) -> usize { self . expect . len () } }
    };
}

impl_132!()