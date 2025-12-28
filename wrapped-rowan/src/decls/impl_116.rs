macro_rules! deps {
    () => {
        CowMut!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < T > std :: ops :: Deref for CowMut < '_ , T > { type Target = T ; fn deref (& self) -> & T { match self { CowMut :: Owned (it) => it , CowMut :: Borrowed (it) => it , } } }
    };
}

impl_116!()