macro_rules! deps {
    () => {
        LinkedHashMap!();
        DefaultHashBuilder!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < K , V > LinkedHashMap < K , V > { # [inline] pub fn new () -> Self { Self { hash_builder : DefaultHashBuilder :: default () , table : HashTable :: new () , values : None , free : None , } } # [inline] pub fn with_capacity (capacity : usize) -> Self { Self { hash_builder : DefaultHashBuilder :: default () , table : HashTable :: with_capacity (capacity) , values : None , free : None , } } }
    };
}

impl_3!();