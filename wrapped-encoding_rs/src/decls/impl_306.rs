macro_rules! deps {
    () => {
        Endian!();
        LittleEndian!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl Endian for LittleEndian { # [cfg (target_endian = "little")] const OPPOSITE_ENDIAN : bool = false ; # [cfg (target_endian = "big")] const OPPOSITE_ENDIAN : bool = true ; }
    };
}

impl_306!()