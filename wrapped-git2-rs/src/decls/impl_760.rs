macro_rules! deps {
    () => {
        StatusEntry!();
        Binding!();
    };
}

macro_rules! impl_760 {
    () => {
        deps!();
        impl < 'statuses > Binding for StatusEntry < 'statuses > { type Raw = * const raw :: git_status_entry ; unsafe fn from_raw (raw : * const raw :: git_status_entry) -> StatusEntry < 'statuses > { StatusEntry { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_status_entry { self . raw } }
    };
}

impl_760!()