macro_rules! deps {
    () => {
        Internable!();
        Interned!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T : Internable > Interned < T > { # [inline] pub fn new (obj : T) -> Self { Self :: new_generic (obj) } }
    };
}

impl_28!()