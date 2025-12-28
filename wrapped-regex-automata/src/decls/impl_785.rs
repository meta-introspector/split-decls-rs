macro_rules! deps {
    () => {
        DebugByte!();
        StartByteMap!();
    };
}

macro_rules! impl_785 {
    () => {
        deps!();
        impl core :: fmt :: Debug for StartByteMap { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: util :: escape :: DebugByte ; write ! (f , "StartByteMap{{") ? ; for byte in 0 ..= 255 { if byte > 0 { write ! (f , ", ") ? ; } let start = self . map [usize :: from (byte)] ; write ! (f , "{:?} => {:?}" , DebugByte (byte) , start) ? ; } write ! (f , "}}") ? ; Ok (()) } }
    };
}

impl_785!();