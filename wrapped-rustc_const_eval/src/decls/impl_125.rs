macro_rules! deps {
    () => {
        MemoryKind!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl fmt :: Display for MemoryKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MemoryKind :: Heap { was_made_global } => { write ! (f , "heap allocation{}" , if * was_made_global { " (made global)" } else { "" }) } } } }
    };
}

impl_125!();