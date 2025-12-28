macro_rules! deps {
    () => {
        DebugByte!();
        Transition!();
    };
}

macro_rules! impl_499 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Transition { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{:?} => {}" , crate :: util :: escape :: DebugByte (self . byte) , self . next . as_usize ()) } }
    };
}

impl_499!()