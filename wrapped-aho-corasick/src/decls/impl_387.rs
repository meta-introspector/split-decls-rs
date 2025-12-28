macro_rules! deps {
    () => {
        RareByteOffset!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl Default for RareByteOffset { fn default () -> RareByteOffset { RareByteOffset { max : 0 } } }
    };
}

impl_387!()