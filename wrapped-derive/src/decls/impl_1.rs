macro_rules! deps {
    () => {
        TypeVisitor!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'ast > Visit < 'ast > for TypeVisitor < '_ > { fn visit_lifetime (& mut self , lt : & 'ast Lifetime) { if lt . ident != "static" { self . found_lifetimes = true ; } visit_lifetime (self , lt) } fn visit_type_path (& mut self , ty : & 'ast TypePath) { if let Some (ident) = ty . path . get_ident () { if let Some (maybe_borrowed) = self . typarams . get (ident) { self . found_typarams = true ; if maybe_borrowed . is_some () { self . found_lifetimes = true ; } } } visit_type_path (self , ty) } }
    };
}

impl_1!();