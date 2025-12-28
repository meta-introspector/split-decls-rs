macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Span { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{}..{}" , self . start , self . end) } }
    };
}

impl_447!()