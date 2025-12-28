macro_rules! deps {
    () => {
        CowMut!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < T > std :: ops :: DerefMut for CowMut < '_ , T > { fn deref_mut (& mut self) -> & mut T { match self { CowMut :: Owned (it) => it , CowMut :: Borrowed (it) => it , } } }
    };
}

impl_117!()