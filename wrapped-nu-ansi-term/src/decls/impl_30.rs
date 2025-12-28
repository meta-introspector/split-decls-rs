macro_rules! deps {
    () => {
        OSControl!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < 'a , S : 'a + ToOwned + ? Sized > Clone for OSControl < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug , { fn clone (& self) -> Self { match self { Self :: Link { url : u } => Self :: Link { url : u . clone () } , Self :: Title => Self :: Title , } } }
    };
}

impl_30!();