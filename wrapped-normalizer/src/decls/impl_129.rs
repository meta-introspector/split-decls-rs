macro_rules! deps {
    () => {
        IsNormalizedSinkUtf16!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        # [cfg (feature = "utf16_iter")] impl < 'a > IsNormalizedSinkUtf16 < 'a > { pub fn new (slice : & 'a [u16]) -> Self { IsNormalizedSinkUtf16 { expect : slice } } pub fn remaining_len (& self) -> usize { self . expect . len () } }
    };
}

impl_129!();