macro_rules! deps {
    () => {
        Literal!();
        Buffer!();
    };
}

macro_rules! impl_specific_lit_to_lit {
    () => {
        deps!();
        macro_rules ! impl_specific_lit_to_lit { ($ ty : ty , $ variant : ident) => { impl < B : crate :: Buffer > From <$ ty > for Literal < B > { fn from (src : $ ty) -> Self { Literal ::$ variant (src) } } } ; }
    };
}

impl_specific_lit_to_lit!();