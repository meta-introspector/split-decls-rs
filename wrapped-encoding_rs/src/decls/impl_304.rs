macro_rules! deps {
    () => {
        BigEndian!();
        Endian!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl Endian for BigEndian { # [cfg (target_endian = "little")] const OPPOSITE_ENDIAN : bool = true ; # [cfg (target_endian = "big")] const OPPOSITE_ENDIAN : bool = false ; }
    };
}

impl_304!();