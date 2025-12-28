macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T : RefCnt , S : Strategy < T > > Deref for Guard < T , S > { type Target = T ; # [inline] fn deref (& self) -> & T { self . inner . borrow () } }
    };
}

impl_13!()