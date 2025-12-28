macro_rules! deps {
    () => {
        UpmappingResult!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < T > UpmappingResult < T > { pub fn call_site (self) -> T { self . call_site } pub fn collect < FI : FromIterator < T > > (self) -> FI { FI :: from_iter (self) } }
    };
}

impl_55!()