macro_rules! deps {
    () => {
        RunTimeEndian!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl Default for RunTimeEndian { # [cfg (target_endian = "little")] # [inline] fn default () -> RunTimeEndian { RunTimeEndian :: Little } # [cfg (target_endian = "big")] # [inline] fn default () -> RunTimeEndian { RunTimeEndian :: Big } }
    };
}

impl_102!();