macro_rules! deps {
    () => {
        Internable!();
    };
}

macro_rules! Interned {
    () => {
        deps!();
        pub struct Interned < T : Internable + ? Sized > { arc : Arc < T > , }
    };
}

Interned!();