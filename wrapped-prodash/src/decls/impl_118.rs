macro_rules! deps {
    () => {
        Mode!();
        Location!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl Mode { fn percent_location (& self) -> Option < Location > { if self . percent { Some (self . location) } else { None } } fn throughput_location (& self) -> Option < Location > { if self . throughput { Some (self . location) } else { None } } }
    };
}

impl_118!()