macro_rules! deps {
    () => {
        Iter!();
        Value!();
    };
}

macro_rules! SeqRefDeserializer {
    () => {
        deps!();
        struct SeqRefDeserializer < 'de > { iter : slice :: Iter < 'de , Value > , }
    };
}

SeqRefDeserializer!();