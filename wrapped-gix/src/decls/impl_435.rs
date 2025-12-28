macro_rules! deps {
    () => {
        OwnedOrStaticAtomicBool!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl From < Arc < AtomicBool > > for OwnedOrStaticAtomicBool { fn from (flag : Arc < AtomicBool >) -> Self { OwnedOrStaticAtomicBool :: Owned { flag , private : false } } }
    };
}

impl_435!()