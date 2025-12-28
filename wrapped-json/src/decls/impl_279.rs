macro_rules! deps {
    () => {
        Map!();
        Value!();
        MapRefDeserializer!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl < 'de > MapRefDeserializer < 'de > { fn new (map : & 'de Map < String , Value >) -> Self { MapRefDeserializer { iter : map . into_iter () , value : None , } } }
    };
}

impl_279!();