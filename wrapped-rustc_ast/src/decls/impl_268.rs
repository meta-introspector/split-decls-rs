macro_rules! deps {
    () => {
        HasAttrs!();
        AttrVec!();
        Attribute!();
        AstNodeWrapper!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < Wrapped : HasAttrs , Tag > HasAttrs for AstNodeWrapper < Wrapped , Tag > { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = Wrapped :: SUPPORTS_CUSTOM_INNER_ATTRS ; fn attrs (& self) -> & [Attribute] { self . wrapped . attrs () } fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { self . wrapped . visit_attrs (f) ; } }
    };
}

impl_268!();