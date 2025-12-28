macro_rules! deps {
    () => {
        MapWith!();
    };
}

macro_rules! impl_690 {
    () => {
        deps!();
        impl < I , T , F > MapWith < I , T , F > { # [doc = " Creates a new `MapWith` iterator."] pub (super) fn new (base : I , item : T , map_op : F) -> Self { MapWith { base , item , map_op } } }
    };
}

impl_690!()