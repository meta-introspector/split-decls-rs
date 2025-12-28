macro_rules! deps {
    () => {
        Default!();
        OwnedOrStaticAtomicBool!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl Default for OwnedOrStaticAtomicBool { fn default () -> Self { OwnedOrStaticAtomicBool :: Owned { flag : Arc :: new (AtomicBool :: default ()) , private : true , } } }
    };
}

impl_431!()