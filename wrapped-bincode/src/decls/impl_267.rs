macro_rules! deps {
    () => {
        Limit!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < const N : usize > InternalLimitConfig for Limit < N > { const LIMIT : Option < usize > = Some (N) ; }
    };
}

impl_267!();