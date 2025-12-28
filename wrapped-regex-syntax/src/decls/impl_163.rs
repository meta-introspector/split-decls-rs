macro_rules! deps {
    () => {
        Formatter!();
        Seq!();
        Result!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Seq { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Seq") ? ; if let Some (lits) = self . literals () { f . debug_list () . entries (lits . iter ()) . finish () } else { write ! (f , "[∞]") } } }
    };
}

impl_163!();