macro_rules! deps {
    () => {
        HistoryBufInner!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < T , S : HistoryBufStorage < T > + ? Sized > fmt :: Debug for HistoryBufInner < T , S > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < [T] as fmt :: Debug > :: fmt (self , f) } }
    };
}

impl_70!();