macro_rules! deps {
    () => {
        Vtable!();
        Owned!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < T > Owned < T > { const VTABLE : Vtable = Vtable { clone : owned_clone :: < T > , into_vec : owned_to_vec :: < T > , into_mut : owned_to_mut :: < T > , is_unique : owned_is_unique , drop : owned_drop :: < T > , } ; }
    };
}

impl_122!()