macro_rules! deps {
    () => {
        ResolverContext!();
        Context!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl < 'a > Deref for ResolverContext < 'a > { type Target = Context < 'a > ; fn deref (& self) -> & Self :: Target { self . ctx } }
    };
}

impl_432!()