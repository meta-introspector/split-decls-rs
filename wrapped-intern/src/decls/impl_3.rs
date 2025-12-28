macro_rules! deps {
    () => {
        Interned!();
        Internable!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < T : Internable > Interned < T > { # [inline] pub fn new (obj : T) -> Self { Self :: new_generic (obj) } }
    };
}

impl_3!()