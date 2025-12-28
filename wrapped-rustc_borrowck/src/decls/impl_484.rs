macro_rules! deps {
    () => {
        Locations!();
        NormalizeLocation!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl NormalizeLocation for Locations { fn to_locations (self) -> Locations { self } }
    };
}

impl_484!()