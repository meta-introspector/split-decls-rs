macro_rules! deps {
    () => {
        VisitorCons!();
        VisitorNil!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl VisitorNil { pub (crate) fn with < V > (self , visitor : V) -> VisitorCons < V , Self > { VisitorCons (visitor , self) } }
    };
}

impl_289!()