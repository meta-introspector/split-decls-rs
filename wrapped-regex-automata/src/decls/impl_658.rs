macro_rules! deps {
    () => {
        TryHalfMatchesIter!();
    };
}

macro_rules! impl_658 {
    () => {
        deps!();
        impl < 'h , F > core :: fmt :: Debug for TryHalfMatchesIter < 'h , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("TryHalfMatchesIter") . field ("it" , & self . it) . field ("finder" , & "<closure>") . finish () } }
    };
}

impl_658!();