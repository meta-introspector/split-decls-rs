macro_rules! deps {
    () => {
        StackBox!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < T > std :: ops :: Deref for StackBox < T > { type Target = T ; fn deref (& self) -> & T { unsafe { self . ptr . as_ref () } } }
    };
}

impl_58!()