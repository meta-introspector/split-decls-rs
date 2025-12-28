macro_rules! deps {
    () => {
        Map!();
        IntoIter!();
        Value!();
    };
}

macro_rules! MapDeserializer {
    () => {
        deps!();
        struct MapDeserializer { iter : < Map < String , Value > as IntoIterator > :: IntoIter , value : Option < Value > , }
    };
}

MapDeserializer!();