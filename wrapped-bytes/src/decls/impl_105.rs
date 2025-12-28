macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > PartialOrd < & 'a T > for Bytes where Bytes : PartialOrd < T > , { fn partial_cmp (& self , other : & & 'a T) -> Option < cmp :: Ordering > { self . partial_cmp (& * * other) } }
    };
}

impl_105!();