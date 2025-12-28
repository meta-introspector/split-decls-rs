macro_rules! deps {
    () => {
        OwnedOrStaticAtomicBool!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl < 'a > From < & 'a Arc < AtomicBool > > for OwnedOrStaticAtomicBool { fn from (value : & 'a Arc < AtomicBool >) -> Self { OwnedOrStaticAtomicBool :: Owned { flag : value . clone () , private : false , } } }
    };
}

impl_434!();