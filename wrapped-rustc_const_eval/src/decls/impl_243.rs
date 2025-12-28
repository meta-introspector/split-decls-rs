macro_rules! deps {
    () => {
        Machine!();
        MemoryKind!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl < T : fmt :: Display > fmt :: Display for MemoryKind < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MemoryKind :: Stack => write ! (f , "stack variable") , MemoryKind :: CallerLocation => write ! (f , "caller location") , MemoryKind :: Machine (m) => write ! (f , "{m}") , } } }
    };
}

impl_243!();