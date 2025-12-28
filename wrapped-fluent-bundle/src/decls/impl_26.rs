macro_rules! deps {
    () => {
        EntryKind!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl std :: fmt :: Display for EntryKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Message => f . write_str ("message") , Self :: Term => f . write_str ("term") , Self :: Function => f . write_str ("function") , } } }
    };
}

impl_26!();