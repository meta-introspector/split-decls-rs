macro_rules! deps {
    () => {
        VisitorCons!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < A , B > VisitorCons < A , B > { pub (crate) const fn with < V > (self , visitor : V) -> VisitorCons < V , Self > { VisitorCons (visitor , self) } }
    };
}

impl_291!()