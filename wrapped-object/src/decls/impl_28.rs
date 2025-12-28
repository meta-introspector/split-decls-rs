macro_rules! deps {
    () => {
        LittleEndian!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Default for LittleEndian { # [inline] fn default () -> LittleEndian { LittleEndian } }
    };
}

impl_28!();