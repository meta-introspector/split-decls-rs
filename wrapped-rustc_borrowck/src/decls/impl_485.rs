macro_rules! deps {
    () => {
        Locations!();
        NormalizeLocation!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl NormalizeLocation for Location { fn to_locations (self) -> Locations { Locations :: Single (self) } }
    };
}

impl_485!()