macro_rules! deps {
    () => {
        Endianness!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Default for Endianness { # [cfg (target_endian = "little")] # [inline] fn default () -> Endianness { Endianness :: Little } # [cfg (target_endian = "big")] # [inline] fn default () -> Endianness { Endianness :: Big } }
    };
}

impl_25!();