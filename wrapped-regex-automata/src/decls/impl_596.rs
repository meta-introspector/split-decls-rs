macro_rules! deps {
    () => {
        ByteClassSet!();
    };
}

macro_rules! impl_596 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl Default for ByteClassSet { fn default () -> ByteClassSet { ByteClassSet :: empty () } }
    };
}

impl_596!();