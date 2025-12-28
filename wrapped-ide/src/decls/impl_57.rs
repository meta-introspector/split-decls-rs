macro_rules! deps {
    () => {
        UpmappingResult!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < T > UpmappingResult < T > { pub (crate) fn map < U > (self , f : impl Fn (T) -> U) -> UpmappingResult < U > { UpmappingResult { call_site : f (self . call_site) , def_site : self . def_site . map (f) } } }
    };
}

impl_57!();