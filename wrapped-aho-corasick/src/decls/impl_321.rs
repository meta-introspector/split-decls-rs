macro_rules! deps {
    () => {
        ByteClassSet!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl Default for ByteClassSet { fn default () -> ByteClassSet { ByteClassSet :: empty () } }
    };
}

impl_321!()