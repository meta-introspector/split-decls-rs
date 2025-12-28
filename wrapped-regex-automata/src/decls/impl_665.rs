macro_rules! deps {
    () => {
        TryMatchesIter!();
    };
}

macro_rules! impl_665 {
    () => {
        deps!();
        impl < 'h , F > core :: fmt :: Debug for TryMatchesIter < 'h , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("TryMatchesIter") . field ("it" , & self . it) . field ("finder" , & "<closure>") . finish () } }
    };
}

impl_665!()