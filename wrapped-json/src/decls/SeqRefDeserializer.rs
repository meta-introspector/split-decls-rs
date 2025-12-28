macro_rules! deps {
    () => {
        Value!();
        Iter!();
    };
}

macro_rules! SeqRefDeserializer {
    () => {
        deps!();
        struct SeqRefDeserializer < 'de > { iter : slice :: Iter < 'de , Value > , }
    };
}

SeqRefDeserializer!()