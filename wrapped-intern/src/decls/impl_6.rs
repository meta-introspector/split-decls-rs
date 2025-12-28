macro_rules! deps {
    () => {
        Internable!();
        Interned!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T : Internable + ? Sized > Drop for Interned < T > { # [inline] fn drop (& mut self) { if Arc :: count (& self . arc) == 2 { self . drop_slow () ; } } }
    };
}

impl_6!()