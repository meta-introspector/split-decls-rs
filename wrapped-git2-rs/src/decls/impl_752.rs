macro_rules! deps {
    () => {
        Statuses!();
        Binding!();
    };
}

macro_rules! impl_752 {
    () => {
        deps!();
        impl < 'repo > Binding for Statuses < 'repo > { type Raw = * mut raw :: git_status_list ; unsafe fn from_raw (raw : * mut raw :: git_status_list) -> Statuses < 'repo > { Statuses { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_status_list { self . raw } }
    };
}

impl_752!()