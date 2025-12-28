macro_rules! deps {
    () => {
        Lazy!();
    };
}

macro_rules! impl_679 {
    () => {
        deps!();
        impl < T , F : Fn () -> T > core :: ops :: Deref for Lazy < T , F > { type Target = T ; fn deref (& self) -> & T { Lazy :: get (self) } }
    };
}

impl_679!();