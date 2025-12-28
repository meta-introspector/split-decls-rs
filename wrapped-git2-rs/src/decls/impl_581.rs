macro_rules! deps {
    () => {
        PushUpdate!();
        Binding!();
    };
}

macro_rules! impl_581 {
    () => {
        deps!();
        impl < 'a > Binding for PushUpdate < 'a > { type Raw = * const raw :: git_push_update ; unsafe fn from_raw (raw : * const raw :: git_push_update) -> PushUpdate < 'a > { PushUpdate { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> Self :: Raw { self . raw } }
    };
}

impl_581!()