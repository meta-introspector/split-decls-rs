macro_rules! deps {
    () => {
        Strategy!();
        Guard!();
        RefCnt!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < T : RefCnt , S : Strategy < T > > Deref for Guard < T , S > { type Target = T ; # [inline] fn deref (& self) -> & T { self . inner . borrow () } }
    };
}

impl_141!()