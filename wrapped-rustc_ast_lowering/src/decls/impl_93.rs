macro_rules! deps {
    () => {
        UnstableAbi!();
        GateReason!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl fmt :: Display for UnstableAbi { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self { abi , .. } = self ; match self . explain { GateReason :: Experimental => { write ! (f , "the extern {abi} ABI is experimental and subject to change") } GateReason :: ImplDetail => { write ! (f , "the extern {abi} ABI is an implementation detail and perma-unstable") } } } }
    };
}

impl_93!();