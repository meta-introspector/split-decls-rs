macro_rules! deps {
    () => {
        Locations!();
    };
}

macro_rules! NormalizeLocation {
    () => {
        deps!();
        trait NormalizeLocation : fmt :: Debug + Copy { fn to_locations (self) -> Locations ; }
    };
}

NormalizeLocation!();