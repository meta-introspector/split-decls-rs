macro_rules! deps {
    () => {
        LittleEndian!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Default for LittleEndian { fn default () -> LittleEndian { panic ! ("LittleEndian default") } }
    };
}

impl_13!()