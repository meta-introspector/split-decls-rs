macro_rules! deps {
    () => {
        MapInit!();
    };
}

macro_rules! impl_707 {
    () => {
        deps!();
        impl < I , INIT , F > MapInit < I , INIT , F > { # [doc = " Creates a new `MapInit` iterator."] pub (super) fn new (base : I , init : INIT , map_op : F) -> Self { MapInit { base , init , map_op } } }
    };
}

impl_707!();