macro_rules! deps {
    () => {
        ApproximateByteSet!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl ApproximateByteSet { # [doc = " Create a new set from the given needle."] fn new (needle : & [u8]) -> ApproximateByteSet { let mut bits = 0 ; for & b in needle { bits |= 1 << (b % 64) ; } ApproximateByteSet (bits) } # [doc = " Return true if and only if the given byte might be in this set. This"] # [doc = " may return a false positive, but will never return a false negative."] # [inline (always)] fn contains (& self , byte : u8) -> bool { self . 0 & (1 << (byte % 64)) != 0 } }
    };
}

impl_117!()