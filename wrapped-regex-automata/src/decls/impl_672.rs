macro_rules! deps {
    () => {
        TryCapturesIter!();
    };
}

macro_rules! impl_672 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'h , F > core :: fmt :: Debug for TryCapturesIter < 'h , F > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("TryCapturesIter") . field ("it" , & self . it) . field ("caps" , & self . caps) . field ("finder" , & "<closure>") . finish () } }
    };
}

impl_672!();