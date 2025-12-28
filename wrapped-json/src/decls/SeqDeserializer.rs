macro_rules! deps {
    () => {
        Value!();
        IntoIter!();
    };
}

macro_rules! SeqDeserializer {
    () => {
        deps!();
        struct SeqDeserializer { iter : vec :: IntoIter < Value > , }
    };
}

SeqDeserializer!();