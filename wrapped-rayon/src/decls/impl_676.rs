macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! impl_676 {
    () => {
        deps!();
        impl < I , F > Map < I , F > { # [doc = " Creates a new `Map` iterator."] pub (super) fn new (base : I , map_op : F) -> Self { Map { base , map_op } } }
    };
}

impl_676!();