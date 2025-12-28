macro_rules! deps {
    () => {
        SearchKind!();
        Teddy!();
        RabinKarp!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl SearchKind { fn memory_usage (& self) -> usize { match * self { SearchKind :: Teddy (ref ted) => ted . memory_usage () , SearchKind :: RabinKarp => 0 , } } }
    };
}

impl_115!();