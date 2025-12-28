macro_rules! deps {
    () => {
        Captures!();
        CapturesDebugMap!();
    };
}

macro_rules! impl_611 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Captures { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let mut dstruct = f . debug_struct ("Captures") ; dstruct . field ("pid" , & self . pid) ; if let Some (pid) = self . pid { dstruct . field ("spans" , & CapturesDebugMap { pid , caps : self }) ; } dstruct . finish () } }
    };
}

impl_611!()