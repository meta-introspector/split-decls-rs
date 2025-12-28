macro_rules! deps {
    () => {
        Fused!();
        StreamDeserializer!();
        Read!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'de , R , T > FusedIterator for StreamDeserializer < 'de , R , T > where R : Read < 'de > + Fused , T : de :: Deserialize < 'de > , { }
    };
}

impl_49!();