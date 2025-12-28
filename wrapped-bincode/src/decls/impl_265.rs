macro_rules! deps {
    () => {
        NoLimit!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl InternalLimitConfig for NoLimit { const LIMIT : Option < usize > = None ; }
    };
}

impl_265!()