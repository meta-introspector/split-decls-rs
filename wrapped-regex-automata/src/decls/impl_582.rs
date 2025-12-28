macro_rules! deps {
    () => {
        DebugByte!();
        U8!();
        Unit!();
        UnitKind!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Unit { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { match self . 0 { UnitKind :: U8 (b) => write ! (f , "{:?}" , DebugByte (b)) , UnitKind :: EOI (_) => write ! (f , "EOI") , } } }
    };
}

impl_582!()