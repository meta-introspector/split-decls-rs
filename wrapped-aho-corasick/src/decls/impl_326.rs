macro_rules! deps {
    () => {
        ByteSet!();
        BitSet!();
    };
}

macro_rules! impl_326 {
    () => {
        deps!();
        impl core :: fmt :: Debug for BitSet { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let mut fmtd = f . debug_set () ; for b in 0u8 ..= 255 { if (ByteSet { bits : * self }) . contains (b) { fmtd . entry (& b) ; } } fmtd . finish () } }
    };
}

impl_326!();