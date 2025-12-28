macro_rules! deps {
    () => {
        OwnedOrStaticAtomicBool!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl Deref for OwnedOrStaticAtomicBool { type Target = std :: sync :: atomic :: AtomicBool ; fn deref (& self) -> & Self :: Target { match self { OwnedOrStaticAtomicBool :: Owned { flag , .. } => flag , OwnedOrStaticAtomicBool :: Shared (flag) => flag , } } }
    };
}

impl_432!()