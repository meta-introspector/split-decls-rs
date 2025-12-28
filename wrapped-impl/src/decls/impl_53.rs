macro_rules! deps {
    () => {
        ParamsInScope!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'a > ParamsInScope < 'a > { pub fn new (generics : & 'a Generics) -> Self { ParamsInScope { names : generics . type_params () . map (| param | & param . ident) . collect () , } } pub fn intersects (& self , ty : & Type) -> bool { let mut found = false ; crawl (self , ty , & mut found) ; found } }
    };
}

impl_53!()