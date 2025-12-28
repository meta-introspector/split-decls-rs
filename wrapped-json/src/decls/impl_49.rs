macro_rules! deps {
    () => {
        Read!();
        StreamDeserializer!();
        Fused!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'de , R , T > FusedIterator for StreamDeserializer < 'de , R , T > where R : Read < 'de > + Fused , T : de :: Deserialize < 'de > , { }
    };
}

impl_49!()