macro_rules! deps {
    () => {
        BorrowCompat!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < T > core :: fmt :: Display for BorrowCompat < T > where T : core :: fmt :: Display , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_194!()