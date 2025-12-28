macro_rules! deps {
    () => {
        VacantEntryRef!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < K , Q , V , S , A > Debug for VacantEntryRef < '_ , '_ , K , Q , V , S , A > where K : Borrow < Q > , Q : Debug + ? Sized , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntryRef") . field (& self . key ()) . finish () } }
    };
}

impl_290!()