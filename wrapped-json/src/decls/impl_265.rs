macro_rules! deps {
    () => {
        MapDeserializer!();
        Map!();
        Value!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl MapDeserializer { fn new (map : Map < String , Value >) -> Self { MapDeserializer { iter : map . into_iter () , value : None , } } }
    };
}

impl_265!();