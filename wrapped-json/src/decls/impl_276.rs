macro_rules! deps {
    () => {
        Value!();
        SeqRefDeserializer!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < 'de > SeqRefDeserializer < 'de > { fn new (slice : & 'de [Value]) -> Self { SeqRefDeserializer { iter : slice . iter () } } }
    };
}

impl_276!()