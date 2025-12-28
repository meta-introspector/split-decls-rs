macro_rules! deps {
    () => {
        RawIter!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T > RawIter < T > { unsafe fn drop_elements (& mut self) { if T :: NEEDS_DROP && self . items != 0 { for item in self { item . drop () ; } } } }
    };
}

impl_76!()