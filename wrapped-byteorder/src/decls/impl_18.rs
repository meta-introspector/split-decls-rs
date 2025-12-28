macro_rules! deps {
    () => {
        LittleEndian!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Default for LittleEndian { fn default () -> LittleEndian { panic ! ("LittleEndian default") } }
    };
}

impl_18!();