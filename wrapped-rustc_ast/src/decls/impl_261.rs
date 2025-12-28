macro_rules! deps {
    () => {
        AttrVec!();
        Attribute!();
        HasAttrs!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl < T : HasAttrs > HasAttrs for Option < T > { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = T :: SUPPORTS_CUSTOM_INNER_ATTRS ; fn attrs (& self) -> & [Attribute] { self . as_ref () . map (| inner | inner . attrs ()) . unwrap_or (& []) } fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { if let Some (inner) = self . as_mut () { inner . visit_attrs (f) ; } } }
    };
}

impl_261!()