macro_rules! deps {
    () => {
        HasAttrs!();
        Attribute!();
        AttrVec!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl < T : HasAttrs > HasAttrs for Box < T > { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = T :: SUPPORTS_CUSTOM_INNER_ATTRS ; fn attrs (& self) -> & [Attribute] { (* * self) . attrs () } fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { (* * self) . visit_attrs (f) ; } }
    };
}

impl_260!()