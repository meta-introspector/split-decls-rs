macro_rules! MapKeyDeserializer {
    () => {
        struct MapKeyDeserializer < 'de > { key : Cow < 'de , str > , }
    };
}

MapKeyDeserializer!();