macro_rules! deps {
    () => {
        IntoIter!();
        Value!();
        Map!();
    };
}

macro_rules! MapDeserializer {
    () => {
        deps!();
        struct MapDeserializer { iter : < Map < String , Value > as IntoIterator > :: IntoIter , value : Option < Value > , }
    };
}

MapDeserializer!()