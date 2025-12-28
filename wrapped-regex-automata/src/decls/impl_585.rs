macro_rules! deps {
    () => {
        ByteClasses!();
    };
}

macro_rules! impl_585 {
    () => {
        deps!();
        impl Default for ByteClasses { fn default () -> ByteClasses { ByteClasses :: singletons () } }
    };
}

impl_585!()