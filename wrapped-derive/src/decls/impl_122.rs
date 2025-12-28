macro_rules! deps {
    () => {
        RemoveLifetime!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl VisitMut for RemoveLifetime { fn visit_lifetime_mut (& mut self , i : & mut Lifetime) { i . ident = Ident :: new ("_" , Span :: call_site ()) ; visit_mut :: visit_lifetime_mut (self , i) ; } }
    };
}

impl_122!();