macro_rules! deps {
    () => {
        Value!();
        Map!();
        IntoIter!();
    };
}

macro_rules! MapRefDeserializer {
    () => {
        deps!();
        struct MapRefDeserializer < 'de > { iter : < & 'de Map < String , Value > as IntoIterator > :: IntoIter , value : Option < & 'de Value > , }
    };
}

MapRefDeserializer!();