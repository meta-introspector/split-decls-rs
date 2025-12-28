macro_rules! deps {
    () => {
        StopWatchSpan!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl fmt :: Display for StopWatchSpan { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:.2?}" , self . time) ? ; if let Some (mut instructions) = self . instructions { let mut prefix = "" ; if instructions > 10000 { instructions /= 1000 ; prefix = "k" ; } if instructions > 10000 { instructions /= 1000 ; prefix = "m" ; } if instructions > 10000 { instructions /= 1000 ; prefix = "g" ; } write ! (f , ", {instructions}{prefix}instr") ? ; } write ! (f , ", {}" , self . memory) ? ; Ok (()) } }
    };
}

impl_23!();